//! A collection of generic functions to use in `ISlot.transfer`.
//!
//! All methods, if they edit the item values, try to transfer
//! the items from `items.0` to `items.1`.
use crate::traits;

/// Two item instances to represent slot to slot interaction.
///
/// This is the input almost all slot management functions take.
pub type Items<II> = (Option<II>, Option<II>);
/// `Items<II>` Wrapped in `Result`, so that if the slot management
/// function fails you can choose your fallback option.
///
/// For example, you cannot combine two stacks if they are different items
/// Therefore you need to choose a fallback for what happens next.
/// Personally, swapping them has worked for me in my games.
pub type ItemsRes<'a, II> = Result<Items<II>, (&'a str, Items<II>)>;

/// Returns the inverse of the two inputs, specifically `(items.1, items.0)`.
pub fn swap<'a, II: traits::IItemInstance<'a>>(
    items: (Option<II>, Option<II>),
) -> (Option<II>, Option<II>) {
    (items.1, items.0)
}

/// If the result is Err, the items will be swapped. Otherwise, they will not be swapped.
pub fn swap_if_err<'a, II>(items: ItemsRes<II>) -> Items<II>
where
    II: traits::IItemInstance<'a>,
{
    match items {
        Ok(inner) => inner,
        Err(inner) => swap(inner.1),
    }
}

/// Does nothing to the items, just extracts them from Result.
pub fn unwrap_items_res<'a, II>(items: ItemsRes<II>) -> Items<II>
where
    II: traits::IItemInstance<'a>,
{
    match items {
        Ok(inner) => inner,
        Err(inner) => inner.1,
    }
}
/// Combines two stacks of items. Tries to put `items.0` into `items.1`.
///
/// The defined behavior for this can be seen here:
/// ```
/// use game_inventory::sample_structs::{ItemInstance};
/// use game_inventory::traits::{IItem, IItemInstance};
/// use game_inventory::slot_management::combine_stack;
/// use game_inventory::sample_items::{TORCH, TORCH_INST};
///
/// // Combine two stacks of the same item.
/// let items = (
///     Some(ItemInstance {
///         item: &TORCH,
///         quantity: 90,
///     }),
///     TORCH_INST,
/// );
/// let res = combine_stack(items).ok().unwrap();
/// // Accounts for the max quantity of the torch, and does not overfill.
/// assert_eq!(res.0.unwrap().item().name(),TORCH.name());
/// assert_eq!(res.0.unwrap().quant(),13);
/// assert_eq!(res.1.unwrap().item().name(),TORCH.name());
/// assert_eq!(res.1.unwrap().quant(),100);
/// ```
/// You will not be able to combine the item stacks if:
/// ```
/// use game_inventory::sample_structs::ItemInstance;
/// use game_inventory::slot_management::combine_stack;
/// use game_inventory::sample_items::{TORCH_INST,SWORD_INST,JUNK_INST, TORCH_FULL_STACK_INST};
/// // Either item is None.
/// assert!(combine_stack((None, None,)).is_err());
/// assert!(combine_stack((TORCH_INST, None,)).is_err());
/// assert!(combine_stack((None, TORCH_INST,)).is_err());
/// assert!(combine_stack((None, SWORD_INST)).is_err());
/// assert!(combine_stack((SWORD_INST, None)).is_err());
/// // The items are not the same.
/// assert!(combine_stack((TORCH_INST, JUNK_INST)).is_err());
/// assert!(combine_stack((JUNK_INST, TORCH_INST)).is_err());
/// // One of the items has a full stack.
/// assert!(combine_stack((TORCH_FULL_STACK_INST, TORCH_INST)).is_err());
/// assert!(combine_stack((TORCH_INST, TORCH_FULL_STACK_INST)).is_err());
/// ```
pub fn combine_stack<'a, II>(items: Items<II>) -> ItemsRes<'a, II>
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    let (c, o) = match items {
        (Some(c), Some(o)) => (c, o),
        _ => {
            return Err((
                "Both items need to be Some for this operation to work.",
                items,
            ))
        }
    };
    if !c.item().stackable() {
        return Err(("Cannot combine unstackable items.", items));
    }
    if c.item().name() != o.item().name() {
        return Err((
            "Both items must be the same for this operation to work.",
            items,
        ));
    }
    let stack_size = c.item().max_quant();
    if c.quant() >= stack_size || o.quant() >= stack_size {
        return Err((
            "Cannot combine stacks when the stack amount is reached on an item.",
            items,
        ));
    }
    let combined = c.quant() + o.quant();
    if combined < stack_size {
        return Ok((None, Some(II::new(c.item(), combined))));
    }
    let left_over = combined - stack_size;
    return Ok((
        Some(II::new(c.item(), left_over)),
        Some(II::new(c.item(), stack_size)),
    ));
}

/// Splits a stack of items into two. Tries to split `items.0` and put the second half into `items.1`
pub fn half_stack_split<'a, II>(items: Items<II>) -> ItemsRes<'a, II>
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    let c = match items.0 {
        Some(c) => c,
        None => return Err(("items.0 must be Some for this operation to work.", items)),
    };
    if !c.item().stackable() {
        return Err((
            "items.0 must be stackable for this operation to work",
            items,
        ));
    }
    if match items.1 {
        Some(o) => c.item().name() != o.item().name(),
        None => false,
    } {
        return Err((
            "Both items must be the same for this operation to work.",
            items,
        ));
    }
    if c.quant() < 2 {
        return Err((
            "items.0 has 1 item in its stack. This cannot be split in two.",
            items,
        ));
    }
    let other_quant = match items.1 {
        Some(o) => o.quant(),
        None => 0,
    };
    let half_stack = c.quant() / 2;
    return Ok((
        Some(II::new(c.item(), half_stack)),
        Some(II::new(
            c.item(),
            other_quant + half_stack + (c.quant() % 2),
        )),
    ));
}

/// Removes a single item from a stack. Tries to take a single item from `items.0` and put it into `items.1`.
pub fn remove_from_stack<'a, II>(items: Items<II>) -> ItemsRes<'a, II>
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    let c = match items.0 {
        Some(c) => c,
        None => return Err(("items.0 must be Some for this operation to work.", items)),
    };
    if !c.item().stackable() {
        return Err((
            "items.0 must be stackable for this operation to work",
            items,
        ));
    }
    let o = match items.1 {
        Some(o) => o,
        None => {
            if c.quant() < 2 {
                return Ok((None, Some(II::new(c.item(), 1))));
            }
            return Ok((
                Some(II::new(c.item(), c.quant() - 1)),
                Some(II::new(c.item(), 1)),
            ));
        }
    };
    if o.item().name() != c.item().name() {
        return Err((
            "Both items must be the same for this operation to work.",
            items,
        ));
    }
    if o.quant() >= o.item().max_quant() {
        return Err(("Cannot add to other as it is the max quantity", items));
    }
    if c.quant() < 2 {
        return Ok((None, Some(II::new(o.item(), o.quant() + 1))));
    }
    return Ok((
        Some(II::new(c.item(), c.quant() - 1)),
        Some(II::new(o.item(), o.quant() + 1)),
    ));
}

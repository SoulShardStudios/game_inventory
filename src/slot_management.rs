//! A collection of generic functions to use in `Slot.transfer`.
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
/// The `&'a str` is for the error message.
///
/// For example, you cannot combine two stacks if they are different items
/// Therefore you need to choose a fallback for what happens next.
/// Personally, swapping them has worked for me in my games.
pub type ItemsRes<'a, II> = Result<Items<II>, (&'a str, Items<II>)>;

/// Returns the inverse of the two inputs, specifically `(items.1, items.0)`.
pub fn swap<'a, II: traits::ItemInstance<'a>>(
    items: (Option<II>, Option<II>),
) -> (Option<II>, Option<II>) {
    (items.1, items.0)
}

/// If the result is Err, the items will be swapped. Otherwise, they will not be swapped.
///
/// ```
/// # use game_inventory::samples::TORCH_INST;
/// # use game_inventory::helpers::{combine_stack, swap_if_err};
/// # use game_inventory::traits::{Item, ItemInstance};
/// let items = (TORCH_INST, None);
/// let unwrapped = swap_if_err(combine_stack(items));
/// assert_eq!(items.1.is_none(), unwrapped.0.is_none());
/// assert_eq!(items.0.unwrap().item().name(), unwrapped.1.unwrap().item().name());
/// assert_eq!(items.0.unwrap().quant(), unwrapped.1.unwrap().quant());
/// ```
pub fn swap_if_err<'a, II>(items: ItemsRes<II>) -> Items<II>
where
    II: traits::ItemInstance<'a>,
{
    match items {
        Ok(inner) => inner,
        Err(inner) => swap(inner.1),
    }
}

/// Does nothing to the items, just extracts them from Result.
///
/// ```
/// # use game_inventory::samples::TORCH_INST;
/// # use game_inventory::helpers::{combine_stack, unwrap_items_res};
/// # use game_inventory::traits::{Item, ItemInstance};
/// let items = (TORCH_INST, None);
/// let unwrapped = unwrap_items_res(combine_stack(items));
/// assert_eq!(items.0.unwrap().item().name(), unwrapped.0.unwrap().item().name());
/// assert_eq!(items.0.unwrap().quant(), unwrapped.0.unwrap().quant());
/// assert_eq!(items.1.is_none(), unwrapped.1.is_none());
/// ```
pub fn unwrap_items_res<'a, II>(items: ItemsRes<II>) -> Items<II>
where
    II: traits::ItemInstance<'a>,
{
    match items {
        Ok(inner) => inner,
        Err(inner) => inner.1,
    }
}
/// Combines two stacks of items. Tries to put `items.0` into `items.1`.
///
/// ```
/// # use game_inventory::samples::{DefaultItemInstance, TORCH, TORCH_INST};
/// # use game_inventory::traits::{Item, ItemInstance};
/// # use game_inventory::helpers::combine_stack;
/// let items = (
///     Some(DefaultItemInstance {
///         item: &TORCH,
///         quantity: 90,
///     }),
///     TORCH_INST,
/// );
/// let res = combine_stack(items).ok().unwrap();
/// assert_eq!(res.0.unwrap().item().name(),TORCH.name());
/// assert_eq!(res.0.unwrap().quant(),13);
/// assert_eq!(res.1.unwrap().item().name(),TORCH.name());
/// assert_eq!(res.1.unwrap().quant(),100);
/// ```
/// You will not be able to combine the item stacks if:
/// ```
/// # use game_inventory::samples::{DefaultItemInstance, TORCH_INST, SWORD_INST, JUNK_INST, TORCH_FULL_STACK_INST};
/// # use game_inventory::slot_management::combine_stack;
/// // Either item is None.
/// assert!(combine_stack::<DefaultItemInstance>((None, None,)).is_err());
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
    II: traits::ItemInstance<'a> + Copy + 'a,
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
///
/// ```
/// # use game_inventory::samples::{TORCH, DefaultItemInstance};
/// # use game_inventory::helpers::half_stack_split;
/// # use game_inventory::traits::{Item, ItemInstance};
/// let res = half_stack_split((
/// Some(DefaultItemInstance {
///     item: &TORCH,
///     quantity: 11,
/// }),
/// Some(DefaultItemInstance {
///     item: &TORCH,
///     quantity: 3,
/// }),
/// )).ok().unwrap();
/// assert!(res.0.unwrap().item().name() == TORCH.name());
/// assert!(res.0.unwrap().quant() == 5);
/// assert!(res.1.unwrap().item().name() == TORCH.name());
/// assert!(res.1.unwrap().quant() == 9);
/// ```
/// You will not be able to split the stack in half if:
/// ```
/// # use game_inventory::samples::{TORCH_INST, SWORD_INST, JUNK_INST};
/// # use game_inventory::helpers::half_stack_split;
/// // items.0 is None
/// assert!(half_stack_split((None,TORCH_INST)).is_err());
/// assert!(half_stack_split((None, SWORD_INST)).is_err());
/// // items.0 is unstackable
/// assert!(half_stack_split((SWORD_INST, None)).is_err());
/// // The items are not the same.
/// assert!(half_stack_split((TORCH_INST, JUNK_INST)).is_err());
/// assert!(half_stack_split((JUNK_INST, TORCH_INST)).is_err());
/// ```
pub fn half_stack_split<'a, II>(items: Items<II>) -> ItemsRes<'a, II>
where
    II: traits::ItemInstance<'a> + Copy + 'a,
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
///
/// ```
/// # use game_inventory::samples::{DefaultItemInstance, TORCH_INST, TORCH};
/// # use game_inventory::helpers::remove_from_stack;
/// # use game_inventory::traits::{ItemInstance, Item};
/// let res = remove_from_stack((
///     Some(DefaultItemInstance {
///         item: &TORCH,
///         quantity: 3,
///     }),
///     TORCH_INST,
/// )).ok().unwrap();
/// assert!(res.0.unwrap().item().name() == TORCH.name());
/// assert!(res.0.unwrap().quant() == 2);
/// assert!(res.1.unwrap().item().name() == TORCH.name());
/// assert!(res.1.unwrap().quant() == 24);
/// ```
/// Also accounts for the edge case of `items.0` having a quantity of `1`:
/// ```
/// # use game_inventory::samples::{DefaultItemInstance, TORCH_INST, TORCH};
/// # use game_inventory::helpers::remove_from_stack;
/// # use game_inventory::traits::{ItemInstance, Item};
/// let res = remove_from_stack((
///     Some(DefaultItemInstance {
///         item: &TORCH,
///         quantity: 1,
///     }),
///     Some(DefaultItemInstance {
///         item: &TORCH,
///         quantity: 20,
///     }),
/// )).ok().unwrap();
/// assert!(res.1.unwrap().item().name() == TORCH.name());
/// assert!(res.1.unwrap().quant() == 21);
/// assert!(res.0.is_none());
/// ```
/// And accounts for `items.1` being `None`
/// ```
/// # use game_inventory::samples::{DefaultItemInstance, TORCH_INST, TORCH};
/// # use game_inventory::helpers::remove_from_stack;
/// # use game_inventory::traits::{ItemInstance, Item};
/// let res = remove_from_stack((
///     Some(DefaultItemInstance {
///         item: &TORCH,
///         quantity: 3,
///     }),
///     None,
/// )).ok().unwrap();
/// assert!(res.0.unwrap().item().name() == TORCH.name());
/// assert!(res.0.unwrap().quant() == 2);
/// assert!(res.1.unwrap().item().name() == TORCH.name());
/// assert!(res.1.unwrap().quant() == 1);
/// ```
/// You will not be able to remove one from the stack if:
/// ```
/// # use game_inventory::samples::{DefaultItemInstance, TORCH, TORCH_INST, TORCH_FULL_STACK_INST, SWORD_INST, JUNK_INST,};
/// # use game_inventory::helpers::remove_from_stack;
/// // items.0 is None.
/// assert!(remove_from_stack((None, TORCH_FULL_STACK_INST)).is_err());
/// // Any of the items are unstackable.
/// assert!(remove_from_stack((None, SWORD_INST)).is_err());
/// assert!(remove_from_stack((SWORD_INST, None)).is_err());
/// // The items are not the same.
/// assert!(remove_from_stack((TORCH_INST, JUNK_INST)).is_err());
/// assert!(remove_from_stack((JUNK_INST, TORCH_INST)).is_err());
/// ```
pub fn remove_from_stack<'a, II>(items: Items<II>) -> ItemsRes<'a, II>
where
    II: traits::ItemInstance<'a> + Copy + 'a,
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

//! A collection of generic functions to use in `Slot.transfer`.
//!
//! All methods, if they edit the item values, try to transfer
//! the items from `items.0` to `items.1`.
use crate::traits::{Item, ItemInstance};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct SlotErr(String);

impl Display for SlotErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for SlotErr {}

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
pub type ItemsRes<II> = Result<Items<II>, (Box<dyn std::error::Error>, Items<II>)>;

/// Returns the inverse of the two inputs, specifically `(items.1, items.0)`.
pub fn swap<'a, Id: Eq, I: Item<Id = Id>, II: ItemInstance<'a, I>>(
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
/// let unwrapped = swap_if_err(combine_stack(items.clone()));
/// assert_eq!(items.1.is_none(), unwrapped.0.is_none());
/// assert_eq!(items.0.as_ref().unwrap().item().id(), unwrapped.1.as_ref().unwrap().item().id());
/// assert_eq!(items.0.as_ref().unwrap().quant(), unwrapped.1.unwrap().quant());
/// ```
pub fn swap_if_err<'a, Id: Eq, I: Item<Id = Id>, II: ItemInstance<'a, I>>(
    items: ItemsRes<II>,
) -> Items<II> {
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
/// let unwrapped = unwrap_items_res(combine_stack(items.clone()));
/// assert_eq!(items.0.as_ref().unwrap().item().id(), unwrapped.0.as_ref().unwrap().item().id());
/// assert_eq!(items.0.unwrap().quant(), unwrapped.0.unwrap().quant());
/// assert_eq!(items.1.is_none(), unwrapped.1.is_none());
/// ```
pub fn unwrap_items_res<'a, Id: Eq, I: Item<Id = Id>, II: ItemInstance<'a, I>>(
    items: ItemsRes<II>,
) -> Items<II> {
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
/// assert_eq!(res.0.clone().unwrap().item().id(),TORCH.id());
/// assert_eq!(res.0.unwrap().quant(),13);
/// assert_eq!(res.1.clone().unwrap().item().id(),TORCH.id());
/// assert_eq!(res.1.unwrap().quant(),100);
/// ```
/// You will not be able to combine the item stacks if:
/// ```
/// # use game_inventory::samples::{DefaultItemInstance, DefaultItem, TORCH_INST, SWORD_INST, JUNK_INST, TORCH_FULL_STACK_INST};
/// # use game_inventory::slot_management::combine_stack;
/// // Either item is None.
/// assert!(combine_stack::<'static, &'static str, DefaultItem<'static>, DefaultItemInstance<'static, DefaultItem<'static>>>((None, None,)).is_err());
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
pub fn combine_stack<'a, Id: Eq, I: Item<Id = Id> + 'a, II: ItemInstance<'a, I>>(
    items: Items<II>,
) -> ItemsRes<II> {
    let (c, o) = match &items {
        (Some(c), Some(o)) => (c, o),
        _ => {
            return Err((
                Box::new(SlotErr(
                    "Both items need to be Some for this operation to work.".to_owned(),
                )),
                items,
            ))
        }
    };
    if !c.item().stackable() {
        return Err((
            Box::new(SlotErr("Cannot combine unstackable items.".to_owned())),
            items,
        ));
    }
    if c.item().id() != o.item().id() {
        return Err((
            Box::new(SlotErr(
                "Both items must be the same for this operation to work.".to_owned(),
            )),
            items,
        ));
    }
    let stack_size = c.item().max_quant();
    if c.quant() >= stack_size || o.quant() >= stack_size {
        return Err((
            Box::new(SlotErr(
                "Cannot combine stacks when the stack amount is reached on an item.".to_owned(),
            )),
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
/// assert!(res.0.as_ref().unwrap().item().id() == TORCH.id());
/// assert!(res.0.unwrap().quant() == 5);
/// assert!(res.1.as_ref().unwrap().item().id() == TORCH.id());
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
pub fn half_stack_split<'a, Id: Eq, I: Item<Id = Id> + 'a, II: ItemInstance<'a, I>>(
    items: Items<II>,
) -> ItemsRes<II> {
    let c = match &items.0 {
        Some(c) => c,
        None => {
            return Err((
                Box::new(SlotErr(
                    "items.0 must be Some for this operation to work.".to_owned(),
                )),
                items,
            ))
        }
    };
    if !c.item().stackable() {
        return Err((
            Box::new(SlotErr(
                "items.0 must be stackable for this operation to work".to_owned(),
            )),
            items,
        ));
    }
    if match &items.1 {
        Some(o) => c.item().id() != o.item().id(),
        None => false,
    } {
        return Err((
            Box::new(SlotErr(
                "Both items must be the same for this operation to work.".to_owned(),
            )),
            items,
        ));
    }
    if c.quant() < 2 {
        return Err((
            Box::new(SlotErr(
                "items.0 has 1 item in its stack. This cannot be split in two.".to_owned(),
            )),
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
/// assert!(res.0.as_ref().unwrap().item().id() == TORCH.id());
/// assert!(res.0.as_ref().unwrap().quant() == 2);
/// assert!(res.1.as_ref().unwrap().item().id() == TORCH.id());
/// assert!(res.1.as_ref().unwrap().quant() == 24);
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
/// assert!(res.1.clone().unwrap().item().id() == TORCH.id());
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
/// assert!(res.0.as_ref().unwrap().item().id() == TORCH.id());
/// assert!(res.0.unwrap().quant() == 2);
/// assert!(res.1.as_ref().unwrap().item().id() == TORCH.id());
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
pub fn remove_from_stack<'a, Id: Eq, I: Item<Id = Id> + 'a, II: ItemInstance<'a, I>>(
    items: Items<II>,
) -> ItemsRes<II> {
    let c = match &items.0 {
        Some(c) => c,
        None => {
            return Err((
                Box::new(SlotErr(
                    "items.0 must be Some for this operation to work.".to_owned(),
                )),
                items,
            ))
        }
    };
    if !c.item().stackable() {
        return Err((
            Box::new(SlotErr(
                "items.0 must be stackable for this operation to work".to_owned(),
            )),
            items,
        ));
    }
    let o = match &items.1 {
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
    if o.item().id() != c.item().id() {
        return Err((
            Box::new(SlotErr(
                "Both items must be the same for this operation to work.".to_owned(),
            )),
            items,
        ));
    }
    if o.quant() >= o.item().max_quant() {
        return Err((
            Box::new(SlotErr(
                "Cannot add to other as it is the max quantity".to_owned(),
            )),
            items,
        ));
    }
    if c.quant() < 2 {
        return Ok((None, Some(II::new(o.item(), o.quant() + 1))));
    }
    return Ok((
        Some(II::new(c.item(), c.quant() - 1)),
        Some(II::new(o.item(), o.quant() + 1)),
    ));
}

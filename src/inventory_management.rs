//! A collection of generic functions that operate on a `Vec<ISlot>` (A collection of slots, AKA an inventory).
use crate::slot_management::combine_stack;
use crate::traits::{IItemInstance, ISlot};

/// Checks if a `Vec<ISlot>` contains an item with a matching name and quantity.
pub fn inventory_contains_item<'a, II, S>(inventory: &Vec<S>, other: Option<II>) -> bool
where
    II: IItemInstance<'a> + Copy,
    S: ISlot<'a, II>,
{
    match other {
        Some(o) => inventory.iter().any(|s| match s.item_instance() {
            Some(i) => i.item().name() == o.item().name() && i.quant() == o.quant(),
            None => false,
        }),
        None => false,
    }
}

/// Checks if a `Vec<ISlot>` contains an item with a matching name.
pub fn inventory_contains_item_type<'a, II, S>(inventory: &Vec<S>, name: &str) -> bool
where
    II: IItemInstance<'a> + Copy,
    S: ISlot<'a, II>,
{
    inventory.iter().any(|s| match s.item_instance() {
        Some(i) => i.item().name() == name,
        None => false,
    })
}

/// Attempts to add an item to the given inventory.
/// Returns the leftover item if the inventory is full.
///
/// Accounts for unstackable items, item overflowing, and many other edge cases.
/// See the tests in `tests/inventory_management.rs` for specific behavior.
pub fn add_to_inventory<'a, II, S>(inventory: &mut Vec<S>, other: Option<II>) -> Option<II>
where
    II: IItemInstance<'a> + Copy + 'a,
    S: ISlot<'a, II>,
{
    if inventory.capacity() == 0 {
        return other;
    }
    match other {
        Some(o) => {
            if o.item().stackable() {
                if o.item().max_quant() == o.quant() {
                    return Some(o);
                }
                inventory
                    .iter_mut()
                    .fold(Some(o), |current, slot| match current {
                        Some(c) => match slot.item_instance() {
                            Some(s) => {
                                if s.item().name() != c.item().name() {
                                    return current;
                                }
                                if s.quant() == s.item().max_quant() {
                                    return current;
                                }
                                let res = combine_stack((slot.item_instance(), Some(c)));
                                slot.set_item_instance(&res.1);
                                return res.0;
                            }
                            None => None,
                        },
                        None => None,
                    });
            }
            match inventory
                .iter_mut()
                .find(|slot| slot.item_instance().is_none())
            {
                Some(s) => {
                    s.set_item_instance(&Some(o));
                    return None;
                }
                None => Some(o),
            }
        }
        None => None,
    }
}

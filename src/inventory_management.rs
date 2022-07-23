//! A collection of generic functions that operate on a `Vec<ISlot>` (A collection of slots, AKA an inventory).
use crate::slot_management::combine_stack;
use crate::traits::{IItemInstance, ISlot};

/// Checks if a `Vec<ISlot>` contains an item with a matching name and quantity.
pub fn inventory_contains_item<'a, II, S>(inventory: &Vec<S>, other: II) -> bool
where
    II: IItemInstance<'a> + Copy,
    S: ISlot<'a, II>,
{
    inventory.iter().any(|s| match s.item_instance() {
        Some(i) => i.item().name() == other.item().name() && i.quant() == other.quant(),
        None => false,
    })
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

/// Gets the total quantity of all items with a matching name
pub fn quant_in_inventory<'a, II, S>(inventory: &Vec<S>, name: &str) -> u16
where
    II: IItemInstance<'a> + Copy,
    S: ISlot<'a, II>,
{
    inventory
        .iter()
        .fold(0, |quant, slot| match slot.item_instance() {
            Some(ii) => {
                if ii.item().name() == name {
                    if ii.item().stackable() {
                        return ii.quant() + quant;
                    }
                    return quant + 1;
                }
                quant
            }
            None => quant,
        })
}

/// Gets the quantity of empty slots in an inventory
pub fn empty_quant_in_inventory<'a, II, S>(inventory: &Vec<S>) -> usize
where
    II: IItemInstance<'a> + Copy,
    S: ISlot<'a, II>,
{
    inventory
        .iter()
        .filter(|slot| slot.item_instance().is_none())
        .count()
}

/// Attempts to add a stackable item to the given inventory
fn add_stackable_to_inventory<'a, II, S>(inventory: &mut Vec<S>, other: II) -> Option<II>
where
    II: IItemInstance<'a> + Copy + 'a,
    S: ISlot<'a, II>,
{
    fn try_add_to_slot<'a, II, S>(other: Option<II>, slot: &mut S) -> Option<II>
    where
        II: IItemInstance<'a> + Copy + 'a,
        S: ISlot<'a, II>,
    {
        let c = match other {
            None => return None,
            Some(c) => c,
        };

        let s = match slot.item_instance() {
            None => {
                slot.set_item_instance(&other);
                return None;
            }
            Some(s) => s,
        };

        if s.item().name() != c.item().name() {
            return other;
        }
        if s.quant() == s.item().max_quant() {
            return other;
        }
        let res = combine_stack((slot.item_instance(), Some(c)));
        slot.set_item_instance(&res.1);
        return res.0;
    }
    return inventory.iter_mut().fold(Some(other), try_add_to_slot);
}

/// Attempts to add an unstackable item to the given inventory.
fn add_unstackable_to_inventory<'a, II, S>(inventory: &mut Vec<S>, other: II) -> Option<II>
where
    II: IItemInstance<'a> + Copy + 'a,
    S: ISlot<'a, II>,
{
    return match inventory
        .iter_mut()
        .find(|slot| slot.item_instance().is_none())
    {
        None => Some(other),
        Some(s) => {
            s.set_item_instance(&Some(other));
            return None;
        }
    };
}

/// Attempts to add an item to the given inventory.
/// Returns the leftover item if the inventory is full.
///
/// Accounts for unstackable items, item overflowing, and many other edge cases.
/// See the tests in `tests/inventory_management.rs` for specific behavior.
pub fn add_to_inventory<'a, II, S>(inventory: &mut Vec<S>, other: II) -> Option<II>
where
    II: IItemInstance<'a> + Copy + 'a,
    S: ISlot<'a, II>,
{
    if inventory.capacity() == 0 {
        return None;
    }

    if !other.item().stackable() {
        return add_unstackable_to_inventory(inventory, other);
    }

    if other.item().max_quant() == other.quant() {
        return add_unstackable_to_inventory(inventory, other);
    }

    return add_stackable_to_inventory(inventory, other);
}

/// Attempts to remove an item from the given inventory.
///
/// If you are trying to remove an item from a specific slot,
/// index the `Vec<ISlot>`. This is only for bulk removal of items.
///
/// The `quant()` of `item` will be used to determine how many items to remove from the given inventory.
///
/// Returns the removed items.
pub fn remove_from_inventory<'a, II, S>(inventory: &mut Vec<S>, other: II) -> Option<II>
where
    II: IItemInstance<'a> + Copy + 'a,
    S: ISlot<'a, II>,
{
    fn try_remove<'a, II, S>(current: u16, slot: &mut S, other: &II) -> u16
    where
        II: IItemInstance<'a> + Copy + 'a,
        S: ISlot<'a, II>,
    {
        let s = match slot.item_instance() {
            None => return current,
            Some(s) => s,
        };
        if current == 0 {
            return 0;
        }
        if s.item().name() != other.item().name() {
            return current;
        }
        if !s.item().stackable() {
            slot.set_item_instance(&None);
            return current - 1;
        }
        if s.quant() <= current {
            slot.set_item_instance(&None);
            return current - s.quant();
        }
        slot.set_item_instance(&Some(II::new(s.item(), s.quant() - current)));
        0
    }

    let remaining = inventory.iter_mut().fold(other.quant(), |current, slot| {
        try_remove(current, slot, &other)
    });
    if remaining == 0 {
        return None;
    }
    return Some(II::new(other.item(), remaining));
}

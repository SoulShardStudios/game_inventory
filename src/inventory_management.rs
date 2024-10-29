//! A collection of generic functions that operate on a `Vec<Slot>` (A collection of slots, AKA an inventory).
use crate::slot_management::{combine_stack, unwrap_items_res};
use crate::traits::{Item, ItemInstance, Slot};

/// Checks if a `Vec<Slot>` contains an item with a matching name and quantity.
///
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH, SWORD, JUNK, TORCH_FULL_STACK_INST, SWORD_INST};
/// # use game_inventory::traits::{Slot, Item};
/// # use game_inventory::helpers::inventory_contains_item_type;
/// let inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// assert!(inventory_contains_item_type(&inventory, TORCH.id()));
/// assert!(inventory_contains_item_type(&inventory, SWORD.id()));
/// assert!(!inventory_contains_item_type(&inventory, JUNK.id()));
/// ```
pub fn inventory_contains_item<Id: Eq, I: Item<Id = Id>, II: ItemInstance<I>, S: Slot<I, II>>(
    inventory: &Vec<S>,
    other: II,
) -> bool {
    inventory.iter().any(|s| match s.item_instance() {
        Some(i) => i.item().id() == other.item().id() && i.quant() == other.quant(),
        None => false,
    })
}

/// Checks if a `Vec<Slot>` contains an item with a matching name.
///
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH_FULL_STACK_INST, SWORD_INST, TORCH_INST};
/// # use game_inventory::traits::{Slot, Item};
/// # use game_inventory::helpers::inventory_contains_item;
/// let inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// assert!(inventory_contains_item(
///     &inventory,
///     TORCH_FULL_STACK_INST.clone().unwrap()
/// ));
/// assert!(inventory_contains_item(&inventory, SWORD_INST.clone().unwrap()));
/// assert!(!inventory_contains_item(&inventory, TORCH_INST.clone().unwrap()));
/// ```
pub fn inventory_contains_item_type<
    Id: Eq,
    I: Item<Id = Id>,
    II: ItemInstance<I>,
    S: Slot<I, II>,
>(
    inventory: &Vec<S>,
    id: Id,
) -> bool {
    inventory.iter().any(|s| match s.item_instance() {
        Some(i) => i.item().id() == id,
        None => false,
    })
}

/// Gets the total quantity of all items with a matching name.
///
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH_FULL_STACK_INST, SWORD_INST, TORCH_INST, TORCH};
/// # use game_inventory::traits::{Slot, Item};
/// # use game_inventory::helpers::quant_in_inventory;
/// let inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(TORCH_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// assert_eq!(quant_in_inventory(&inventory, TORCH.id()), 123)
/// ```
/// If the item is unstackable, even if the item's amount is greater than one somehow,
/// This method only counts it as one item.
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, SWORD};
/// # use game_inventory::traits::{Slot, Item, ItemInstance};
/// # use game_inventory::helpers::quant_in_inventory;
/// # use std::sync::Arc;
/// let inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(Some(DefaultItemInstance::new(Arc::new(SWORD.clone()), 123))),
/// ];
/// assert_eq!(quant_in_inventory(&inventory, SWORD.id()), 2)
/// ```
pub fn quant_in_inventory<Id: Eq, I: Item<Id = Id>, II: ItemInstance<I>, S: Slot<I, II>>(
    inventory: &Vec<S>,
    id: Id,
) -> u16 {
    inventory
        .iter()
        .fold(0, |quant, slot| match slot.item_instance() {
            Some(ii) => {
                if ii.item().id() == id {
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

/// Gets the quantity of empty slots in an inventory.
///
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH_FULL_STACK_INST, SWORD_INST, TORCH_INST};
/// # use game_inventory::traits::Slot;
/// # use game_inventory::helpers::empty_quant_in_inventory;
/// let inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(None),
///     DefaultSlot::new(TORCH_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// assert_eq!(empty_quant_in_inventory(&inventory), 2)
/// ```
pub fn empty_quant_in_inventory<Id: Eq, I: Item<Id = Id>, II: ItemInstance<I>, S: Slot<I, II>>(
    inventory: &Vec<S>,
) -> usize {
    inventory
        .iter()
        .filter(|slot| slot.item_instance().is_none())
        .count()
}

/// Attempts to add an item to the given inventory.
///
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH_FULL_STACK_INST, SWORD_INST, TORCH_INST, TORCH, SWORD};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::add_to_inventory;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(TORCH_INST.clone()),
/// ];
/// add_to_inventory(&mut inventory, TORCH_INST.clone().unwrap());
/// assert!(inventory[0].item_instance().unwrap().item().id() == TORCH.id());
/// assert!(inventory[0].item_instance().unwrap().quant() == TORCH.max_quant());
/// assert!(inventory[1].item_instance().unwrap().item().id() == SWORD.id());
/// assert!(inventory[2].item_instance().unwrap().item().id() == TORCH.id());
/// assert!(inventory[2].item_instance().unwrap().quant() == TORCH_INST.as_ref().unwrap().quant() * 2);
/// ```
/// Does not add the item to the given inventory if its full.
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH_FULL_STACK_INST, SWORD_INST, TORCH_INST, JUNK_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::add_to_inventory;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
/// ];
///
/// let instances_to_test = vec![TORCH_INST.clone(), TORCH_FULL_STACK_INST.clone(), JUNK_INST.clone(), SWORD_INST.clone()];
/// instances_to_test.iter().for_each(|inst| {
/// let res = add_to_inventory(&mut inventory, inst.clone().unwrap());
/// assert!(res.as_ref().unwrap().item().id() == inst.as_ref().unwrap().item().id());
/// assert!(res.unwrap().quant() == inst.as_ref().unwrap().quant());
/// });
/// ```
/// Also works for unstackable items.
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH_FULL_STACK_INST, SWORD_INST, TORCH, SWORD};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::add_to_inventory;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// add_to_inventory(&mut inventory, SWORD_INST.clone().unwrap());
/// assert!(inventory[0].item_instance().unwrap().item().id() == TORCH.id());
/// assert!(inventory[0].item_instance().unwrap().quant() == TORCH.max_quant());
/// assert!(inventory[1].item_instance().unwrap().item().id() == SWORD.id());
/// assert!(inventory[2].item_instance().unwrap().item().id() == SWORD.id());
/// ```
pub fn add_to_inventory<Id: Eq, I: Item<Id = Id>, II: ItemInstance<I> + Clone, S: Slot<I, II>>(
    inventory: &mut Vec<S>,
    other: II,
) -> Option<II> {
    if inventory.capacity() == 0 {
        return None;
    }

    return inventory
        .iter_mut()
        .fold(Some(other), add_to_inventory_try_add_to_slot);
}

fn add_to_inventory_try_add_to_slot<
    Id: Eq,
    I: Item<Id = Id>,
    II: ItemInstance<I> + Clone,
    S: Slot<I, II>,
>(
    other: Option<II>,
    slot: &mut S,
) -> Option<II> {
    let c = match &other {
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

    if s.item().id() != c.item().id() {
        return other;
    }
    if s.quant() == s.item().max_quant() {
        return other;
    }
    let res = unwrap_items_res(combine_stack((slot.item_instance(), Some(c.clone()))));
    slot.set_item_instance(&res.1);
    return res.0;
}

/// Attempts to remove an item from the given inventory.
///
/// If you are trying to remove an item from a specific slot,
/// index the `Vec<Slot>`. This is only for bulk removal of items.
///
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, TORCH, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::{quant_in_inventory, remove_from_inventory};
/// # use std::sync::Arc;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(TORCH_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// assert!(remove_from_inventory(&mut inventory, DefaultItemInstance::new(Arc::new(TORCH.clone()), 123)).is_none());
/// assert_eq!(quant_in_inventory(&inventory, TORCH.id()), 0)
/// ```
/// Does not use unstackable items `.quant()` method, treats every unstackable item as one removal.
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, SWORD, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::{quant_in_inventory, remove_from_inventory};
/// # use std::sync::Arc;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(TORCH_INST.clone()),
///     DefaultSlot::new(Some(DefaultItemInstance::new(Arc::new(SWORD.clone()), 123))),
/// ];
/// assert!(remove_from_inventory(&mut inventory, DefaultItemInstance::new(Arc::new(SWORD.clone()), 2)).is_none());
/// assert_eq!(quant_in_inventory(&inventory, SWORD.id()), 0);
/// ```
/// If the inventory does not have that may items to remove,
/// this function will return the quantity it was unable to remove.
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, TORCH, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::{quant_in_inventory, remove_from_inventory};
/// # use std::sync::Arc;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(None),
///     DefaultSlot::new(None),
/// ];
/// let res = remove_from_inventory(&mut inventory, DefaultItemInstance::new(Arc::new(TORCH.clone()), 123)).unwrap();
/// assert_eq!(res.item().id(), TORCH.id());
/// assert_eq!(res.quant(), 23);
/// assert_eq!(quant_in_inventory(&inventory, TORCH.id()), 0);
/// ```
/// Guarantees that items not requested to be removed will remain untouched.
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, TORCH, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::{quant_in_inventory, remove_from_inventory};
/// # use std::sync::Arc;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(TORCH_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// assert!(remove_from_inventory(&mut inventory, DefaultItemInstance::new(Arc::new(TORCH.clone()), 100)).is_none());
/// assert_eq!(quant_in_inventory(&inventory, TORCH.id()), 23);
/// ```
pub fn remove_from_inventory<Id: Eq, I: Item<Id = Id>, II: ItemInstance<I>, S: Slot<I, II>>(
    inventory: &mut Vec<S>,
    other: II,
) -> Option<II> {
    let remaining = inventory.iter_mut().fold(other.quant(), |current, slot| {
        remove_from_inventory_try_remove(current, slot, &other)
    });
    if remaining == 0 {
        return None;
    }
    return Some(II::new(other.item(), remaining));
}

fn remove_from_inventory_try_remove<
    Id: Eq,
    I: Item<Id = Id>,
    II: ItemInstance<I>,
    S: Slot<I, II>,
>(
    current: u16,
    slot: &mut S,
    other: &II,
) -> u16 {
    let s = match slot.item_instance() {
        None => return current,
        Some(s) => s,
    };
    if current == 0 {
        return 0;
    }
    if s.item().id() != other.item().id() {
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

/// Checks if an item can be added to the given inventory.
///
/// This function evaluates whether an item can be added based on whether it is stackable or unstackable.
/// For stackable items, it checks if there is enough space to add more items to existing stacks to the inventory. For unstackable
/// items, it checks if there are empty slots in the inventory.
///
/// Adding a stackable item to an inventory with space
/// ```
/// # use std::sync::Arc;
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::can_add_item_to_inventory;
/// let inventory = vec![
///     DefaultSlot::new(None),
///     DefaultSlot::new(TORCH_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// let can_add = can_add_item_to_inventory(&inventory, DefaultItemInstance::new(Arc::new(TORCH.clone()), 10));
/// assert!(can_add);
/// ```
///
/// Adding an unstackable item to an inventory with space
/// ```
/// # use std::sync::Arc;
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, SWORD, SWORD_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::can_add_item_to_inventory;
/// let inventory = vec![
///     DefaultSlot::new(SWORD_INST.clone()),
///     DefaultSlot::new(None),
/// ];
/// let can_add = can_add_item_to_inventory(&inventory, DefaultItemInstance::new(Arc::new(SWORD.clone()), 1));
/// assert!(can_add);
/// ```
///
/// Adding to a full inventory
/// ```
/// # use std::sync::Arc;
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, TORCH};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::can_add_item_to_inventory;
/// let inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST.clone()),
///     DefaultSlot::new(SWORD_INST.clone()),
/// ];
/// let can_add = can_add_item_to_inventory(&inventory, DefaultItemInstance::new(Arc::new(TORCH.clone()), 1));
/// assert!(!can_add);
/// ```
///
/// Adding a stackable item to an inventory with partial stacks
/// ```
/// # use std::sync::Arc;
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH, TORCH_INST, JUNK};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::can_add_item_to_inventory;
/// let inventory = vec![
///     DefaultSlot::new(Some(DefaultItemInstance::new(Arc::new(TORCH.clone()), 50))),
///     DefaultSlot::new(Some(DefaultItemInstance::new(Arc::new(JUNK.clone()), 30))),
/// ];
/// let can_add = can_add_item_to_inventory(&inventory, DefaultItemInstance::new(Arc::new(TORCH.clone()), 50));
/// assert!(can_add);
/// ```
///
/// Attempting to add a stackable item that exceeds the maximum quantity
/// ```
/// # use std::sync::Arc;
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::can_add_item_to_inventory;
/// let inventory = vec![
///     DefaultSlot::new(Some(DefaultItemInstance::new(Arc::new(TORCH.clone()), 70))),
/// ];
/// let can_add = can_add_item_to_inventory(&inventory, DefaultItemInstance::new(Arc::new(TORCH.clone()), 31));
/// assert!(!can_add);
/// ```
pub fn can_add_item_to_inventory<Id: Eq, I: Item<Id = Id>, II: ItemInstance<I>, S: Slot<I, II>>(
    inventory: &Vec<S>,
    other: II,
) -> bool {
    if !other.item().stackable() {
        return can_add_unstackable_item_to_inventory(inventory);
    }
    return can_add_stackable_item_to_inventory(inventory, other);
}

fn can_add_unstackable_item_to_inventory<
    Id: Eq,
    I: Item<Id = Id>,
    II: ItemInstance<I>,
    S: Slot<I, II>,
>(
    inventory: &Vec<S>,
) -> bool {
    inventory.iter().any(|slot| slot.item_instance().is_none())
}

fn can_add_stackable_item_to_inventory<
    Id: Eq,
    I: Item<Id = Id>,
    II: ItemInstance<I>,
    S: Slot<I, II>,
>(
    inventory: &Vec<S>,
    mut other: II,
) -> bool {
    let max_stack = other.item().max_quant();
    if other.quant() == max_stack {
        return can_add_unstackable_item_to_inventory(inventory);
    }

    for slot in inventory {
        let item = match slot.item_instance() {
            None => {
                return true;
            }
            Some(x) => x,
        };

        if item.item().id() == other.item().id() {
            if item.quant() == max_stack {
                continue;
            }
            if item.quant() + other.quant() <= max_stack {
                return true;
            }

            let space_available = max_stack - item.quant();
            other = II::new(other.item(), space_available);
        }
    }

    if other.quant() == 0 {
        return true;
    }

    if other.quant() > 0 {
        return can_add_unstackable_item_to_inventory(inventory);
    }

    false
}

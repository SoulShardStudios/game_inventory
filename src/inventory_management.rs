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
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(None),
/// ];
/// assert!(inventory_contains_item_type(&inventory, TORCH.id()));
/// assert!(inventory_contains_item_type(&inventory, SWORD.id()));
/// assert!(!inventory_contains_item_type(&inventory, JUNK.id()));
/// ```
pub fn inventory_contains_item<
    'a,
    Id: Eq,
    I: Item<Id = Id> + 'a,
    II: ItemInstance<'a, I>,
    S: Slot<'a, I, II>,
>(
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
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(None),
/// ];
/// assert!(inventory_contains_item(
///     &inventory,
///     TORCH_FULL_STACK_INST.unwrap()
/// ));
/// assert!(inventory_contains_item(&inventory, SWORD_INST.unwrap()));
/// assert!(!inventory_contains_item(&inventory, TORCH_INST.unwrap()));
/// ```
pub fn inventory_contains_item_type<
    'a,
    Id: Eq,
    I: Item<Id = Id> + 'a,
    II: ItemInstance<'a, I>,
    S: Slot<'a, I, II>,
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
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(TORCH_INST),
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
/// let inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(Some(DefaultItemInstance::new(&SWORD, 123))),
/// ];
/// assert_eq!(quant_in_inventory(&inventory, SWORD.id()), 2)
/// ```
pub fn quant_in_inventory<
    'a,
    Id: Eq,
    I: Item<Id = Id> + 'a,
    II: ItemInstance<'a, I>,
    S: Slot<'a, I, II>,
>(
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
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(None),
///     DefaultSlot::new(TORCH_INST),
///     DefaultSlot::new(None),
/// ];
/// assert_eq!(empty_quant_in_inventory(&inventory), 2)
/// ```
pub fn empty_quant_in_inventory<
    'a,
    Id: Eq,
    I: Item<Id = Id> + 'a,
    II: ItemInstance<'a, I>,
    S: Slot<'a, I, II>,
>(
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
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(TORCH_INST),
/// ];
/// add_to_inventory(&mut inventory, TORCH_INST.unwrap());
/// assert!(inventory[0].item_instance().unwrap().item().id() == TORCH.id());
/// assert!(inventory[0].item_instance().unwrap().quant() == TORCH.max_quant());
/// assert!(inventory[1].item_instance().unwrap().item().id() == SWORD.id());
/// assert!(inventory[2].item_instance().unwrap().item().id() == TORCH.id());
/// assert!(inventory[2].item_instance().unwrap().quant() == TORCH_INST.unwrap().quant() * 2);
/// ```
/// Does not add the item to the given inventory if its full.
/// ```
/// # use game_inventory::samples::{DefaultSlot, TORCH_FULL_STACK_INST, SWORD_INST, TORCH_INST, JUNK_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::add_to_inventory;
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
/// ];
///
/// let instances_to_test = vec![TORCH_INST, TORCH_FULL_STACK_INST, JUNK_INST, SWORD_INST];
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
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(None),
/// ];
/// add_to_inventory(&mut inventory, SWORD_INST.unwrap());
/// assert!(inventory[0].item_instance().unwrap().item().id() == TORCH.id());
/// assert!(inventory[0].item_instance().unwrap().quant() == TORCH.max_quant());
/// assert!(inventory[1].item_instance().unwrap().item().id() == SWORD.id());
/// assert!(inventory[2].item_instance().unwrap().item().id() == SWORD.id());
/// ```
pub fn add_to_inventory<
    'a,
    Id: Eq,
    I: Item<Id = Id> + 'a,
    II: ItemInstance<'a, I> + Clone,
    S: Slot<'a, I, II>,
>(
    inventory: &mut Vec<S>,
    other: II,
) -> Option<II> {
    if inventory.capacity() == 0 {
        return None;
    }
    fn try_add_to_slot<
        'a,
        Id: Eq,
        I: Item<Id = Id> + 'a,
        II: ItemInstance<'a, I> + Clone,
        S: Slot<'a, I, II>,
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
    return inventory.iter_mut().fold(Some(other), try_add_to_slot);
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
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(TORCH_INST),
///     DefaultSlot::new(None),
/// ];
/// assert!(remove_from_inventory(&mut inventory, DefaultItemInstance::new(&TORCH, 123)).is_none());
/// assert_eq!(quant_in_inventory(&inventory, TORCH.id()), 0)
/// ```
/// Does not use unstackable items `.quant()` method, treats every unstackable item as one removal.
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, SWORD, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::{quant_in_inventory, remove_from_inventory};
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(TORCH_INST),
///     DefaultSlot::new(Some(DefaultItemInstance::new(&SWORD, 123))),
/// ];
/// assert!(remove_from_inventory(&mut inventory, DefaultItemInstance::new(&SWORD, 2)).is_none());
/// assert_eq!(quant_in_inventory(&inventory, SWORD.id()), 0);
/// ```
/// If the inventory does not have that may items to remove,
/// this function will return the quantity it was unable to remove.
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, TORCH, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::{quant_in_inventory, remove_from_inventory};
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(None),
///     DefaultSlot::new(None),
/// ];
/// let res = remove_from_inventory(&mut inventory, DefaultItemInstance::new(&TORCH, 123)).unwrap();
/// assert_eq!(res.item().id(), TORCH.id());
/// assert_eq!(res.quant(), 23);
/// assert_eq!(quant_in_inventory(&inventory, TORCH.id()), 0);
/// ```
/// Guarantees that items not requested to be removed will remain untouched.
/// ```
/// # use game_inventory::samples::{DefaultSlot, DefaultItemInstance, TORCH_FULL_STACK_INST, SWORD_INST, TORCH, TORCH_INST};
/// # use game_inventory::traits::{Slot, ItemInstance, Item};
/// # use game_inventory::helpers::{quant_in_inventory, remove_from_inventory};
/// let mut inventory = vec![
///     DefaultSlot::new(TORCH_FULL_STACK_INST),
///     DefaultSlot::new(SWORD_INST),
///     DefaultSlot::new(TORCH_INST),
///     DefaultSlot::new(None),
/// ];
/// assert!(remove_from_inventory(&mut inventory, DefaultItemInstance::new(&TORCH, 100)).is_none());
/// assert_eq!(quant_in_inventory(&inventory, TORCH.id()), 23);
/// ```
pub fn remove_from_inventory<
    'a,
    Id: Eq,
    I: Item<Id = Id> + 'a,
    II: ItemInstance<'a, I> + Clone,
    S: Slot<'a, I, II>,
>(
    inventory: &mut Vec<S>,
    other: II,
) -> Option<II> {
    fn try_remove<
        'a,
        Id: Eq,
        I: Item<Id = Id> + 'a,
        II: ItemInstance<'a, I> + Clone,
        S: Slot<'a, I, II>,
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

    let remaining = inventory.iter_mut().fold(other.quant(), |current, slot| {
        try_remove(current, slot, &other)
    });
    if remaining == 0 {
        return None;
    }
    return Some(II::new(other.item(), remaining));
}

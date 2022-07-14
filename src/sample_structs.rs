use std::fmt::Debug;

use crate::data_types;

#[derive(Debug)]
pub struct Item {
    pub name: &'static str,
    pub max_stack_quantity: u16,
}

impl data_types::IItem for Item {
    fn is_stackable(&self) -> bool {
        true
    }

    fn max_stack_quantity(&self) -> u16 {
        self.max_stack_quantity
    }

    fn name(&self) -> &'static str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemInstance {
    pub item: &'static Item,
    pub quantity: u16,
}

impl<'a> data_types::IItemInstance<Item> for ItemInstance {
    fn get_quantity(&self) -> u16 {
        self.quantity
    }

    fn get_item(&self) -> &'static Item {
        self.item
    }
    fn new(item: &'static Item, quantity: u16) -> Self {
        ItemInstance {
            item: item,
            quantity: quantity,
        }
    }
}

pub struct Slot {
    pub item_instance: Option<ItemInstance>,
    pub on_item_changed: Option<fn(Option<ItemInstance>)>,
}

impl<'a> Debug for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicSlot")
            .field("item_instance", &self.item_instance)
            .finish()
    }
}

impl<'a> data_types::ISlot<'a, Item, ItemInstance> for Slot {
    fn get_item_instance(&self) -> Option<ItemInstance> {
        self.item_instance
    }

    fn set_item_instance(&mut self, item_instance: Option<ItemInstance>) {
        match self.on_item_changed {
            None => {}
            Some(x) => {
                (x)(item_instance);
            }
        }
        self.item_instance = item_instance
    }

    fn set_change_callback(&mut self, callback: Option<fn(Option<ItemInstance>)>) {
        self.on_item_changed = callback
    }
}

#[derive(Debug)]
pub struct Inventory {
    pub slots: Vec<Slot>,
}

impl<'a> data_types::IInventory<'a, Item, ItemInstance, Slot> for Inventory {
    fn size(&self) -> usize {
        self.slots.capacity()
    }

    fn get_slots(&self) -> &[Slot] {
        &self.slots
    }

    fn get_slots_mut(&mut self) -> &mut [Slot] {
        &mut self.slots
    }
}

use std::fmt::Debug;

use crate::data_types;

#[derive(Debug)]
pub struct Item<'a> {
    pub name: &'a str,
    pub max_stack_quantity: u16,
}

impl<'a> data_types::IItem for Item<'a> {
    fn is_stackable(&self) -> bool {
        true
    }

    fn max_stack_quantity(&self) -> u16 {
        self.max_stack_quantity
    }

    fn name(&self) -> &'a str {
        self.name
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemInstance<'a> {
    pub item: &'a Item<'a>,
    pub quantity: u16,
}

impl<'a> data_types::IItemInstance<'a, Item<'a>> for ItemInstance<'a> {
    fn get_quantity(&self) -> u16 {
        self.quantity
    }

    fn get_item(&self) -> &'a Item<'a> {
        self.item
    }

    fn new(item: &'a Item, quantity: u16) -> Self {
        ItemInstance {
            item: item,
            quantity: quantity,
        }
    }
}

pub struct Slot<'a> {
    pub item_instance: Option<ItemInstance<'a>>,
    pub on_item_changed: Option<fn(Option<ItemInstance<'a>>)>,
}

impl<'a> Debug for Slot<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicSlot")
            .field("item_instance", &self.item_instance)
            .finish()
    }
}

impl<'a> data_types::ISlot<'a, Item<'a>, ItemInstance<'a>> for Slot<'a> {
    fn get_item_instance(&self) -> Option<ItemInstance<'a>> {
        self.item_instance
    }

    fn set_item_instance(&mut self, item_instance: Option<ItemInstance<'a>>) {
        match self.on_item_changed {
            None => {}
            Some(x) => {
                (x)(item_instance);
            }
        }
        self.item_instance = item_instance
    }

    fn set_change_callback(&mut self, callback: Option<fn(Option<ItemInstance<'a>>)>) {
        self.on_item_changed = callback
    }
}

#[derive(Debug)]
pub struct Inventory<'a> {
    pub slots: Vec<Slot<'a>>,
}

impl<'a> data_types::IInventory<'a, Item<'a>, ItemInstance<'a>, Slot<'a>> for Inventory<'a> {
    fn size(&self) -> usize {
        self.slots.capacity()
    }

    fn get_slots(&self) -> &[Slot<'a>] {
        &self.slots
    }

    fn get_slots_mut(&mut self) -> &mut [Slot<'a>] {
        &mut self.slots
    }
}

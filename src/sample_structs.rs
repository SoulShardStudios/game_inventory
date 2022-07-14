use crate::data_types;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub max_stack_amount: u16,
}

impl data_types::BaseItem for Item {
    fn is_stackable(&self) -> bool {
        true
    }

    fn max_stack_amount(&self) -> u16 {
        self.max_stack_amount
    }

    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IItem<'a> {
    pub item: &'a Item,
    pub quantity: u16,
}

impl<'a> data_types::ItemInstance<Item> for IItem<'a> {
    fn get_quantity(&self) -> u16 {
        self.quantity
    }

    fn get_item(&self) -> &Item {
        self.item
    }
}

#[derive(Debug)]
pub struct BasicSlot<'a> {
    pub item_instance: Option<IItem<'a>>,
}

impl<'a> data_types::Slot<'a, Item, IItem<'a>> for BasicSlot<'a> {
    fn get_item_instance(&self) -> Option<IItem<'a>> {
        self.item_instance
    }

    fn set_item_instance(&mut self, item_instance: Option<IItem<'a>>) {
        self.item_instance = item_instance
    }
}

#[derive(Debug)]
pub struct BasicInventory<'a> {
    pub slots: Vec<BasicSlot<'a>>,
}

impl<'a> data_types::Inventory<'a, Item, IItem<'a>, BasicSlot<'a>> for BasicInventory<'a> {
    fn size(&self) -> usize {
        self.slots.capacity()
    }

    fn get_slots(&self) -> &[BasicSlot<'a>] {
        &self.slots
    }

    fn get_slots_mut(&mut self) -> &mut [BasicSlot<'a>] {
        &mut self.slots
    }
}

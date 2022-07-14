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

#[derive(Debug)]
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
pub struct BasicInventory<'a> {
    pub items: Vec<Option<IItem<'a>>>,
}

impl<'a> data_types::Inventory<'a, Item, IItem<'a>> for BasicInventory<'a> {
    fn size(&self) -> usize {
        self.items.capacity()
    }

    fn get_items(&self) -> &[Option<IItem<'a>>] {
        &self.items
    }

    fn get_items_mut(&mut self) -> &mut [Option<IItem<'a>>] {
        &mut self.items
    }
}

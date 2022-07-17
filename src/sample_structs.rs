use std::{fmt::Debug, marker::PhantomData};

use crate::traits;

#[derive(Debug)]
pub struct Item<'a> {
    pub name: &'a str,
    pub max_quantity: u16,
}

impl<'a> traits::IDebugItem for Item<'a> {}

impl<'a> traits::IItem for Item<'a> {
    fn stackable(&self) -> bool {
        self.max_quantity > 1
    }

    fn max_quant(&self) -> u16 {
        self.max_quantity
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemInstance<'a> {
    pub item: &'a (dyn traits::IDebugItem),
    pub quantity: u16,
}

impl<'a> traits::IItemInstance<'a> for ItemInstance<'a> {
    fn quant(&self) -> u16 {
        self.quantity
    }

    fn item(&self) -> &'a dyn traits::IDebugItem {
        self.item
    }

    fn new(item: &'a dyn traits::IDebugItem, quantity: u16) -> Self {
        ItemInstance {
            item: item,
            quantity: quantity,
        }
    }
}

pub struct Slot<'a, II>
where
    II: traits::IItemInstance<'a>,
{
    pub item_instance: Option<II>,
    pub modified: bool,
    pub phantom_data: PhantomData<&'a II>,
}

impl<'a, II> Debug for Slot<'a, II>
where
    II: traits::IItemInstance<'a> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicSlot")
            .field("item_instance", &self.item_instance)
            .finish()
    }
}

impl<'a, II> traits::ISlot<'a, II> for Slot<'a, II>
where
    II: traits::IItemInstance<'a> + Sized + Copy,
{
    fn item_instance(&self) -> Option<II> {
        self.item_instance
    }

    fn set_item_instance(&mut self, item_instance: &Option<II>) {
        self.set_modified(true);
        self.item_instance = *item_instance
    }

    fn modified(&mut self) -> bool {
        self.modified
    }

    fn set_modified(&mut self, modified: bool) {
        self.modified = modified
    }

    fn new(item_instance: Option<II>) -> Self {
        Slot {
            item_instance: item_instance,
            modified: false,
            phantom_data: PhantomData,
        }
    }
}

use std::fmt::Debug;

use crate::{combine_stack, traits};
use std::marker::PhantomData;

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

    fn name(&self) -> &'a str {
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
    pub on_item_changed: &'a Option<fn(Option<II>)>,
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
        match &self.item_instance {
            Some(i) => Some(II::new(i.item(), i.quant())),
            None => None,
        }
    }

    fn set_item_instance(&mut self, item_instance: &Option<II>) {
        match self.on_item_changed {
            None => {}
            Some(x) => {
                (x)(*item_instance);
            }
        }
        self.item_instance = *item_instance
    }

    fn set_change_callback(&mut self, callback: &'a Option<fn(Option<II>)>) {
        self.on_item_changed = callback
    }
}

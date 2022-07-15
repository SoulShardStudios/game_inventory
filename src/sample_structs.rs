use std::fmt::Debug;

use crate::traits;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Item<'a> {
    pub name: &'a str,
    pub max_quantity: u16,
}

impl<'a> traits::IDebugItem for Item<'a> {}

impl<'a> traits::IItem for Item<'a> {
    fn stackable(&self) -> bool {
        true
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

impl<'a, II: traits::IItemInstance<'a> + Sized> traits::ISlot<'a, II> for Slot<'a, II> {
    fn get_item_instance(&self) -> Option<II> {
        match &self.item_instance {
            Some(i) => Some(II::new(i.item(), i.quant())),
            None => None,
        }
    }

    fn set_item_instance(&mut self, item_instance: &Option<II>) {
        match self.on_item_changed {
            None => {}
            Some(x) => {
                (x)(match &item_instance {
                    Some(i) => Some(II::new(i.item(), i.quant())),
                    None => None,
                });
            }
        }
        self.item_instance = match &item_instance {
            Some(i) => Some(II::new(i.item(), i.quant())),
            None => None,
        }
    }

    fn set_change_callback(&mut self, callback: &'a Option<fn(Option<II>)>) {
        self.on_item_changed = callback
    }
}

#[derive(Debug)]
pub struct Inventory<'a, II, S>
where
    II: traits::IItemInstance<'a>,
    S: traits::ISlot<'a, II>,
{
    pub slots: Vec<&'a mut S>,
    _phantom: PhantomData<II>,
}

impl<'a, II: traits::IItemInstance<'a>, S: traits::ISlot<'a, II>> traits::IInventory<'a, II, S>
    for Inventory<'a, II, S>
{
    fn size(&self) -> usize {
        self.slots.capacity()
    }

    fn slots(&self) -> &Vec<&'a mut S> {
        &self.slots
    }

    fn slots_mut(&mut self) -> &mut Vec<&'a mut S> {
        &mut self.slots
    }
}

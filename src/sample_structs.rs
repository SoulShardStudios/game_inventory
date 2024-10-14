//! A collection of sample structs used for testing the system, and showing how it can be used.
use std::{fmt::Debug, marker::PhantomData};

use crate::traits::{Item, ItemInstance, Slot};

/// A sample item struct used for testing.
///
/// Has the minimum amount of fields required to make the system work.
///
/// As long as your implementation satisfies the trait bounds it does not matter what immutable
/// item data you put in here.
#[derive(Debug, Clone)]
pub struct DefaultItem<'a> {
    pub name: &'a str,
    pub max_quantity: u16,
}

impl<'a> Item for DefaultItem<'a> {
    type Id = &'a str;
    fn stackable(&self) -> bool {
        self.max_quantity > 1
    }

    fn max_quant(&self) -> u16 {
        self.max_quantity
    }

    fn id(&self) -> &'a str {
        self.name
    }
}

/// A sample item instance struct used for testing.
///
/// Has the minimum amount of fields required to make the system work.
///
/// As long as your implementation satisfies the trait bounds it does not matter what instanced
/// item data you put in here.
#[derive(Debug, Clone)]
pub struct DefaultItemInstance<'a, I: Item> {
    pub item: &'a I,
    pub quantity: u16,
}

impl<'a, I: Item> ItemInstance<'a, I> for DefaultItemInstance<'a, I> {
    fn quant(&self) -> u16 {
        self.quantity
    }

    fn item(&self) -> &'a I {
        self.item
    }

    fn new(item: &'a I, quantity: u16) -> Self {
        DefaultItemInstance { item, quantity }
    }
}

/// A sample slot struct used for testing.
///
/// A significant reduction in boiler plate would
/// come from making your own slot non-generic. For the purpose of an
/// example, I decided that I wanted mine to be generic to prove it was possible
/// if for some reason you need that functionality.
///
/// The main thing you probably want to change other than that is the transfer method.
/// some methods like `half_stack_split` and `combine_stack` would be pretty useful.
pub struct DefaultSlot<'a, I: Item, II: ItemInstance<'a, I>> {
    pub item_instance: Option<II>,
    pub modified: bool,
    pub phantom: PhantomData<&'a I>,
}

impl<'a, I: Item, II: ItemInstance<'a, I> + Debug> Debug for DefaultSlot<'a, I, II> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicSlot")
            .field("item_instance", &self.item_instance)
            .field("modified", &self.modified)
            .finish()
    }
}

impl<'a, I: Item, II: ItemInstance<'a, I> + Sized + Clone> Slot<'a, I, II>
    for DefaultSlot<'a, I, II>
{
    fn item_instance(&self) -> Option<II> {
        self.item_instance.clone()
    }

    fn set_item_instance(&mut self, item_instance: &Option<II>) {
        self.set_modified(true);
        self.item_instance = item_instance.clone()
    }

    fn modified(&mut self) -> bool {
        self.modified
    }

    fn set_modified(&mut self, modified: bool) {
        self.modified = modified
    }

    fn new(item_instance: Option<II>) -> Self {
        DefaultSlot {
            item_instance,
            modified: false,
            phantom: PhantomData,
        }
    }
}

//! A collection of sample structs used for testing the system, and showing how it can be used.
use std::{fmt::Debug, marker::PhantomData};

use crate::traits::{IItem, IItemInstance, ISlot};

/// A sample item struct used for testing.
///
/// Has the minimum amount of fields required to make the system work.
///
/// As long as your implementation satisfies the trait bounds it does not matter what immutable
/// item data you put in here.
#[derive(Debug)]
pub struct DefaultItem<'a> {
    pub name: &'a str,
    pub max_quantity: u16,
}

impl<'a> IItem for DefaultItem<'a> {
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

/// A sample item instance struct used for testing.
///
/// Has the minimum amount of fields required to make the system work.
///
/// As long as your implementation satisfies the trait bounds it does not matter what instanced
/// item data you put in here.
#[derive(Debug, Clone, Copy)]
pub struct DefaultItemInstance<'a> {
    pub item: &'a dyn IItem,
    pub quantity: u16,
}

impl<'a> IItemInstance<'a> for DefaultItemInstance<'a> {
    fn quant(&self) -> u16 {
        self.quantity
    }

    fn item(&self) -> &'a dyn IItem {
        self.item
    }

    fn new(item: &'a dyn IItem, quantity: u16) -> Self {
        DefaultItemInstance {
            item: item,
            quantity: quantity,
        }
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
pub struct DefaultSlot<'a, II>
where
    II: IItemInstance<'a>,
{
    pub item_instance: Option<II>,
    pub modified: bool,
    pub phantom: PhantomData<&'a II>,
}

impl<'a, II> Debug for DefaultSlot<'a, II>
where
    II: IItemInstance<'a> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicSlot")
            .field("item_instance", &self.item_instance)
            .field("modified", &self.modified)
            .finish()
    }
}

impl<'a, II> ISlot<'a, II> for DefaultSlot<'a, II>
where
    II: IItemInstance<'a> + Sized + Copy,
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
        DefaultSlot {
            item_instance: item_instance,
            modified: false,
            phantom: PhantomData,
        }
    }
}

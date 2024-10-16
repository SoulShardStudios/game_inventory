//! All traits that are needed to interface with the inventory system.

use crate::slot_management::swap;
/// Trait for defining what static item data is necessary for the inventory system.
///
/// Static item data are things like the items name, the items base damage. Data
/// about the item that does not change item stack to item stack should be stored here.
pub trait Item {
    type Id: Eq;
    /// Whether the item can be put into stacks.
    /// A sword you may only want to have one of,
    /// While throwing knives may be stackable.
    fn stackable(&self) -> bool;
    /// The maximum quantity of a stack. This does not matter
    /// for non stackable items.
    fn max_quant(&self) -> u16;
    /// The Unique ID of the item
    fn id(&self) -> Self::Id;
}
/// Trait for storing item instance data.
///
/// If you have two stacks of items, the quantity of items
/// in each stack is stored separately. This is where you store that data.
pub trait ItemInstance<'a, I: Item> {
    /// The quantity of items in this instance.
    fn quant(&self) -> u16;
    /// The item stored by this instance.
    fn item(&self) -> &'a I;
    /// Creates a new item instance.
    fn new(item: &'a I, quantity: u16) -> Self;
}
/// Trait for defining an item slot.
///
/// Manages access and data binding for item instances. If you need to change the behavior
/// by which the player is able to modify the stored item instance, for example,
/// restricting it to only be items where the item type is `ItemType::Armor`, this is the place to
/// define that behavior for.
pub trait Slot<'a, I: Item, II: ItemInstance<'a, I> + Sized> {
    /// Get the item instance stored by this slot.
    fn item_instance(&self) -> Option<II>;
    /// Set the item instance stored by this slot.
    fn set_item_instance(&mut self, item_instance: &Option<II>);
    /// Switch out the item instance stored in this slot with some rules.
    ///
    /// The input is the item you intend to put into the slot and the output
    /// is the item the slot is expelling.
    ///
    /// Here are some examples of why this is useful: If you click a slot in the UI,
    /// there are a variety of behaviors that can occur. Maybe on right clicking you
    /// want to split the current stack in half, or swap the item your holding
    /// with the item in the slot.
    ///
    /// The button variable is necessary for changing the behavior based on user input.
    fn transfer(&mut self, item_instance: Option<II>, _button: &str) -> Option<II> {
        let res = swap((self.item_instance(), item_instance));
        self.set_modified(true);
        self.set_item_instance(&res.0);
        res.1
    }
    /// Whether the slots contents have been modified.
    fn modified(&mut self) -> bool;
    /// Sets whether the slots contents have been modified.
    fn set_modified(&mut self, modified: bool);
    fn new(item_instance: Option<II>) -> Self;
}

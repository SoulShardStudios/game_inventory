//! A collection of sample items.
//! Good for writing tests and seeing what item definitions look like.
//!
//! The reason that there are two stackable items is
//! so that you can test that items with different names
//! do not combine together or any other weird behavior.

use crate::sample_structs::{Item, ItemInstance};

/// A simple stackable item.
pub const TORCH: Item = Item {
    name: "torch",
    max_quantity: 100,
};

/// A simple stackable item.
pub const JUNK: Item = Item {
    name: "junk",
    max_quantity: 100,
};

/// A simple unstackable item.
/// It is unstackable because in the impl
/// for IItem, `stackable()` is just `self.max_quantity > 1`.
/// if its 0, that means it cannot be stacked with other items.
pub const SWORD: Item = Item {
    name: "sword",
    max_quantity: 0,
};

/// An ItemInstance of the sword item for testing slot and inventory management.
pub const SWORD_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &SWORD,
    quantity: 0,
});

/// An ItemInstance of the junk item for testing slot and inventory management.
pub const JUNK_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &JUNK,
    quantity: 91,
});

/// An ItemInstance of the torch item for testing slot and inventory management.
pub const TORCH_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &TORCH,
    quantity: 23,
});

/// An ItemInstance of the torch that has a full stack, as `self.quantity == self.item.max_quant()`
pub const TORCH_FULL_STACK_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &TORCH,
    quantity: 100,
});

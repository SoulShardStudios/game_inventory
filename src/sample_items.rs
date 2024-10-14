//! A collection of sample items.
//! Good for writing tests and seeing what item definitions look like.
//!
//! The reason that there are two stackable items is
//! so that you can test that items with different names
//! do not combine together or any other weird behavior.

use std::sync::Arc;

use crate::sample_structs::{DefaultItem, DefaultItemInstance};

/// A simple stackable item.
pub const TORCH: DefaultItem = DefaultItem {
    name: "torch",
    max_quantity: 100,
};

/// A simple stackable item.
pub const JUNK: DefaultItem = DefaultItem {
    name: "junk",
    max_quantity: 100,
};

/// A simple unstackable item.
/// It is unstackable because in the impl
/// for Item, `stackable()` is just `self.max_quantity > 1`.
/// if its 0, that means it cannot be stacked with other items.
pub const SWORD: DefaultItem = DefaultItem {
    name: "sword",
    max_quantity: 0,
};

/// An DefaultItemInstance of the sword item for testing slot and inventory management.
pub const SWORD_INST: Option<DefaultItemInstance<DefaultItem>> = Some(DefaultItemInstance {
    item: &SWORD,
    quantity: 0,
});

/// An DefaultItemInstance of the junk item for testing slot and inventory management.
pub const JUNK_INST: Option<DefaultItemInstance<DefaultItem>> = Some(DefaultItemInstance {
    item: &JUNK,
    quantity: 91,
});

/// An DefaultItemInstance of the torch item for testing slot and inventory management.
pub const TORCH_INST: Option<DefaultItemInstance<DefaultItem>> = Some(DefaultItemInstance {
    item: &TORCH,
    quantity: 23,
});

/// An DefaultItemInstance of the torch that has a full stack, as `self.quantity == self.item.max_quant()`
pub const TORCH_FULL_STACK_INST: Option<DefaultItemInstance<DefaultItem>> =
    Some(DefaultItemInstance {
        item: &TORCH,
        quantity: 100,
    });

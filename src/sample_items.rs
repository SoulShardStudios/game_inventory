//! A collection of sample items.
//! Good for writing tests and seeing what item definitions look like.
//!
//! The reason that there are two stackable items is
//! so that you can test that items with different names
//! do not combine together or any other weird behavior.

use std::sync::Arc;

use crate::sample_structs::{DefaultItem, DefaultItemInstance};

// Define simple stackable items as static, not const

/// A simple stackable item.
pub static TORCH: DefaultItem = DefaultItem {
    name: "torch",
    max_quantity: 100,
};

/// A simple stackable item.
pub static JUNK: DefaultItem = DefaultItem {
    name: "junk",
    max_quantity: 100,
};

/// A simple unstackable item.
pub static SWORD: DefaultItem = DefaultItem {
    name: "sword",
    max_quantity: 0,
};

lazy_static::lazy_static! {
    /// A DefaultItemInstance of the sword item for testing slot and inventory management.
    pub static ref SWORD_INST: Option<DefaultItemInstance<DefaultItem<'static>>> = Some(DefaultItemInstance {
        item: Arc::new(SWORD.clone()),
        quantity: 0,
    });

    /// A DefaultItemInstance of the junk item for testing slot and inventory management.
    pub static ref JUNK_INST: Option<DefaultItemInstance<DefaultItem<'static>>> = Some(DefaultItemInstance {
        item: Arc::new(JUNK.clone()),
        quantity: 91,
    });

    /// A DefaultItemInstance of the torch item for testing slot and inventory management.
    pub static ref TORCH_INST: Option<DefaultItemInstance<DefaultItem<'static>>> = Some(DefaultItemInstance {
        item: Arc::new(TORCH.clone()),
        quantity: 23,
    });

    /// A DefaultItemInstance of the torch that has a full stack.
    pub static ref TORCH_FULL_STACK_INST: Option<DefaultItemInstance<DefaultItem<'static>>> = Some(DefaultItemInstance {
        item: Arc::new(TORCH.clone()),
        quantity: 100,
    });
}

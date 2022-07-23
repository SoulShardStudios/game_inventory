//! A framework for generalizing inventory logic and abstracting it away from
//! item data in your specific game.  
//!
//! ## Design specifications
//! - Everything should be interchangeable and as generic as possible.
//! - The architecture should support item instance data and item metadata.
//! - Should be very reliable (made in rust + unit tests).
//! - Fast to set up in new games.
//!
//! ## Restrictions
//! The only assumption that this framework makes is that your items have stacks.
//! Even if your items do not have stacks and are only single items, you can still workshop
//! that to work with this system but it will be more inefficient. However, if your inventory
//! system fundamentally works differently, feel free to take inspiration from the design in
//! here while making your specific tweaks.
//!
//! ## Overall architecture
//!
//! - `trait IItem` Item data that never changes, like how the item looks, its base damage, its description e.t.c.
//! - `trait IItemInstance` Item data that changes between instances, like enchantments, how many you have, their durability, e.t.c.
//! - `trait ISlot` Manages a single item instance. Good for binding user action to different types of instance modification (stack splitting, stack combining, e.t.c.). Allows for binding to the UI via a callback function.
//! - `Vec<ISlot>` Is the way an inventory is composed. There are builtin functions in `inventory_management` that can help manage the inventory.

pub mod inventory_management;
pub mod sample_items;
pub mod sample_structs;
pub mod slot_management;
pub mod traits;

/// A combination of the `sample_items` and `sample_structs` crates.
///
/// These are used for tests and examples, and if you want you can use them.
pub mod samples {
    pub use crate::sample_items::*;
    pub use crate::sample_structs::*;
}

/// A combination of the `inventory_management` and `slot_management` crates.
///
/// This is a container for all helper functions,
/// whether they are inventory or slot related.
pub mod helpers {
    pub use crate::inventory_management::*;
    pub use crate::slot_management::*;
}

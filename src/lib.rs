//! A framework for generalizing inventory logic and abstracting it away from
//! item data in your specific game.  
//!
//! ## Important note:
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
//! - If you want to manage a collection of slots, simply put them in a `Vec`, there are builtin functions to operate over a `Vec<Slot>`(`add_item_to_inventory`,`inventory_contains_item`,e.t.c.)

pub mod inventory_management;
pub mod sample_structs;
pub mod slot_management;
pub mod traits;

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
//!
//! ## Basic example
//!
//! ```
//! # use game_inventory::traits::{IItem, IItemInstance, ISlot};
//! # use game_inventory::sample_structs::{ItemInstance, Slot};
//! # use game_inventory::helpers::add_to_inventory;
//! // Define your item data however you like:
//! #[derive(Debug)]
//! pub struct Item<'a> {
//!     pub name: &'a str,
//!     pub max_quantity: u16,
//!     pub image: Option<Vec<(u8,u8,u8,u8)>>,
//!     pub item_type: &'a str
//! }
//! // implement IItem for it so it can interact with the rest of the system.
//! impl<'a> IItem for Item<'a> {
//!     fn stackable(&self) -> bool {
//!         self.max_quantity > 1
//!     }
//!
//!     fn max_quant(&self) -> u16 {
//!         self.max_quantity
//!     }
//!
//!     fn name(&self) -> &str {
//!         self.name
//!     }
//! }
//! // start using it in combination with everything else!
//! const CHEESE: Item = Item{name:"Cheese", max_quantity:100, image:None, item_type:"Food"};
//! const CHEESE_INST: Option<ItemInstance> = Some(ItemInstance{item:&CHEESE, quantity:32});
//! const SWORD: Item = Item{name:"Sword", max_quantity:0, image:None, item_type:"Weapon"};
//! const SWORD_INST: Option<ItemInstance> = Some(ItemInstance{item:&SWORD, quantity:0});
//! let mut inventory = vec![
//!     Slot::new(CHEESE_INST),
//!     Slot::new(None),
//!     Slot::new(None),
//!     Slot::new(CHEESE_INST)
//! ];
//! add_to_inventory(&mut inventory, SWORD_INST.unwrap());
//! assert_eq!(inventory[0].item_instance().unwrap().item().name(), CHEESE.name());
//! assert_eq!(inventory[0].item_instance().unwrap().quant(), CHEESE_INST.unwrap().quant());
//! assert_eq!(inventory[1].item_instance().unwrap().item().name(), SWORD.name());
//! assert!(inventory[2].item_instance().is_none());
//! assert_eq!(inventory[3].item_instance().unwrap().item().name(), CHEESE.name());
//! assert_eq!(inventory[3].item_instance().unwrap().quant(), CHEESE_INST.unwrap().quant());
//! ```

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

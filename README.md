# game_inventory

A framework for generalizing inventory logic and abstracting it away from
item data in your specific game.

See more examples and specific documentation about this crate on [docs.rs](https://docs.rs/game_inventory/latest/game_inventory/).

## Basic usage

```rs
use game_inventory::traits::{Item, ItemInstance, Slot};
use game_inventory::sample_structs::{DefaultItemInstance, DefaultSlot};
use game_inventory::helpers::add_to_inventory;
// Define your item data however you like.
#[derive(Debug)]
pub struct DefaultItem<'a> {
    pub name: &'a str,
    pub max_quantity: u16,
    pub image: Option<Vec<(u8,u8,u8,u8)>>,
    pub item_type: &'a str
}
// implement Item for it so it can interact with the rest of the system.
impl<'a> Item for DefaultItem<'a> {
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
// start using it in combination with everything else!
const CHEESE: DefaultItem = DefaultItem{name:"Cheese", max_quantity:100, image:None, item_type:"Food"};
const CHEESE_INST: Option<DefaultItemInstance> = Some(DefaultItemInstance{item:&CHEESE, quantity:32});
const SWORD: DefaultItem = DefaultItem{name:"Sword", max_quantity:0, image:None, item_type:"Weapon"};
const SWORD_INST: Option<DefaultItemInstance> = Some(DefaultItemInstance{item:&SWORD, quantity:0});
let mut inventory = vec![
    DefaultSlot::new(CHEESE_INST),
    DefaultSlot::new(None),
    DefaultSlot::new(None),
    DefaultSlot::new(CHEESE_INST)
];
add_to_inventory(&mut inventory, SWORD_INST.unwrap());
assert_eq!(inventory[0].item_instance().unwrap().item().name(), CHEESE.name());
assert_eq!(inventory[0].item_instance().unwrap().quant(), CHEESE_INST.unwrap().quant());
assert_eq!(inventory[1].item_instance().unwrap().item().name(), SWORD.name());
assert!(inventory[2].item_instance().is_none());
assert_eq!(inventory[3].item_instance().unwrap().item().name(), CHEESE.name());
assert_eq!(inventory[3].item_instance().unwrap().quant(), CHEESE_INST.unwrap().quant());
```

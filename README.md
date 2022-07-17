# inventory-rs

A fully generic, expandable inventory system built in rust.

## Design specifications

- Everything should be interchangeable and as generic as possible
- The architecture should support item instance data and item metadata
- Should be entirely bug free (unit tests and built in rust, so it should be good)
- Fast to set up in new games

## Overall architecture and data types

- `trait IItem` Item data that never changes, like how the item looks, its base damage, its description e.t.c.
- `trait IItemInstance` Item data that changes between instances, like enchantments, how many you have, their durability, e.t.c.
- `trait ISlot` Manages a single item instance. Good for binding user action to different types of instance modification (stack splitting, stack combining, e.t.c.). Allows for binding to the UI via a callback function.
- If you want to manage a collection of slots, simply put them in a `Vec`, there are builtin functions to operate over a `Vec<Slot>`(`add_item_to_inventory`,`inventory_contains_item`,e.t.c.)

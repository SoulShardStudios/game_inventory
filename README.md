# inventory-rs
A fully generic, expandable inventory system built in rust.

## Design specifications

 - Everything should be interchangeable and as generic as possible
 - The architecture should support item instance data and item metadata
 - Should be entirely bug free (unit tests and bult in rust, so it should be good)
 - Fast to set up in new games
 
## Overall architecture and data types

 - `trait IItem` The broadsword item should always look the same and have the same base damage. This is that data.  
 - `trait IItemInstance` This is typically item that changes between instances like different stacks containing different amounts.  
 - `trait ISlot` Manages a single item instance. Good for binding user action to different types of instance modification (stack splitting, stack combining, e.t.c.).  
 - If you want to manage a collection of slots, simply put them in a `Vec`, there are builtins to operate over a vec of slots (`add_item_to_inventory`,`inventory_contains_item`,e.t.c.)

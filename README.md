# inventory-rs
A fully generic, expandable inventory system built in rust.

## Design specifications

 - Everything should be interchangeable and as generic as possible
 - The architecture should support Item instance data and item metadata
 - Tests are required for sorting out bugs
 - Built with rust to include of the robust guarantees rust ensures
 
## Overall architecture and data types

 - `trait IItem` Is for static item data. say you have a sword, between different swords, they have the same icon, the same base damage, e.t.c. that gets stored in static item data.
 - `trait IItemInstance` Is for instance item data. If there are two stacks of an item, you need to store the quantity of those items. That gets stored in instance data. 
 - `trait ISlot` Manages a single item instance. Good for binding user action to different types of instance modification (stack splitting, stack combining, e.t.c.).
 - `trait IInventory` Manages a collection of item instances. good for applying operations over a whole collection of slots, or just simply storing slots together.  

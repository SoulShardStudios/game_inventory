# inventory-rs
A fully generic, expandable inventory system built in rust.

## Design specifications

 - Everything should be interchangeable and as generic as possible
 - The architecture should support Item instance data and item metadata
 - Tests are required for sorting out bugs
 - Built with rust to include of the robust guarantees rust ensures
 
## Overall architecture and data types

 - `trait IItem` The broadsword item should always look the same and have the same base damage. This is that data.  
 - `trait IItemInstance` This is typically item that changes between instances like different stacks containing different amounts.  
 - `trait ISlot` Manages a single item instance. Good for binding user action to different types of instance modification (stack splitting, stack combining, e.t.c.).  
 - `trait IInventory` Manages a collection of item instances. good for applying operations over a whole collection of slots, or just simply storing slots together.  

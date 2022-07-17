# inventory-rs

A framework for generalizing inventory logic and abstracting it away from
item data in your specific game.

## Design specifications

- Everything should be interchangeable and as generic as possible.
- The architecture should support item instance data and item metadata.
- Should be very reliable (made in rust + unit tests).
- Fast to set up in new games.

## Restrictions

The only assumption that this framework makes is that your items have stacks.
Even if your items do not have stacks and are only single items, you can still workshop
that to work with this system but it will be more inefficient. However, if your inventory
system fundamentally works differently, feel free to take inspiration from the design in
here while making your specific tweaks.

## Overall architecture

- `trait IItem` Item data that never changes, like how the item looks, its base damage, its description e.t.c.
- `trait IItemInstance` Item data that changes between instances, like enchantments, how many you have, their durability, e.t.c.
- `trait ISlot` Manages a single item instance. Good for binding user action to different types of instance modification (stack splitting, stack combining, e.t.c.). Allows for binding to the UI via a callback function.
- `Vec<ISlot>` Is the way an inventory is composed. There are builtin functions in `inventory_management` that can help manage the inventory.

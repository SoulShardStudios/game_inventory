use game_inventory::sample_structs::{Item, ItemInstance};

pub const TORCH: Item = Item {
    name: "torch",
    max_quantity: 100,
};

pub const JUNK: Item = Item {
    name: "junk",
    max_quantity: 100,
};

pub const SWORD: Item = Item {
    name: "sword",
    max_quantity: 0,
};

pub const SWORD_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &SWORD,
    quantity: 0,
});

pub const JUNK_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &JUNK,
    quantity: 91,
});

pub const TORCH_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &TORCH,
    quantity: 23,
});

pub const TORCH_FULL_STACK_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &TORCH,
    quantity: 100,
});

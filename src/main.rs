mod data_types;
mod sample_structs;
use crate::data_types::Inventory;
use crate::data_types::Slot;
use crate::sample_structs::BasicInventory;
use crate::sample_structs::BasicSlot;
use crate::sample_structs::IItem;
use crate::sample_structs::Item;

fn main() {
    let test_item = Item {
        name: "brocoli".to_string(),
        max_stack_amount: 100,
    };

    let mut inv = BasicInventory {
        slots: vec![BasicSlot {
            item_instance: Some(IItem {
                item: &test_item,
                quantity: 10,
            }),
        }],
    };

    let inst2 = inv.get_slots_mut()[0].transfer(Some(IItem {
        item: &test_item,
        quantity: 102,
    }));

    println!("{:#?}", inst2);

    println!("{:#?}", test_item);

    println!("{:#?}", inv);
}

mod data_types;
mod sample_structs;
use crate::sample_structs::BasicInventory;
use crate::sample_structs::IItem;
use crate::sample_structs::Item;

fn main() {
    let test_item = Item {
        name: "brocoli".to_string(),
        max_stack_amount: 100,
    };

    let inv = BasicInventory {
        items: vec![Some(IItem {
            item: &test_item,
            quantity: 10,
        })],
    };

    println!("{:#?}", test_item);

    println!("{:#?}", inv);
}

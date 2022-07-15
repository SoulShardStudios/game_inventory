mod data_types;
mod sample_structs;
mod slot_management;
use crate::data_types::IInventory;
use crate::data_types::ISlot;
use crate::sample_structs::Inventory;
use crate::sample_structs::Item;
use crate::sample_structs::ItemInstance;
use crate::sample_structs::Slot;

fn main() {
    fn on_change(iitem: Option<ItemInstance>) {
        println!("change callback:{:#?}", iitem)
    }

    let test_item: Item = Item {
        name: "brocoli",
        max_stack_quantity: 100,
    };

    let mut inv = Inventory {
        slots: vec![Slot {
            item_instance: Some(ItemInstance {
                item: &test_item,
                quantity: 10,
            }),
            on_item_changed: Some(on_change),
        }],
    };

    let inst2 = inv.get_slots_mut()[0].transfer(Some(ItemInstance {
        item: &test_item,
        quantity: 102,
    }));

    println!("{:#?}", inst2);

    println!("{:#?}", test_item);

    println!("{:#?}", inv);
}

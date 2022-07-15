use inventory_rs::{ISlot, Item, ItemInstance, Slot};

#[test]
fn test_combine_stack() {
    let test_item: Item = Item {
        name: "brocoli",
        max_quantity: 100,
    };

    let mut slot = Slot {
        item_instance: Some(ItemInstance {
            item: &test_item,
            quantity: 10,
        }),
        on_item_changed: &None,
    };
    let inst2 = slot.transfer(Some(ItemInstance {
        item: &test_item,
        quantity: 102,
    }));
}

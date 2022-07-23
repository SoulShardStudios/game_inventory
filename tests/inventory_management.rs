/*
mod items;
use game_inventory::inventory_management::{
    add_to_inventory, empty_quant_in_inventory, inventory_contains_item,
    inventory_contains_item_type, quant_in_inventory, remove_from_inventory,
};
use game_inventory::sample_structs::{ItemInstance, Slot};
use game_inventory::traits::{IItem, IItemInstance, ISlot};
use items::{JUNK, JUNK_INST, SWORD, SWORD_INST, TORCH, TORCH_FULL_STACK_INST, TORCH_INST};

#[test]
fn test_empty_quant_in_inventory() {
    let inventory = vec![
        Slot::new(TORCH_FULL_STACK_INST),
        Slot::new(SWORD_INST),
        Slot::new(None),
        Slot::new(TORCH_INST),
        Slot::new(None),
    ];
    assert_eq!(empty_quant_in_inventory(&inventory), 2)
}

mod add_to {
    use super::*;

    #[test]
    fn full() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
        ];

        let instances_to_test = vec![TORCH_INST, TORCH_FULL_STACK_INST, JUNK_INST, SWORD_INST];
        instances_to_test.iter().for_each(|inst| {
            let res = add_to_inventory(&mut inventory, inst.unwrap());
            assert!(res.unwrap().item().name() == inst.unwrap().item().name());
            assert!(res.unwrap().quant() == inst.unwrap().quant());
        });
    }

    #[test]
    fn stackable() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(TORCH_INST),
        ];
        add_to_inventory(&mut inventory, TORCH_INST.unwrap());
        assert!(inventory[0].item_instance().unwrap().item().name() == TORCH.name());
        assert!(inventory[0].item_instance().unwrap().quant() == TORCH.max_quant());
        assert!(inventory[1].item_instance().unwrap().item().name() == SWORD.name());
        assert!(inventory[2].item_instance().unwrap().item().name() == TORCH.name());
        assert!(inventory[2].item_instance().unwrap().quant() == TORCH_INST.unwrap().quant() * 2);
    }

    #[test]
    fn stackable_with_none() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(None),
            Slot::new(TORCH_INST),
        ];
        add_to_inventory(&mut inventory, TORCH_INST.unwrap());
        assert!(inventory[0].item_instance().unwrap().item().name() == TORCH.name());
        assert!(inventory[0].item_instance().unwrap().quant() == TORCH.max_quant());
        assert!(inventory[1].item_instance().unwrap().item().name() == TORCH.name());
        assert!(inventory[1].item_instance().unwrap().quant() == TORCH_INST.unwrap().quant());
        assert!(inventory[2].item_instance().unwrap().item().name() == TORCH.name());
        assert!(inventory[2].item_instance().unwrap().quant() == TORCH_INST.unwrap().quant());
    }

    #[test]
    fn unstackable() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(None),
        ];
        add_to_inventory(&mut inventory, SWORD_INST.unwrap());
        assert!(inventory[0].item_instance().unwrap().item().name() == TORCH.name());
        assert!(inventory[0].item_instance().unwrap().quant() == TORCH.max_quant());
        assert!(inventory[1].item_instance().unwrap().item().name() == SWORD.name());
        assert!(inventory[2].item_instance().unwrap().item().name() == SWORD.name());
    }
}
mod contains {
    use super::*;
    #[test]
    fn contains_type() {
        let full = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(None),
        ];
        assert!(inventory_contains_item_type(&full, TORCH.name()));
        assert!(inventory_contains_item_type(&full, SWORD.name()));
        assert!(!inventory_contains_item_type(&full, JUNK.name()));
    }
    #[test]
    fn contains_item() {
        let inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(None),
        ];
        assert!(inventory_contains_item(
            &inventory,
            TORCH_FULL_STACK_INST.unwrap()
        ));
        assert!(inventory_contains_item(&inventory, SWORD_INST.unwrap()));
        assert!(!inventory_contains_item(&inventory, TORCH_INST.unwrap()));
    }
}
mod quant_in_inventory {
    use super::*;
    #[test]
    fn stackable() {
        let inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(TORCH_INST),
            Slot::new(None),
        ];
        assert_eq!(quant_in_inventory(&inventory, TORCH.name()), 123)
    }
    #[test]
    fn unstackable() {
        let inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(Some(ItemInstance::new(&SWORD, 123))),
        ];
        assert_eq!(quant_in_inventory(&inventory, SWORD.name()), 2)
    }
}

mod remove_from_inventory {
    use super::*;
    #[test]
    fn stackable() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(TORCH_INST),
            Slot::new(None),
        ];
        assert!(remove_from_inventory(&mut inventory, ItemInstance::new(&TORCH, 123)).is_none());
        assert_eq!(quant_in_inventory(&inventory, TORCH.name()), 0)
    }
    #[test]
    fn unstackable() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(TORCH_INST),
            Slot::new(Some(ItemInstance::new(&SWORD, 123))),
        ];
        assert!(remove_from_inventory(&mut inventory, ItemInstance::new(&SWORD, 2)).is_none());
        assert_eq!(quant_in_inventory(&inventory, SWORD.name()), 0)
    }
    #[test]
    fn leftover() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(None),
            Slot::new(None),
        ];
        let res = remove_from_inventory(&mut inventory, ItemInstance::new(&TORCH, 123)).unwrap();
        assert_eq!(res.item().name(), TORCH.name());
        assert_eq!(res.quant(), 23);
        assert_eq!(quant_in_inventory(&inventory, TORCH.name()), 0);
    }

    #[test]
    fn remaining() {
        let mut inventory = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(TORCH_INST),
            Slot::new(None),
        ];
        assert!(remove_from_inventory(&mut inventory, ItemInstance::new(&TORCH, 100)).is_none());
        assert_eq!(quant_in_inventory(&inventory, TORCH.name()), 23);
    }
}
*/

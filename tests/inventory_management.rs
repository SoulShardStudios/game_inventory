mod items;
use inventory_rs::{
    add_to_inventory, inventory_contains_item, inventory_contains_item_type, IItem, IItemInstance,
    ISlot, Slot,
};
use items::{JUNK, JUNK_INST, SWORD, SWORD_INST, TORCH, TORCH_FULL_STACK_INST, TORCH_INST};
mod add_to {
    use super::*;

    #[test]
    fn full() {
        let mut full = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
        ];

        let insts_to_test = vec![TORCH_INST, TORCH_FULL_STACK_INST, JUNK_INST, SWORD_INST];
        insts_to_test.iter().for_each(|inst| {
            let res = add_to_inventory(&mut full, *inst);
            assert!(res.unwrap().item().name() == inst.unwrap().item().name());
            assert!(res.unwrap().quant() == inst.unwrap().quant());
        });
    }

    #[test]
    fn stackable() {
        let mut full = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(TORCH_INST),
        ];
        add_to_inventory(&mut full, TORCH_INST);
        assert!(full[0].item_instance().unwrap().item().name() == TORCH.name());
        assert!(full[0].item_instance().unwrap().quant() == TORCH.max_quant());
        assert!(full[1].item_instance().unwrap().item().name() == SWORD.name());
        assert!(full[2].item_instance().unwrap().item().name() == TORCH.name());
        assert!(full[2].item_instance().unwrap().quant() == TORCH_INST.unwrap().quant() * 2);
    }

    #[test]
    fn unstackable() {
        let mut full = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(None),
        ];
        add_to_inventory(&mut full, SWORD_INST);
        assert!(full[0].item_instance().unwrap().item().name() == TORCH.name());
        assert!(full[0].item_instance().unwrap().quant() == TORCH.max_quant());
        assert!(full[1].item_instance().unwrap().item().name() == SWORD.name());
        assert!(full[2].item_instance().unwrap().item().name() == SWORD.name());
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
        let full = vec![
            Slot::new(TORCH_FULL_STACK_INST),
            Slot::new(SWORD_INST),
            Slot::new(None),
        ];
        assert!(inventory_contains_item(&full, TORCH_FULL_STACK_INST));
        assert!(inventory_contains_item(&full, SWORD_INST));
        assert!(!inventory_contains_item(&full, TORCH_INST));
    }
}

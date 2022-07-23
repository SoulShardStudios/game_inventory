/*
use game_inventory::sample_items::{
    JUNK_INST, SWORD_INST, TORCH, TORCH_FULL_STACK_INST, TORCH_INST,
};
use game_inventory::sample_structs::{ItemInstance, Slot};
use game_inventory::slot_management::{combine_stack, half_stack_split, remove_from_stack, swap};
use game_inventory::traits::{IItem, IItemInstance, ISlot};

#[test]
fn test_swap() {}
mod combine {
    use super::*;

    #[test]
    fn combine_edge_cases() {

    }
}
mod split {
    use super::*;
    #[test]
    fn simple() {
        let res = half_stack_split((
            Some(ItemInstance {
                item: &TORCH,
                quantity: 10,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 2,
            }),
        ));
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 5);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 7);
    }

    #[test]
    fn uneven() {
        let res = half_stack_split((
            Some(ItemInstance {
                item: &TORCH,
                quantity: 11,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 3,
            }),
        ));
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 5);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 9);
    }

    #[test]
    fn none() {
        let res = half_stack_split((
            Some(ItemInstance {
                item: &TORCH,
                quantity: 11,
            }),
            None,
        ));
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 5);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 6);
    }

    #[test]
    fn edge_cases() {
        assert_was_swapped(
            (
                None,
                Some(ItemInstance {
                    item: &TORCH,
                    quantity: 34,
                }),
            ),
            combine_stack,
        );
        assert_was_swapped((JUNK_INST, None), combine_stack);
        assert_was_swapped((None, SWORD_INST), combine_stack);
        assert_was_swapped((SWORD_INST, None), combine_stack);
        assert_was_swapped((TORCH_INST, JUNK_INST), combine_stack);
        assert_was_swapped((JUNK_INST, TORCH_INST), combine_stack);
    }
}
mod remove {
    use super::*;

    #[test]
    fn simple() {
        let res = remove_from_stack((
            Some(ItemInstance {
                item: &TORCH,
                quantity: 3,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
        ));
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 2);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 21);
    }
    #[test]
    fn other_empty() {
        let res = remove_from_stack((
            Some(ItemInstance {
                item: &TORCH,
                quantity: 3,
            }),
            None,
        ));
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 2);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 1);
    }
    #[test]
    fn none_when_empty() {
        let res = remove_from_stack((
            Some(ItemInstance {
                item: &TORCH,
                quantity: 1,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
        ));
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 21);
        assert!(res.0.is_none());
    }
    #[test]
    fn edge_cases() {
        assert_was_swapped(
            (
                None,
                Some(ItemInstance {
                    item: &TORCH,
                    quantity: 100,
                }),
            ),
            combine_stack,
        );
        assert_was_swapped(
            (
                Some(ItemInstance {
                    item: &TORCH,
                    quantity: 1,
                }),
                None,
            ),
            combine_stack,
        );
        assert_was_swapped((None, SWORD_INST), combine_stack);
        assert_was_swapped((SWORD_INST, None), combine_stack);
        assert_was_swapped((TORCH_INST, JUNK_INST), combine_stack);
        assert_was_swapped((JUNK_INST, TORCH_INST), combine_stack);
    }
}
mod modified {
    use super::*;

    #[test]
    fn set() {
        let mut slot = <Slot<ItemInstance<'static>>>::new(None);
        assert_eq!(slot.modified, false);
        slot.set_item_instance(&TORCH_INST);
        assert_eq!(slot.modified, true);
    }
    #[test]
    fn swap() {
        let mut slot = <Slot<ItemInstance<'static>>>::new(None);
        assert_eq!(slot.modified, false);
        slot.transfer(TORCH_INST, "");
        assert_eq!(slot.modified, true);
    }
}
*/

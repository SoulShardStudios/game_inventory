mod items;
use inventory_rs::{
    combine_stack, half_stack_split, single_stack_split, swap, IItem, IItemInstance, ISlot,
    ItemInstance, Slot,
};
use items::{JUNK_INST, SWORD_INST, TORCH, TORCH_FULL_STACK_INST, TORCH_INST};

pub fn assert_was_swapped<'a>(
    _0: Option<ItemInstance<'a>>,
    _1: Option<ItemInstance<'a>>,
    method: fn(
        Option<ItemInstance<'a>>,
        Option<ItemInstance<'a>>,
    ) -> (Option<ItemInstance<'a>>, Option<ItemInstance<'a>>),
) {
    let res = method(_0, _1);
    match (_0, _1) {
        (Some(a), Some(b)) => {
            assert!(res.0.unwrap().quant() == b.quant());
            assert!(res.1.unwrap().quant() == a.quant());
            assert!(res.1.unwrap().item().name() == a.item().name());
            assert!(res.0.unwrap().item().name() == b.item().name());
        }
        (None, Some(b)) => {
            assert!(res.0.unwrap().quant() == b.quant());
            assert!(res.0.unwrap().item().name() == b.item().name());
        }
        (Some(a), None) => {
            assert!(res.1.unwrap().quant() == a.quant());
            assert!(res.1.unwrap().item().name() == a.item().name());
        }
        (None, None) => {}
    }
}

#[test]
fn test_swap() {
    assert_was_swapped(
        Some(ItemInstance {
            item: &TORCH,
            quantity: 10,
        }),
        Some(ItemInstance {
            item: &TORCH,
            quantity: 20,
        }),
        swap,
    );
    assert_was_swapped(
        Some(ItemInstance {
            item: &TORCH,
            quantity: 10,
        }),
        None,
        swap,
    );
    assert_was_swapped(
        None,
        Some(ItemInstance {
            item: &TORCH,
            quantity: 20,
        }),
        swap,
    );
}
mod combine {
    use super::*;

    #[test]
    fn simple() {
        let res = combine_stack(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
        );
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 40);
        assert!(res.1.is_none());
    }

    #[test]
    fn overflow() {
        let res = combine_stack(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 90,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
        );
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 100);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 10);
    }

    #[test]
    fn edge_cases() {
        assert_was_swapped(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 10,
            }),
            None,
            combine_stack,
        );
        assert_was_swapped(
            None,
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
            combine_stack,
        );
        assert_was_swapped(None, SWORD_INST, combine_stack);
        assert_was_swapped(SWORD_INST, None, combine_stack);
        assert_was_swapped(TORCH_INST, JUNK_INST, combine_stack);
        assert_was_swapped(JUNK_INST, TORCH_INST, combine_stack);
        assert_was_swapped(TORCH_FULL_STACK_INST, TORCH_INST, combine_stack);
        assert_was_swapped(TORCH_INST, TORCH_FULL_STACK_INST, combine_stack);
    }
}
mod split {
    use super::*;
    #[test]
    fn simple() {
        let res = half_stack_split(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 10,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 2,
            }),
        );
        println!("{:#?}", res);
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 5);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 7);
    }

    #[test]
    fn uneven() {
        let res = half_stack_split(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 11,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 3,
            }),
        );
        println!("{:#?}", res);
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 5);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 9);
    }

    #[test]
    fn none() {
        let res = half_stack_split(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 11,
            }),
            None,
        );
        println!("{:#?}", res);
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 5);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 6);
    }

    #[test]
    fn edge_cases() {
        assert_was_swapped(
            None,
            Some(ItemInstance {
                item: &TORCH,
                quantity: 34,
            }),
            combine_stack,
        );
        assert_was_swapped(JUNK_INST, None, combine_stack);
        assert_was_swapped(None, SWORD_INST, combine_stack);
        assert_was_swapped(SWORD_INST, None, combine_stack);
        assert_was_swapped(TORCH_INST, JUNK_INST, combine_stack);
        assert_was_swapped(JUNK_INST, TORCH_INST, combine_stack);
    }
}
mod single {
    use super::*;

    #[test]
    fn simple() {
        let res = single_stack_split(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 3,
            }),
        );
        println!("{:#?}", res);
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 21);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 2);
    }
    #[test]
    fn current_empty() {
        let res = single_stack_split(
            None,
            Some(ItemInstance {
                item: &TORCH,
                quantity: 3,
            }),
        );
        println!("{:#?}", res);
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 1);
        assert!(res.1.unwrap().item().name() == TORCH.name());
        assert!(res.1.unwrap().quant() == 2);
    }
    #[test]
    fn remove_at_end() {
        let res = single_stack_split(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 20,
            }),
            Some(ItemInstance {
                item: &TORCH,
                quantity: 1,
            }),
        );
        println!("{:#?}", res);
        assert!(res.0.unwrap().item().name() == TORCH.name());
        assert!(res.0.unwrap().quant() == 21);
        assert!(res.1.is_none());
    }
    #[test]
    fn edge_cases() {
        assert_was_swapped(
            None,
            Some(ItemInstance {
                item: &TORCH,
                quantity: 100,
            }),
            combine_stack,
        );
        assert_was_swapped(
            Some(ItemInstance {
                item: &TORCH,
                quantity: 1,
            }),
            None,
            combine_stack,
        );
        assert_was_swapped(None, SWORD_INST, combine_stack);
        assert_was_swapped(SWORD_INST, None, combine_stack);
        assert_was_swapped(TORCH_INST, JUNK_INST, combine_stack);
        assert_was_swapped(JUNK_INST, TORCH_INST, combine_stack);
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

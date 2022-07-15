use inventory_rs::{
    combine_stack, half_stack_split, single_stack_split, swap, IItem, IItemInstance, Item,
    ItemInstance,
};

fn assert_was_swapped<'a>(
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

const TORCH: Item = Item {
    name: "torch",
    max_quantity: 100,
};

const JUNK: Item = Item {
    name: "junk",
    max_quantity: 100,
};

const SWORD: Item = Item {
    name: "sword",
    max_quantity: 0,
};

const SWORD_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &SWORD,
    quantity: 0,
});

const JUNK_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &JUNK,
    quantity: 91,
});

const TORCH_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &TORCH,
    quantity: 23,
});

const TORCH_FULL_STACK_INST: Option<ItemInstance> = Some(ItemInstance {
    item: &TORCH,
    quantity: 100,
});

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
    fn test_simple() {
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
    fn test_overflow() {
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
    fn test_edge_cases() {
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
    fn test_simple() {
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
    fn test_uneven() {
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
    fn test_none() {
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
    fn test_edge_cases() {
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
    fn test_simple() {
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
    fn test_current_empty() {
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
    fn test_remove_at_end() {
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
    fn test_edge_cases() {
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

use crate::{combine_stack, traits};

pub fn inventory_contains_item<'a, II, S>(inventory: &Vec<S>, other: Option<II>) -> bool
where
    II: traits::IItemInstance<'a> + Copy,
    S: traits::ISlot<'a, II>,
{
    match other {
        Some(o) => inventory.iter().any(|s| match s.item_instance() {
            Some(i) => i.item().name() == o.item().name() && i.quant() == o.quant(),
            None => false,
        }),
        None => false,
    }
}

pub fn inventory_contains_item_type<'a, II, S>(inventory: &Vec<S>, name: &str) -> bool
where
    II: traits::IItemInstance<'a> + Copy,
    S: traits::ISlot<'a, II>,
{
    inventory.iter().any(|s| match s.item_instance() {
        Some(i) => i.item().name() == name,
        None => false,
    })
}

pub fn add_to_inventory<'a, II, S>(inventory: &mut Vec<S>, other: Option<II>) -> Option<II>
where
    II: traits::IItemInstance<'a> + Copy + 'a,
    S: traits::ISlot<'a, II>,
{
    if inventory.capacity() == 0 {
        return other;
    }
    match other {
        Some(o) => {
            if o.item().stackable() {
                if o.item().max_quant() == o.quant() {
                    return Some(o);
                }
                inventory
                    .iter_mut()
                    .fold(Some(o), |current, slot| match current {
                        Some(c) => match slot.item_instance() {
                            Some(s) => {
                                if s.item().name() != c.item().name() {
                                    return current;
                                }
                                if s.quant() == s.item().max_quant() {
                                    return current;
                                }
                                let res = combine_stack((slot.item_instance(), Some(c)));
                                slot.set_item_instance(&res.0);
                                return res.1;
                            }
                            None => None,
                        },
                        None => None,
                    });
            }
            match inventory
                .iter_mut()
                .find(|slot| slot.item_instance().is_none())
            {
                Some(s) => {
                    s.set_item_instance(&Some(o));
                    return None;
                }
                None => Some(o),
            }
        }
        None => None,
    }
}

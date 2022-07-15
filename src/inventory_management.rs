use crate::{combine_stack, traits};

pub fn inventory_contains_item<'a, II, S>(inventory: &Vec<S>, other: Option<II>) -> bool
where
    II: traits::IItemInstance<'a> + Copy + 'a,
    S: traits::ISlot<'a, II> + 'a,
{
    match other {
        Some(o) => inventory.iter().any(|s| match s.item_instance() {
            Some(i) => i.item().name() == o.item().name(),
            None => false,
        }),
        None => false,
    }
}

pub fn add_to_inventory<'a, II, S>(inventory: &mut Vec<S>, other: Option<II>) -> Option<II>
where
    II: traits::IItemInstance<'a> + Copy + 'a,
    S: traits::ISlot<'a, II> + 'a,
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
                inventory.iter_mut().fold(Some(o), |current, slot| {
                    let res = combine_stack(slot.item_instance(), current);
                    slot.set_item_instance(&res.0);
                    return res.1;
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

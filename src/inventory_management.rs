use crate::{
    slot_management::combine_stack,
    traits::{IInventory, IItem, IItemInstance, ISlot},
};

pub fn add_to_inventory<'a, II, S, Inv>(inventory: &mut Inv, other: Option<II>) -> Option<II>
where
    II: IItemInstance<'a> + Copy + 'a,
    S: ISlot<'a, II> + 'a,
    Inv: IInventory<'a, II, S>,
{
    if inventory.size() == 0 {
        return other;
    }
    match other {
        Some(o) => {
            if o.item().stackable() {
                if o.item().max_quant() == o.quant() {
                    return Some(o);
                }

                fn wrap_combine_for_iter<'a, II, S>(slot: &mut &'a mut S, other: &mut Option<II>)
                where
                    II: IItemInstance<'a> + Copy + 'a,
                    S: ISlot<'a, II>,
                {
                    let res = combine_stack(slot.get_item_instance(), *other);
                    slot.set_item_instance(&res.0);
                    *other = res.1;
                }

                let mut remaining = Some(o);
                inventory
                    .slots_mut()
                    .iter_mut()
                    .for_each(|slot| wrap_combine_for_iter(slot, &mut remaining));
                return remaining;
            }
            match inventory
                .slots_mut()
                .iter_mut()
                .find(|slot| slot.get_item_instance().is_none())
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

pub fn inventory_contains_item<'a, II, S, Inv>(inventory: &mut Inv, other: Option<II>) -> bool
where
    II: IItemInstance<'a> + Copy + 'a,
    S: ISlot<'a, II> + 'a,
    Inv: IInventory<'a, II, S> + 'a,
{
    match other {
        Some(o) => inventory
            .slots()
            .iter()
            .any(|s| match s.get_item_instance() {
                Some(i) => i.item().name() == o.item().name(),
                None => false,
            }),
        None => false,
    }
}

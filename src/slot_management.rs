use crate::data_types::{BaseItem, ItemInstance};

pub fn swap<I: BaseItem, II: ItemInstance<I>>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>) {
    (other, current)
}

pub fn combine_stack<I: BaseItem, II: ItemInstance<I>>(
    mut current: Option<II>,
    mut other: Option<II>,
) -> (Option<II>, Option<II>) {
    if current.is_none() || other.is_none() {
        return swap(current, other);
    }
    let u_current = current.as_mut().unwrap();
    let u_other = other.as_mut().unwrap();
    if u_current.get_item().name() != u_other.get_item().name() {
        return swap(current, other);
    }
    let item = u_current.get_item();
    if !item.is_stackable() {
        return swap(current, other);
    }
    let stack_size = item.max_stack_quantity();
    let current_quant = u_current.get_quantity();
    let other_quant = u_other.get_quantity();
    if current_quant >= stack_size || other_quant >= stack_size {
        return swap(current, other);
    }

    if current_quant + other_quant < stack_size {
        return (Some(II::new(item, current_quant + other_quant)), None);
    }
    let left_over = current_quant + other_quant - stack_size;
    return (
        Some(II::new(item, stack_size)),
        Some(II::new(item, left_over)),
    );
}

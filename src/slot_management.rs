use crate::data_types::{IItem, IItemInstance};

pub fn swap<I: IItem, II: IItemInstance<I>>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>) {
    (other, current)
}

pub fn combine_stack<I: IItem, II: IItemInstance<I> + Copy>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>)
where
    I: 'static,
{
    return match (current, other) {
        (Some(c), Some(o)) => combine_stack_some(&c, &o),
        (Some(c), None) => swap(Some(c), None),
        (None, Some(o)) => swap(None, Some(o)),
        (None, None) => swap(None, None),
    };
}

fn combine_stack_some<I: IItem, II: IItemInstance<I> + Copy>(
    current: &II,
    other: &II,
) -> (Option<II>, Option<II>)
where
    I: 'static,
{
    if current.get_item().name() != other.get_item().name() {
        return swap(Some(*current), Some(*other));
    }
    if !current.get_item().is_stackable() {
        return swap(Some(*current), Some(*other));
    }
    let stack_size = current.get_item().max_stack_quantity();
    let current_quant = current.get_quantity();
    let other_quant = other.get_quantity();
    if current_quant >= stack_size || other_quant >= stack_size {
        return swap(Some(*current), Some(*other));
    }

    if current_quant + other_quant < stack_size {
        return (
            Some(II::new(current.get_item(), current_quant + other_quant)),
            None,
        );
    }
    let left_over = current_quant + other_quant - stack_size;
    return (
        Some(II::new(current.get_item(), stack_size)),
        Some(II::new(current.get_item(), left_over)),
    );
}

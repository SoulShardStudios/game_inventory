use crate::data_types::{IItem, IItemInstance};

pub fn swap<'a, I: IItem, II: IItemInstance<'a, I>>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>) {
    (other, current)
}

pub fn combine_stack<'a, I: IItem, II: IItemInstance<'a, I> + Copy>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>)
where
    I: 'a,
    II: 'a,
{
    return match (current, other) {
        (Some(c), Some(o)) => {
            if c.item().name() != o.item().name() {
                return swap(Some(c), Some(o));
            }
            if !c.item().stackable() {
                return swap(Some(c), Some(o));
            }
            let stack_size = c.item().max_stack_quantity();
            let current_quant = c.quant();
            let other_quant = o.quant();
            if current_quant >= stack_size || other_quant >= stack_size {
                return swap(Some(c), Some(o));
            }

            if current_quant + other_quant < stack_size {
                return (Some(II::new(c.item(), current_quant + other_quant)), None);
            }
            let left_over = current_quant + other_quant - stack_size;
            return (
                Some(II::new(c.item(), stack_size)),
                Some(II::new(c.item(), left_over)),
            );
        }
        (Some(c), None) => swap(Some(c), None),
        (None, Some(o)) => swap(None, Some(o)),
        (None, None) => swap(None, None),
    };
}

pub fn half_stack_split<'a, I: IItem, II: IItemInstance<'a, I> + Copy>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>)
where
    I: 'a,
    II: 'a,
{
    return match current {
        Some(c) => match other {
            Some(_o) => swap(current, other),
            None => {
                if !c.item().stackable() {
                    return swap(current, other);
                }
                if c.quant() < 2 {
                    return swap(current, other);
                }
                let half_stack = c.quant() / 2;
                return (
                    Some(II::new(c.item(), half_stack)),
                    Some(II::new(c.item(), half_stack + (c.quant() % 2))),
                );
            }
        },
        None => swap(current, other),
    };
}

pub fn single_stack_split<'a, I: IItem, II: IItemInstance<'a, I> + Copy>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>)
where
    I: 'a,
    II: 'a,
{
    return match other {
        Some(o) => {
            if !o.item().stackable() {
                return swap(current, other);
            }
            match current {
                Some(c) => {
                    if c.item().name() != o.item().name() {
                        return swap(current, other);
                    }
                    if c.quant() == c.item().max_stack_quantity() {
                        return swap(current, other);
                    }
                    if o.quant() < 2 {
                        return (Some(II::new(o.item(), c.quant() + 1)), None);
                    }
                    return (
                        Some(II::new(o.item(), c.quant() + 1)),
                        Some(II::new(o.item(), o.quant() - 1)),
                    );
                }
                None => {
                    if o.quant() < 2 {
                        return (Some(II::new(o.item(), 1)), None);
                    }
                    return (
                        Some(II::new(o.item(), 1)),
                        Some(II::new(o.item(), o.quant() - 1)),
                    );
                }
            }
        }
        None => swap(current, other),
    };
}

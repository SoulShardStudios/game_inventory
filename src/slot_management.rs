use crate::traits;

pub fn swap<'a, II: traits::IItemInstance<'a>>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>) {
    (other, current)
}

pub fn combine_stack<'a, II>(current: Option<II>, other: Option<II>) -> (Option<II>, Option<II>)
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    return match (current, other) {
        (Some(c), Some(o)) => {
            if c.item().name() != o.item().name() {
                return swap(Some(c), Some(o));
            }
            if !c.item().stackable() {
                return swap(Some(c), Some(o));
            }
            let stack_size = c.item().max_quant();
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

pub fn half_stack_split<'a, II>(current: Option<II>, other: Option<II>) -> (Option<II>, Option<II>)
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    return match current {
        Some(c) => {
            if !c.item().stackable() {
                return swap(current, other);
            }
            if match other {
                Some(o) => c.item().name() != o.item().name(),
                None => false,
            } {
                return swap(current, other);
            }
            if c.quant() < 2 {
                return swap(current, other);
            }
            let other_quant = match other {
                Some(o) => o.quant(),
                None => 0,
            };

            let half_stack = c.quant() / 2;
            return (
                Some(II::new(c.item(), half_stack)),
                Some(II::new(
                    c.item(),
                    other_quant + half_stack + (c.quant() % 2),
                )),
            );
        }
        None => swap(current, other),
    };
}

pub fn single_stack_split<'a, II>(
    current: Option<II>,
    other: Option<II>,
) -> (Option<II>, Option<II>)
where
    II: traits::IItemInstance<'a> + Copy + 'a,
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
                    if c.quant() == c.item().max_quant() {
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

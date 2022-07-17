use crate::traits;
/// Returns the inverse of the two inputs, specifically `(items.1, items.0)`.
pub fn swap<'a, II: traits::IItemInstance<'a>>(
    items: (Option<II>, Option<II>),
) -> (Option<II>, Option<II>) {
    (items.1, items.0)
}
/// Combines two stacks of items. Tries to put `items.0` into `items.1`.
///
/// Accounts for unstackable items, item overflowing, and many other edge cases.
/// See the tests in `tests/slot_management.rs` for specific behavior.
pub fn combine_stack<'a, II>(items: (Option<II>, Option<II>)) -> (Option<II>, Option<II>)
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    return match items {
        (Some(c), Some(o)) => {
            if c.item().name() != o.item().name() {
                return swap(items);
            }
            if !c.item().stackable() {
                return swap(items);
            }
            let stack_size = c.item().max_quant();
            if c.quant() >= stack_size || o.quant() >= stack_size {
                return swap(items);
            }
            let combined = c.quant() + o.quant();
            if combined < stack_size {
                return (None, Some(II::new(c.item(), combined)));
            }
            let left_over = combined - stack_size;
            return (
                Some(II::new(c.item(), left_over)),
                Some(II::new(c.item(), stack_size)),
            );
        }
        _ => swap(items),
    };
}

/// Splits a stack of items into two. Tries to split `items.0` and put the second half into `items.1`
///
/// Accounts for unstackable items, item overflowing, and many other edge cases.
/// See the tests in `tests/slot_management.rs` for specific behavior.
pub fn half_stack_split<'a, II>(items: (Option<II>, Option<II>)) -> (Option<II>, Option<II>)
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    return match items.0 {
        Some(c) => {
            if !c.item().stackable() {
                return swap(items);
            }
            if match items.1 {
                Some(o) => c.item().name() != o.item().name(),
                None => false,
            } {
                return swap(items);
            }
            if c.quant() < 2 {
                return swap(items);
            }
            let other_quant = match items.1 {
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
        None => swap(items),
    };
}

/// Removes a single item from a stack. Tries to take a single item from `items.0` and put it into `items.1`.
///
/// Accounts for unstackable items, item overflowing, and many other edge cases.
/// See the tests in `tests/slot_management.rs` for specific behavior.
pub fn remove_from_stack<'a, II>(items: (Option<II>, Option<II>)) -> (Option<II>, Option<II>)
where
    II: traits::IItemInstance<'a> + Copy + 'a,
{
    return match items.0 {
        Some(c) => {
            if !c.item().stackable() {
                return swap(items);
            }
            match items.1 {
                Some(o) => {
                    if o.item().name() != c.item().name() {
                        return swap(items);
                    }
                    if o.quant() == o.item().max_quant() {
                        return swap(items);
                    }
                    if c.quant() < 2 {
                        return (None, Some(II::new(o.item(), o.quant() + 1)));
                    }
                    return (
                        Some(II::new(c.item(), c.quant() - 1)),
                        Some(II::new(o.item(), o.quant() + 1)),
                    );
                }
                None => {
                    if c.quant() < 2 {
                        return (None, Some(II::new(c.item(), 1)));
                    }
                    return (
                        Some(II::new(c.item(), c.quant() - 1)),
                        Some(II::new(c.item(), 1)),
                    );
                }
            }
        }
        None => swap(items),
    };
}

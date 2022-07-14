pub trait BaseItem {
    fn is_stackable(&self) -> bool;
    fn max_stack_amount(&self) -> u16;
    fn name(&self) -> &String;
}

pub trait ItemInstance<Item: BaseItem> {
    fn get_quantity(&self) -> u16;
    fn get_item(&self) -> &Item;
}

pub trait Inventory<'a, I: BaseItem, II: ItemInstance<I>> {
    fn size(&self) -> usize;
    fn get_items(&self) -> &[Option<II>];
    fn get_items_mut(&mut self) -> &mut [Option<II>];
}

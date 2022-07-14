pub trait BaseItem {
    fn is_stackable(&self) -> bool;
    fn max_stack_amount(&self) -> u16;
    fn name(&self) -> &String;
}

pub trait ItemInstance<Item: BaseItem> {
    fn get_quantity(&self) -> u16;
    fn get_item(&self) -> &Item;
}

pub trait Slot<'a, I: BaseItem, II: ItemInstance<I>> {
    fn get_item_instance(&self) -> Option<II>;
    fn set_item_instance(&self, item_instance: Option<II>);
    fn transfer(&self, item_instance: Option<II>) -> Option<II> {
        let original = self.get_item_instance();
        self.set_item_instance(item_instance);
        *original
    }
}

pub trait Inventory<'a, I: BaseItem, II: ItemInstance<I>, S: Slot<'a, I, II>> {
    fn size(&self) -> usize;
    fn get_items(&self) -> &[S];
    fn get_items_mut(&mut self) -> &mut [S];
}

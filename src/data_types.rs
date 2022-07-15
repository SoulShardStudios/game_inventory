use crate::slot_management::swap;

pub trait IItem {
    fn stackable(&self) -> bool;
    fn max_quant(&self) -> u16;
    fn name(&self) -> &str;
}

pub trait IItemInstance<'a, Item: IItem> {
    fn quant(&self) -> u16;
    fn item(&self) -> &'a Item;
    fn new(item: &'a Item, quantity: u16) -> Self;
}

pub trait ISlot<'a, I: IItem, II: IItemInstance<'a, I>> {
    fn get_item_instance(&self) -> Option<II>;
    fn set_item_instance(&mut self, item_instance: Option<II>);
    fn transfer(&mut self, item_instance: Option<II>) -> Option<II> {
        let res = swap(self.get_item_instance(), item_instance);
        self.set_item_instance(res.0);
        res.1
    }

    fn set_change_callback(&mut self, callback: Option<fn(Option<II>)>);
}

pub trait IInventory<'a, I: IItem, II: IItemInstance<'a, I>, S: ISlot<'a, I, II>> {
    fn size(&self) -> usize;
    fn slots(&self) -> &[S];
    fn slots_mut(&mut self) -> &mut [S];
}

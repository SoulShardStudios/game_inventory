use crate::slot_management::swap;

pub trait IItem {
    fn stackable(&self) -> bool;
    fn max_quant(&self) -> u16;
    fn name(&self) -> &str;
}
pub trait IDebugItem: IItem + core::fmt::Debug {}

pub trait IItemInstance<'a> {
    fn quant(&self) -> u16;
    fn item(&self) -> &'a dyn IDebugItem;
    fn new(item: &'a dyn IDebugItem, quantity: u16) -> Self;
}

pub trait ISlot<'a, II: IItemInstance<'a> + Sized> {
    fn item_instance(&self) -> Option<II>;
    fn set_item_instance(&mut self, item_instance: &Option<II>);
    fn transfer(&mut self, item_instance: Option<II>) -> Option<II> {
        let res = swap(self.item_instance(), item_instance);
        self.set_item_instance(&res.0);
        res.1
    }

    fn set_change_callback(&mut self, callback: &'a Option<fn(Option<II>)>);
}

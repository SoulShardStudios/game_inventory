use game_inventory::samples::{DefaultItem, DefaultItemInstance, DefaultSlot, TORCH_INST};
use game_inventory::traits::Slot;

#[test]
fn set() {
    let mut slot = DefaultSlot::new(None);
    assert_eq!(slot.modified, false);
    slot.set_item_instance(&TORCH_INST);
    assert_eq!(slot.modified, true);
}
#[test]
fn swap() {
    let mut slot = <DefaultSlot<
        'static,
        DefaultItem<'static>,
        DefaultItemInstance<DefaultItem<'static>>,
    >>::new(None);
    assert_eq!(slot.modified, false);
    slot.transfer(TORCH_INST.clone(), "");
    assert_eq!(slot.modified, true);
}

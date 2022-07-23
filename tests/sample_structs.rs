use game_inventory::samples::{ItemInstance, Slot, TORCH_INST};
use game_inventory::traits::ISlot;
mod modified {
    use super::*;

    #[test]
    fn set() {
        let mut slot = <Slot<ItemInstance<'static>>>::new(None);
        assert_eq!(slot.modified, false);
        slot.set_item_instance(&TORCH_INST);
        assert_eq!(slot.modified, true);
    }
    #[test]
    fn swap() {
        let mut slot = <Slot<ItemInstance<'static>>>::new(None);
        assert_eq!(slot.modified, false);
        slot.transfer(TORCH_INST, "");
        assert_eq!(slot.modified, true);
    }
}

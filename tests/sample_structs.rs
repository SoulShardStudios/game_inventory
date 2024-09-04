use game_inventory::samples::{DefaultItemInstance, DefaultSlot, TORCH_INST};
use game_inventory::traits::Slot;
mod modified {
    use super::*;

    #[test]
    fn set() {
        let mut slot = <DefaultSlot<DefaultItemInstance<'static>>>::new(None);
        assert_eq!(slot.modified, false);
        slot.set_item_instance(&TORCH_INST);
        assert_eq!(slot.modified, true);
    }
    #[test]
    fn swap() {
        let mut slot = <DefaultSlot<DefaultItemInstance<'static>>>::new(None);
        assert_eq!(slot.modified, false);
        slot.transfer(TORCH_INST, "");
        assert_eq!(slot.modified, true);
    }
}

/*
namespace SoulShard.InventorySystem
{
    public struct InventoryManagementUtilities
    {
        public static _ItemInstance AddUnstackableItemToInventory<_BaseItem, _Slot, _ItemInstance, _Inventory>(
            _Inventory inventory,
            _ItemInstance other
        )
            where _BaseItem : class, IBaseItem
            where _ItemInstance : struct, IItemInstance<_BaseItem>
            where _Slot : class, ISlot<_BaseItem, _ItemInstance>, new()
            where _Inventory: class, IInventory<_BaseItem,_Slot,_ItemInstance>
        {
            if (other.isEmpty)
                return other;
            if (inventory.capacity == 0)
                return other;

            for (int i = 0; i < inventory.slots.Length; i++)
            {
                if (inventory.slots[i].isEmpty)
                {
                    inventory.slots[i].itemInstance = other;
                    return new _ItemInstance();
                }
            }
            return other;
        }

        public static _ItemInstance AddStackableItemToInventory<_BaseItem, _Slot, _ItemInstance, _Inventory>(
            _Inventory inventory,
            _ItemInstance other
        )
            where _BaseItem : class, IBaseItem
            where _ItemInstance : struct, IItemInstance<_BaseItem>
            where _Slot : class, ISlot<_BaseItem, _ItemInstance>, new()
            where _Inventory : class, IInventory<_BaseItem, _Slot, _ItemInstance>
        {
            if (other.isEmpty)
                return other;
            if (inventory.capacity == 0)
                return other;

            uint maxStack = other.item.maxStackAmount;

            if (other.amount == maxStack)
                return AddUnstackableItemToInventory<_BaseItem,_Slot,_ItemInstance,_Inventory>(inventory, other);

            for (int i = 0; i < inventory.slots.Length; i++)
            {
                if (inventory.slots[i].itemInstance.isEmpty)
                    continue;
                if (inventory.slots[i].itemInstance.item.name == other.item.name)
                {
                    if (inventory.slots[i].itemInstance.amount == maxStack)
                        continue;
                    if (inventory.slots[i].itemInstance.amount + other.amount < maxStack)
                    {
                        _ItemInstance newItem = inventory.slots[i].itemInstance;
                        newItem.amount += other.amount;
                        inventory.slots[i].itemInstance = newItem;
                        return new _ItemInstance();
                    }

                    other.amount -= maxStack - inventory.slots[i].itemInstance.amount;
                    _ItemInstance newItem2 = inventory.slots[i].itemInstance;
                    newItem2.amount = maxStack;
                    inventory.slots[i].itemInstance = newItem2;
                }
            }

            if (other.amount > 0)
                return AddUnstackableItemToInventory<_BaseItem, _Slot, _ItemInstance, _Inventory>(inventory, other);

            return other;
        }

        public static _ItemInstance AddItemToInventory<_BaseItem, _Slot, _ItemInstance, _Inventory>(
            _Inventory inventory,
            _ItemInstance other
        )
            where _BaseItem : class, IBaseItem
            where _ItemInstance : struct, IItemInstance<_BaseItem>
            where _Slot : class, ISlot<_BaseItem, _ItemInstance>, new()
            where _Inventory : class, IInventory<_BaseItem, _Slot, _ItemInstance>
        {
            if (other.isEmpty)
                return other;
            if (inventory.capacity == 0)
                return other;

            if (!other.item.isStackable)
                return AddUnstackableItemToInventory<_BaseItem, _Slot, _ItemInstance, _Inventory>(inventory, other);
            return AddStackableItemToInventory<_BaseItem, _Slot, _ItemInstance, _Inventory>(inventory, other);
        }

        // checks if the inventory contains a specific item
        public static bool ContainsItem<_BaseItem, _Slot, _ItemInstance, _Inventory>(
            _Inventory inventory,
            _ItemInstance other
        )
            where _BaseItem : class, IBaseItem
            where _ItemInstance : struct, IItemInstance<_BaseItem>
            where _Slot : class, ISlot<_BaseItem, _ItemInstance>, new()
            where _Inventory : class, IInventory<_BaseItem, _Slot, _ItemInstance>
        {
            if (other.isEmpty)
                return false;
            if (inventory.capacity == 0)
                return false;
            foreach (_Slot s in inventory.slots)
                if (!s.isEmpty)
                    if (s.itemInstance.item.name == other.item.name)
                        if (s.itemInstance.item.isStackable)
                        {
                            if (s.itemInstance.amount == other.amount)
                                return true;
                        }
                        else
                            return true;

            return false;
        }
    }
}

*/

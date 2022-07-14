#[derive(Debug)]
struct Item {
    name: String,
    max_stack_amount: u16,
}

trait BaseItem {
    fn is_stackable(&self) -> bool;
    fn max_stack_amount(&self) -> u16;
    fn name(&self) -> &String;
}

impl BaseItem for Item {
    fn is_stackable(&self) -> bool {
        true
    }

    fn max_stack_amount(&self) -> u16 {
        self.max_stack_amount
    }

    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
struct IItem<'a> {
    item: &'a Item,
    quantity: u16,
}

trait ItemInstance<Item: BaseItem> {
    fn get_quantity(&self) -> u16;
    fn get_item(&self) -> &Item;
}

impl<'a> ItemInstance<Item> for IItem<'a> {
    fn get_quantity(&self) -> u16 {
        self.quantity
    }

    fn get_item(&self) -> &Item {
        self.item
    }
}

#[derive(Debug)]
struct BasicInventory<'a> {
    items: Vec<Option<IItem<'a>>>,
}

impl<'a> Inventory<'a, Item, IItem<'a>> for BasicInventory<'a> {
    fn size(&self) -> usize {
        self.items.capacity()
    }

    fn get_items(&self) -> &[Option<IItem<'a>>] {
        &self.items
    }

    fn get_items_mut(&mut self) -> &mut [Option<IItem<'a>>] {
        &mut self.items
    }
}

trait Inventory<'a, I: BaseItem, II: ItemInstance<I>> {
    fn size(&self) -> usize;
    fn get_items(&self) -> &[Option<IItem<'a>>];
    fn get_items_mut(&mut self) -> &mut [Option<IItem<'a>>];
}

fn main() {
    let test_item = Item {
        name: "brocoli".to_string(),
        max_stack_amount: 100,
    };

    let inv = BasicInventory {
        items: vec![Some(IItem {
            item: &test_item,
            quantity: 10,
        })],
    };

    println!("{:#?}", test_item);

    println!("{:#?}", inv);
}

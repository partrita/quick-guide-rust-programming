use std::collections::HashMap;

struct Item {
    name: String,
    price: f32,
    quantity: u32,
}

struct Storage {
    items: HashMap<String, Item>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            items: HashMap::new(),
        }
    }

    fn storage_store(&mut self, item: Item) {
        self.items.insert(item.name.clone(), item);
    }
}

struct Statistics<'a> {
    items: &'a HashMap<String, Item>,
}

impl<'a> Statistics<'a> {
    fn new(items: &'a HashMap<String, Item>) -> Self {
        Statistics { items }
    }

    fn get_average(&self) -> f32 {
        let total = self.items.values().fold(0.0, |acc, i| acc + i.price);
        let count = self.items.values().fold(0, |acc, i| acc + i.quantity);
        total / count as f32
    }
}

fn main() {
    let mut mystorage = Storage::new();

    let apple = Item {
        name: "apple".to_string(),
        price: 1.0,
        quantity: 10,
    };
    mystorage.storage_store(apple);

    let banana = Item {
        name: "banana".to_string(),
        price: 2.0,
        quantity: 20,
    };
    mystorage.storage_store(banana);

    let stats = Statistics::new(&mystorage.items);
    println!("{}", stats.get_average());
}

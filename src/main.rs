use scattered_rs::Store;

pub mod kvstore {
    tonic::include_proto!("kvstore");
}

struct Node {
    store: Store,
}

impl Node {
    fn new() -> Self {
        Node {
            store: Store::new(),
        }
    }

    fn put(&mut self, key: String, value: String) {
        self.store.put(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        self.store.get(key)
    }
}

fn main() {
    let mut node = Node::new();

    println!("Node initialized");

    node.put("key".to_string(), "value".to_string());

    if let Some(value) = node.get("key") {
        println!("Retrieved value: {} using: {}", value, "key");
    } else {
        println!("Key not found");
    }
}

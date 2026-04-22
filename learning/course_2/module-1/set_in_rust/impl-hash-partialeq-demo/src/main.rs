
use std::{collections::HashMap, hash::{Hash, Hasher}};

#[derive(Debug, PartialEq, Eq, Hash)]
struct UserKey {
    id: u32,
    username: String,
}


struct Tag {
    name: String,
    timestamp: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Tag 
{

}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.timestamp.hash(state);
    }
}


fn main() {
    let mut users: HashMap<UserKey, String> = HashMap::new();

    let key = UserKey {
        id: 1,
        username: String::from("alice"),
    };

    users.insert(key, String::from("Admin"));

    let lookup = UserKey {
        id: 1,
        username: String::from("alice"),
    };

    println!("{:?}", users.get(&lookup)); // Some("Admin")
}

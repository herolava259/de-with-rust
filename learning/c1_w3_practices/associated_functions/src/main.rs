struct User {
    username: String,
    uri: String,
    email: String,
    active: bool
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true, 
        }
    }

    fn deactivate(&mut self){
        self.active = false
    }

    fn from_email(&self, email: String) -> Self {
        let username = email
                                        .split('@')
                                        .next()
                                        .unwrap_or("")
                                        .to_string();

        Self{
            username,
            email,
            uri: self.uri.clone(),
            active: true
        }
    }

    fn update_uri(&mut self, uri: String)
    {
        self.uri = uri.clone();
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("farrer"),
        String::from("farrerle@example.com"),
        String::from("https://farrer.le.com"),
    );

    println!("Username: {}", new_user.username);
    println!("Email: {}", new_user.email);
    println!("URI: {}", new_user.uri);
    new_user.deactivate();
    println!("Active: {}", new_user.active);
    let another_user = new_user.from_email(String::from("tungld"));
    println!("Another User - Username: {}, Email: {}, URI: {}, Active: {}",
        another_user.username, another_user.email, another_user.uri, another_user.active);
    new_user.update_uri(String::from("https://choppermon999.com"));
    println!("Updated URI: {}", new_user.uri);
}

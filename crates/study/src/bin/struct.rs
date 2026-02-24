struct User {
    username: String,
    login_count: u32,
}

impl User {
    fn new(username: String) -> User {
        return User {
            username,
            login_count: 0,
        };
    }

    fn login(&mut self) {
        self.login_count += 1
    }

    fn display(&self) {
        println!("name={}, login_count={}", self.username, self.login_count)
    }
}

fn main() {
    let name = String::from("yaeger");
    let mut user1 = User::new(name);

    user1.display();
    user1.login();
    user1.display();
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//authentication module provides the User struct with the User::new method, since they're both public
pub struct User {
    username: String,
    password_hash: u64,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(&password.to_owned()),
        }
    }

    // If we wanted to give read access to the username field and write access to the password field while keeping them private, we could use getter and setter methods If we wanted to give read access to the username field and write access to the password field while keeping them private, we could use getter and setter methods
    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(&new_password.to_owned())
    }
}
fn hash_password<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// pub keyword makes it public

pub struct User { // creating a User struct with 4 attributes
    pub age: u16,
    pub active: bool,
    pub email: String,
    pub username: String,
}

pub fn run() -> User {
    let hello = "Hello from structs.rs!";
    println!("\n{}\n{}", hello, "-".repeat(hello.len()));

    let user = User { // Creating a user from the User struct
        age: 15,
        active: true,
        email: String::from("someone@email.com"),
        username: String::from("Get good")
    };
    println!("Email: {}\nAge: {}\nActive: {}\nUsername: {}", user.email, user.age, user.active, user.username);
    return user; // return user
}

pub struct Rect { // Rectangle struct
    pub width: u16,
    pub height: u16
}
struct User {
    userid: u64,
    active: bool,
    username: String,
    email: String,
    contact: u64,
}

fn main() {
    let mut user1 = User {
        userid: 123,
        active: true,
        username: String::from("Khush05"),
        email: String::from("Khush05@gmail.com"),
        contact: 123456789,
    };

    user1.username= String::from("Diyu05");

    println!("{}", user1.contact);
    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.userid);
}

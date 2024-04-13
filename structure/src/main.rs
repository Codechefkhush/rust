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

    //  user1.username= String::from("Diyu05");

    println!("{}", user1.contact);
    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.userid);

    //Using struct update syntax
    let mut user2 = User {
        email: String::from("Diyu05@gmail.com"),
        ..user1
    };

    println!("{}", user2.contact);
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.userid);
}

//Returning User instance with function implementation
// fn build_user (userid: u64, username: String, email: String, contact: u64 ) -> User {
//     User {
//         userid, //field init shorthand property
//         active: true,
//         username,
//         email,
//         contact,
//     };
// }

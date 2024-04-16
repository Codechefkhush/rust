//Calculating area of rectangle

//Using functions

// fn main() {
//     let width = 10;
//     let height = 20;

//     println!("The area of rectangle is {}", area(width, height));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//Using tuples

// fn main() {
//     let rect = (30, 50);

//     println!("The area of rectangle is {}", area(rect));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//Using struct

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("The area of rectangle is {}", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//Variables and Mutability

//mutability

// fn main() {
//     let mut x = 5 ;
//     println!("Number is {x}");
//     x= 4;
//     println!("Number is {x}");
// }

//shadowing

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

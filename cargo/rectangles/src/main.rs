// area of a rectangle in a not clear manner
// fn main() {
//     let width = 30;
//     let height = 50;

//     println!("The area of the rectangle is {}", area(width, height));
// }


// fn area(width: u32, height: u32) -> u32{
//     width * height
// }

// area of a rectangle using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rect = Rectangle{
        width: 30,
        height: 50
    };

    println!("rectangle is {rect:?}");

    println!("The area of the rectangle is {}", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


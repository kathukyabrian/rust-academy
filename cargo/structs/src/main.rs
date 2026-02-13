#[derive(Debug)]
struct Person{
    name: String,
    age: u32,
    email: String,
}

impl Person{
    fn age(&self) -> u32{
        self.age
    }
}

fn main() {
    let jam = Person{
        name: String::from("Jamila"),
        age: 25,
        email: String::from("jamilabarasa18@gmail.com"),
    };

    println!("My age is {}", jam.age());

    println!("This person is {jam:?}");
}

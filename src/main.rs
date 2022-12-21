fn main() {
    println!("Hello, world! {}", add(2, 2));

    // Example with shadowing
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;
    println!("The value of shadow_num is: {}", shadow_num);

    #[derive(Debug)]
    enum Book {
        Paperback(bool),
    }

    let book = Book::Paperback(true);
    println!("Book: {:#?}", book);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

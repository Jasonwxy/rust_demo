mod demo;
use std::fs::File;
use std::io::{ErrorKind, Write};


#[derive(Debug)]
struct Person {
    name: String,
    gender: String,
    address: String,
    age: u16,
}

struct Color(u8, u8, u8);
struct Point(f64, f64);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }

    fn higher(&self, rect: &Rectangle) -> bool {
        self.height > rect.height
    }

    fn create(width:u32,height:u32) -> Rectangle{
        Rectangle{width,height}
    }
}

#[derive(Debug)]
enum Book {
    Papery{index:u32},
    Electronic{url:String}    
}

fn main() {
    greet_world();
    loop_demo();
    demo();
    borrow_demo();
    slice_demo();
    struct_demo();
    tuple_struct_demo();
    struct_function();
    enum_demo();
    demo::show_text();
    string_demo();
    result_demo();
}


fn result_demo(){
    let f = File::open("file.txt");

    let _ = match f{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt"){
                Ok(mut fc) => {
                    fc.write_all(b"this is a new").expect("can not write file");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),  
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),            
        },
    };

}


fn string_demo(){
    let mut s = String::from("hello world");
    let word = first_word(&mut s);
    println!("The first world of s is {}",word);
    s.clear();
}

fn first_word(s: &String) -> &str {
    &s[..1]    
}


fn greet_world(){
    let chinese = "世界，你好";
    let english = "hello world";
    let regions = [chinese,english];
    for region in regions{
        println!("{}",&region);
    }
}

fn enum_demo(){
    let book1 = Book::Papery { index: 333 };
    let book2 = Book::Electronic { url: String::from("http://www.123.com/") };
    println!("book1 is {:#?}",book1);
    println!("book2 is {:#?}",book2);
    match_book(book1);
    match_book(book2);
}

fn match_book(book: Book) {
    match book {
        Book::Papery { index } => {
            println!("Papery book {}",index);
        },
        Book::Electronic { url } => {
            println!("E-book {}",url);
        }
    }
}

fn struct_function() {
    let rect1 = Rectangle {
        width: 40,
        height: 50,
    };
    println!("rect1's area is {}", rect1.area());
    let rect2 = Rectangle {
        width: 30,
        height: 70,
    };
    println!("rect2's area is {}",rect2.area());
    println!("{},{}",rect1.wider(&rect2),rect1.higher(&rect2));
    let rect3 = Rectangle::create(50, 60);
    println!("rect3 is {:#?}",rect3)
}

fn tuple_struct_demo() {
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("black is ({},{},{})", black.0, black.1, black.2);
    println!("origin is ({},{})", origin.0, origin.1);
}

fn struct_demo() {
    let p1 = Person {
        name: String::from("zs"),
        gender: String::from("male"),
        address: String::from("BeiJing"),
        age: 22,
    };
    let p2 = Person {
        name: String::from("LiSi"),
        ..p1
    };
    println!(
        "p2 name is {},gender is {},address is {},age is {}",
        p2.name, p2.gender, p2.address, p2.age
    );
}

fn borrow_demo() {
    let x = 5;
    let y = x;
    println!("x = {},y = {}", x, y);
    let x = String::from("aaa");
    let y = x.clone();
    println!("x = {},y = {}", x, y);
    println!("The length of {} is {}", x, get_str_len(&x));
    let mut a = String::from("aaa");
    let b = &mut a;
    b.push_str("abcd");
    println!("b is {}", b);
}

fn slice_demo() {
    let s = String::from("abcdefghijklmnopqrstuvwxyz");
    let p1 = &s[0..10];
    let p2 = &s[10..20];
    println!("p1 is {},p2 is {}", p1, p2);
}

fn get_str_len(s: &String) -> usize {
    s.len()
}

fn demo() {
    let x = 3;
    let y = {
        let x = 5;
        x + 1
    };
    println!("x 的值为{}", x);
    println!("y 的值为{}", y);
    println!("x+y 的值为{}", add(x, y))
}

fn add(x: u8, y: u8) -> u8 {
    x + y
}

fn loop_demo() {
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("值为：{}", i)
    }
    for i in 0..5 {
        println!("a[{}]的值为：{}", i, a[i])
    }
}

fn main() {
    let x: i32 = 5;
    let y: i32 = 10;
    println!("Hello, world!");
    println!("The value of x is: {} {}", x,y);
    // unsigned integer
    let a: u32 = 5;

    //signed integer
    let b: i32 = -5;

    //floating point
    let c: f32 = 5.0;

    println!("The value of a is unsign integer: {}", a);

    println!("The value of b is sign integer: {}", b);
    println!("The value of c is floating integer: {}", c);
    // char
    let letter = 'a';
    let emoji = "\u{1F600}";
    println!("The value of letter is: {}", letter);
    println!("The value of emoji is: {}", emoji);

    // boolean
    let is_rust = true;
    println!("The value of is_rust is: {}", is_rust);
    // array
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let other:[u8;5] =[100;5];
    println!("The value of arr is: {:?}", arr);
    println!("index:{},length:{}",arr[0],other.len());
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup1 = (500, 1);

    println!("The value of tup is: {:?}", tup);
    println!("The value of tup1 is: {:?}", tup1);
    // mut to change to mutable

//arr  vs slice :
// let slice = &arr[1..3]; // [2, 3] don't need to specify the length
// println!("The value of slice is: {:?}", slice);  
    let slice = &arr[1..3]; // [1, 2, 3]
    borrowing_slice(arr, slice);

// String
    let s = String::from("hello");
    // get 0-4 not include 4
    let slice = &s[0..4];
    slice.len();
}
fn borrowing_slice(arr: [u8; 5], slice: &[u8]) {
    println!("The value of arr is: {:?}", arr);
    println!("The value of slice is: {:?}", slice);
    println!("{}{}",arr[0],slice[1]);
    slice.len();
    string.push('a');
    string.push_str("b");
    // mut => mutable in font of string
    let mut string = String::from("hello");
    
    //11. if Statement
    let n = 3;
    if n < 5 {
        println!("n is less than 5");
    } else {
        println!("n is greater than or equal to 5");
    }
    // 12 for loop like python
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {}", element);
    }
    // 13. while loop
    let mut n = 0;
    while n < 5 {
        println!("The value of n is: {}", n);
        n += 1;
    }

    // 14. break (no ; at the end)
    // 15. match
    let n = 3;
    match n {
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        _ => println!("The number is something else"),
    }
    // 16.struct
    // let name = String::from("Bird");
    // let bird = Bird{ name: "Eagle", age: 5};
    struct Bird {
       name: String,
       age: u8,

    }
    impl Bird  {
        fn print_name(&self){
            println!("The name of the bird is: {}", self.name);
        }
    }
   // 16. Trait 
}
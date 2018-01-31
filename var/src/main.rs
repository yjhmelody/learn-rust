fn main() {
    const MAX: u32 = 100;
    println!("{}", MAX);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("len {}", spaces);

    let age = 21;
    let person = Person{age};
    println!("{}", person.age);
    let person2 = person;
    println!("{}", person2.age);
    

    let slice = &String::from("yjh")[1..];
    println!("{}", slice);

    let x = 5;
    let raw = &x as *const i32;
    let point_at = unsafe {*raw};
    let alias = point_at;
    println!("raw point at {}", point_at);
    println!("raw point at {:?}", raw);
    println!("raw point at {}", alias);
    
}

struct Person {
    age: u32,
}
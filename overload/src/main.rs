use std::ops::*;


#[derive(Debug)]
struct Complex {
    a: f64,
    b: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex{a: self.a + other.a, b: self.b + other.b}
    }
}

impl Add<i32> for Complex {
    type Output = f64;
    fn add(self, other: i32) -> f64 {
        self.a + self. b + (other as f64)
    }
}



trait HasArea<T> {
    fn area(&self) -> T;
}

#[derive(Debug)]
struct Square <T>{
    x: T,
    y: T,
    side: T,
}

impl <T> HasArea<T> for Square<T>
    where T: Mul<Output=T> + Copy
{
    fn area(&self) -> T {
        self.side * self.side
    }
}

fn add<T:Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

fn main() {
    let c1 = Complex{a: 1f64, b:2.0};
    let c2 = Complex{a: 2f64, b:-1.0};
    let c3 = c1 + c2;
    println!("{:?}", c3);

    let s = Square {
        x: 1.0f64,
        y: 1.0f64,
        side: 12.0f64,
    };

    println!("{:?}", s);

    add(1, 2);
}

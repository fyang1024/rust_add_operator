use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ComplexNumber {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

fn add_complex_numbers<T: Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}

fn main() {
    let a = ComplexNumber {
        real: 2.1,
        imaginary: 3.2,
    };
    let b = ComplexNumber {
        real: 0.99,
        imaginary: 300.13,
    };
    let result = add_complex_numbers(a, b);
    println!("{:?} + {:?} = {:?}", a, b, result);
}

/*
// A trait that has an associated type `Output`
trait Increment<I> {

    fn inc(self) -> I;
}

// Implement it for i32: incrementing an `i32` yields another `i32`
impl Increment<String> for i32 {
       // fixed: always i32
    fn inc(self) -> String {
        (self + 1).to_string()
    }
}

// Imagine we wanted f32 to increment to f64 – that's fine:
impl Increment<i32> for i32{

    fn inc(self) -> i32 {
        (self + 1) as i32
    }
}

fn main() {
    let a: i32 = 5;
    let b: f32 = 2.5;

    let a_inc = a.inc();   // a_inc: i32
    let b_inc = b.inc();   // b_inc: f64

    println!("{}, {}", a_inc, b_inc);
}
*/

// A trait with an associated type `Output`
trait GenerateL<I> {
    fn generate(self) -> I;
}

// Implementing Generate for String with `Output` being i32
impl GenerateL<String> for String {
    fn generate(self) -> String {
        42.to_string()
    }
}

// You CANNOT implement Generate for String again with a different Output type.
// This will cause a conflict:
impl GenerateL<i32> for String {
    fn generate(self) -> i32 {
        6
    }
}

fn main() {
    //let my_string = String::from("Generate value: ");
    //let result = my_string.generate(); // Will always return an i32 in this case

    // println!("{}", result); // Will print 42
}

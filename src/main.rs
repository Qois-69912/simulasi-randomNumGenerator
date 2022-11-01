fn main() {
    // LCG
    let c: i32 = 10000;
    let m: i32 = 90;
    let a: i32 = 9;
    let x0: i32 = 100;
    let x1: i32 = ((a * x0) + c) % m;

    println!("{}", x1)
}

fn main() {
    // LCG
    let c: i64 = 1013904223;
    let m: i64 = 4294967296;
    let a: i64 = 1664525;
    let x0: i64 = 100;
    let x1: i64 = ((a * x0) + c) % m;

    println!("{}", x1)
}

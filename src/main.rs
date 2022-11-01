use chrono::*;

fn main() {
    // Chrono
    let lc: DateTime<Local> = Local::now();

    // LCG
    let c: i64 = 1013904223;
    let m: i64 = 4294967296;
    let a: i64 = 1664525;
    let x0: i64 = (lc.hour() as i64 * lc.minute() as i64 * lc.second() as i64)
        + lc.day() as i64 * lc.year() as i64 * lc.month() as i64;
    let x1: i64 = ((a * x0) + c) % m;

    println!("{}", x1)
}

use chrono::*;
use std::io;
use std::io::Write;

// Read User Input and make sure it is number
fn usr_int(msg: String) -> i64 {
    let int: i64;

    loop {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error getting guess");

        int = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    return int;
}

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

    // User Input
    let awal: i64;
    let akhir: i64;

    println!("==== NUMBER GENERATOR ==== \n");
    println!("Generate angka");
    awal = usr_int("Antara angka : ".to_string());
    akhir = usr_int("Dan angka : ".to_string());

    // Generate random numbers within a range
    let offset: i64;
    let range: i64;

    if awal > akhir {
        offset = akhir;
        range = awal - akhir;
    } else {
        offset = awal;
        range = akhir - awal;
    }

    let result: i64 = offset + (x1 % range);

    // Resultttt
    println!("Angka acak {} dan {} adalah : {}", awal, akhir, result);
}

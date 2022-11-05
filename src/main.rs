use chrono::*;
use colored::*;
use std::io;
use std::io::Write;

// Read User Input and make sure it is number
fn usr_int(msg: String) -> i64 {
    let int: i64;

    // Terus melakakukan loop hingga user memasukkan angka //
    /* KEKURANGAN :
        Tidak menampilkan pesan error ketika user tidak
    memasukkan angka */
    loop {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error getting guess");

        // Make sure it is number
        int = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Memulai loop
        };

        break;
    }

    return int;
}

fn main() {
    // Chrono
    let lc: DateTime<Local> = Local::now();

    // LCG
    // Untuk keterangan cek README
    let c: i64 = 1013904223;
    let m: i64 = 4294967296;
    let a: i64 = 1664525;
    let x0: i64 = (lc.hour() as i64 * lc.minute() as i64 * lc.second() as i64)
        + lc.day() as i64 * lc.year() as i64 * lc.month() as i64;
    let x1: i64 = ((a * x0) + c) % m;

    // User Input
    let awal: i64;
    let akhir: i64;

    println!("");
    println!("{}", "==============================".yellow().bold());
    println!("{}", "====== NUMBER GENERATOR ======".yellow().bold());
    println!("{}", "==============================".yellow().bold());

    println!("");
    println!("{}", "Generate angka".cyan().bold());

    // Get The user Input
    awal = usr_int("Antara angka : ".green().bold().to_string());
    akhir = usr_int("Dan angka : ".green().to_string());

    // Generate random numbers within a range
    let offset: i64;
    let range: i64;

    // Calculate the range
    if awal > akhir {
        offset = akhir;
        range = awal - akhir;
    } else {
        offset = awal;
        range = akhir - awal;
    }

    // Calculate hasil berdasarkan user input
    // Jika rangenya 0, maka result sama dengan user input (Tentu saja)
    let result: i64;
    if range != 0 {
        result = offset + (x1 % range);
    } else {
        result = awal
    }

    // Resultttt
    // Format : Angka Acak antara x dan y adalah : z
    println!(
        "{} {} {} {} {} {}",
        format!("Angka Acak antara").cyan().bold(),
        awal,
        format!("dan").cyan().bold(),
        akhir,
        format!("adalah :").cyan().bold(),
        String::from(result.to_string()).red().bold()
    );

    println!("");
}

use std::{thread, io, time};

fn main() {
    let mut line = String::new();
    println!("Entrer un chiffre:");
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read the line");
    let n: u32 = line.trim().parse().expect("Input not an integer");

    fizzbuzz(n);
}

fn fizzbuzz(n:u32) {
    let condition1: u32 = 3;
    let condition2: u32 = 5;

    let mut vec = Vec::new();

    for i in 0..n {
        vec.push("".to_owned());

        if (i + 1) % condition1 == 0 {
            vec[i as usize] = "fizz".to_owned();
        }
        if (i + 1) % condition2 == 0 {
            vec[i as usize] = vec[i as usize].to_owned() + "buzz";
        }
        if vec[i as usize].is_empty() {
            let s = format!("{}", i+1 );
            vec[i as usize] = s;
        }
    }
    println!("Calculation finished!");
    thread::sleep(time::Duration::from_secs(5));
    println!("{:?}", vec);
}
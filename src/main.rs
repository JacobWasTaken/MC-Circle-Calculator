use std::io;
use std::f64;
use std::fs;

fn main() {
    // Get the radius of the circle
    println!("Radius:");

    let mut r = String::new();

    io::stdin().read_line(&mut r).expect("Failed to read line");

    let r: f64 = r.trim().parse().expect("Enter a floating point integer");
    let r = r.abs();

    // Get the interval (spacing) for each point
    println!("\nInterval (Whole Number):");

    let mut i = String::new();

    io::stdin().read_line(&mut i).expect("Failed to read line");

    let i: f64 = i.trim().parse().expect("Enter an integer");
    let i = i.abs().floor();

    println!("");

    // Variables
    let mut c = 0.0;
    let d = 2.0 * f64::consts::PI / i;
    let mut a = 0.0;
    let mut stand: u64 = 0;

    // Text file
    println!("Do you want to output to a text file? (y/n)");
    loop {
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Failed to read line");

        match ans.as_str().trim() {
            "y" | "Y" => {
                text();
                break();
            },
            "n" | "N" => break,
            _ => println!("y/n only"),
        }
    }

    // While loop for each point
    while c != i {
        println!("{:.8}, {:.8}", r * f64::cos(a), r * f64::sin(a));
        println!("execute at @e[tag=stand] run tp @e[tag=stand{stand}] ^{:.4} ^ ^{:.4} facing entity @e[tag=stand,limit=1]\n", r * f64::cos(a), r * f64::sin(a));

        a += d;
        stand += 1;
        c += 1.0;
    }
    
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn text() {
    let text = format!("test");
    fs::write("text.txt", text).expect("Unable to write to file");
    println!("Created test.txt in local directory\n");
}
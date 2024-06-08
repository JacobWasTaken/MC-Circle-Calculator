use std::io;
use std::f64;

fn main() {
    // Get the radius of the circle
    println!("Radius:");

    let mut r = String::new();

    io::stdin()
        .read_line(&mut r)
        .expect("Failed tp read line");

    let r: f64 = r.trim().parse().expect("Enter a floating point integer.");
    let r = r.abs();

    // Get the interval (spacing) for each point
    println!("\nInterval (Whole Number):");

    let mut i = String::new();

    io::stdin()
        .read_line(&mut i)
        .expect("Failed tp read line");

    let i: f64 = i.trim().parse().expect("Enter an integer.");
    let i = i.abs().floor();

    println!("");

    // Variables
    let mut c = 0.0;
    let d = 2.0 * f64::consts::PI / i;
    let mut a = 0.0;
    let mut stand: u64 = 0;

    // While loop for each point and command
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
use std::io;

fn main() {
    println!("Select number of your operation.");
    println!("1. Add\n2. Subtract\n3. Multiply\n4. Divide");

    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read line");
    let op: i32 = op.trim().parse().expect("Select number of your operation.");

    match op {
        1 => {
            println!("Enter your first number to be added:");
            let mut x = String::new();
            io::stdin().read_line(&mut x).expect("Failed to read line");
            let x: f32 = x.trim().parse().expect("Enter your first number to be added:");
            
            println!("Enter your second number:");
            let mut y = String::new();
            io::stdin().read_line(&mut y).expect("Failed to read line");
            let y: f32 = y.trim().parse().expect("Enter your second number:");
            add(x,y);
        },
        2 => {
            println!("Enter your first number to be subtracted: (Format num1-num2)");
            let mut x = String::new();
            io::stdin().read_line(&mut x).expect("Failed to read line");
            let x: f32 = x.trim().parse().expect("Enter your first number to be subtracted: (Format num1-num2)");
            
            println!("Enter your second number:");
            let mut y = String::new();
            io::stdin().read_line(&mut y).expect("Failed to read line");
            let y: f32 = y.trim().parse().expect("Enter your second number:");
            sub(x,y);
        },
        3 => {
            println!("Enter your first number to multiply:");
            let mut x = String::new();
            io::stdin().read_line(&mut x).expect("Failed to read line");
            let x: f32 = x.trim().parse().expect("Enter your first number to multiply:");
            
            println!("Enter your second number:");
            let mut y = String::new();
            io::stdin().read_line(&mut y).expect("Failed to read line");
            let y: f32 = y.trim().parse().expect("Enter your second number:");
            mult(x,y);
        },
        4 => {
            println!("Enter your first number to divide: (Format: num1/num2)");
            let mut x = String::new();
            io::stdin().read_line(&mut x).expect("Failed to read line");
            let x: f32 = x.trim().parse().expect("Enter your first number to divide: (Format: num1/num2)");
        
            println!("Enter your second number:");
            let mut y = String::new();
            io::stdin().read_line(&mut y).expect("Failed to read line");
            let y: f32 = y.trim().parse().expect("Enter your second number:");
            div(x,y);
        },
    _ => {println!("Invalid input")}
    }
}

fn sub(ado:f32, adt:f32) {
    let ada = ado-adt;
    println!("{}", ada)
}

fn div(ado:f32, adt:f32) {
    let ada = ado/adt;
    println!("{}", ada)
}

fn mult(ado:f32, adt:f32) {
    let ada = ado*adt;
    println!("{}", ada)
}

fn add(ado:f32, adt:f32) {
    let ada = ado+adt;
    println!("{}", ada)
}
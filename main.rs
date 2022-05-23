use std::io;

fn main() {
    println!("Select number of your operation.");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read line");
    let op: i32 = op.trim().parse().expect("Select number of your operation.");

    if op == 1 {
        println!("Enter your first number to be added:");
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Enter your first number to be added:");
    
        println!("Enter your second number:");
        let mut y = String::new();
        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read line");
        let y: i32 = y.trim().parse().expect("Enter your second number:");
        add(x,y); 
    } else if op == 2 {
        println!("Enter your first number to be subtracted: (Format num1-num2)");
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Enter your first number to be subtracted: (Format num1-num2)");
    
        println!("Enter your second number:");
        let mut y = String::new();
        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read line");
        let y: i32 = y.trim().parse().expect("Enter your second number:");
        sub(x,y); 
    } else if op == 3 {
        println!("Enter your first number to multiply:");
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Enter your first number to multiply:");
    
        println!("Enter your second number:");
        let mut y = String::new();
        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read line");
        let y: i32 = y.trim().parse().expect("Enter your second number:");
        mult(x,y); 
    } else if op == 4 {
        println!("Enter your first number to divide: (Format: num1/num2)");
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Enter your first number to divide: (Format: num1/num2)");
    
        println!("Enter your second number:");
        let mut y = String::new();
        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read line");
        let y: i32 = y.trim().parse().expect("Enter your second number:");
        div(x,y); 
    } else {
        println!("Invalid input.");
    }

}

fn sub(ado:i32, adt:i32) {
    let ada = ado-adt;
    println!("{}", ada)
}

fn div(ado:i32, adt:i32) {
    let ada = ado/adt;
    println!("{}", ada)
}

fn mult(ado:i32, adt:i32) {
    let ada = ado*adt;
    println!("{}", ada)
}

fn add(ado:i32, adt:i32) {
    let ada = ado+adt;
    println!("{}", ada)
}
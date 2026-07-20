use std::io::{self, Write};
fn main(){
    println!("Please enter a temperature number: ");
    io::stdout().flush().unwrap();
    let mut temp = String::new(); 
    io::stdin().read_line(&mut temp).expect("failed to read line");
    let temp: f32 = temp.trim().parse().expect("Please type a number!");
    let celsius = (temp - 32.0) / 1.8;
    println!("the celsius is {celsius}");


}
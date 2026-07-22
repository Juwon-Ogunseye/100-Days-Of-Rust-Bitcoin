// use std::io::{self, Write};
// fn main(){
//     println!("Please enter a temperature number: ");
//     io::stdout().flush().unwrap();
//     let mut temp = String::new(); 
//     io::stdin().read_line(&mut temp).expect("failed to read line");
//     let temp: f32 = temp.trim().parse().expect("Please type a number!");
//     let celsius = (temp - 32.0) / 1.8;
//     println!("the celsius is {celsius}");


// }
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }
fn main() {
    let s = String::from("hello");

    let len = s.len();

    // let slice = &s[3..len];
    let slice = &s[3..];

    println!("{}", slice)

}

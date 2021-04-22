mod math_functions;
use math_functions::add;
use math_functions::get_input;

fn main() {

    println!("Which calculation would you like to do?\n1: addition\n2: subtraction\n3: multiplication\n4: division");
    let choice: i32 = get_input().trim().parse().unwrap();

    if choice == 1
    {
        let answer = add();
        println!("Your answer is: {}", answer);
    }
}
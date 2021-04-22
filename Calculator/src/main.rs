mod math_functions;
use math_functions::get_input;
use math_functions::add;
use math_functions::sub;
use math_functions::mult;
use math_functions::div;

fn main() 
{
    let mut done: bool = false;
    let mut check_done: String;

    while done != true {
        println!("Which calculation would you like to do?\n1: addition\n2: subtraction\n3: multiplication\n4: division");
        let choice: i32 = get_input().trim().parse().unwrap();

        if choice == 1
        {
            let answer = add();
            println!("Your answer is: {}", answer);
        }
        else if choice == 2
        {
            let answer = sub();
            println!("Your answer is: {}", answer);
        }
        else if choice == 3
        {
            let answer = mult();
            println!("Your answer is: {}", answer);
        }
        else if choice == 4
        {
            let answer = div();
            println!("Your answer is: {}", answer);
        }

        println!("Are you done?");
        check_done = get_input().trim().parse().unwrap();

        if check_done.to_lowercase() == "yes"
        {
            done = true;
        }
    }
}
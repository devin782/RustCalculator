mod math_functions;
use math_functions::get_input;

fn main() 
{
    let mut done: bool = false;
    let mut check_done: String;

    while done != true {
        println!("Which calculation would you like to do?
        1: Addition
        2: Subtraction
        3: Multiplication
        4: Division
        5: Square
        6: Cube
        7: Square Root
        8: Quadratic");
            
        let choice: u32 = get_input().trim().parse().unwrap();

        if choice == 1
        {
            let answer = math_functions::add();
            println!("Your answer is: {}", answer);
        }
        else if choice == 2
        {
            let answer = math_functions::sub();
            println!("Your answer is: {}", answer);
        }
        else if choice == 3
        {
            let answer = math_functions::mult();
            println!("Your answer is: {}", answer);
        }
        else if choice == 4
        {
            let answer = math_functions::div();
            println!("Your answer is: {}", answer);
        }
        else if choice == 5
        {
            let answer = math_functions::sqr();
            println!("Your answer is: {}", answer);
        }
        else if choice == 6
        {
            let answer = math_functions::cube();
            println!("Your answer is: {}", answer);
        }
        else if choice == 7
        {
            let answer = math_functions::sqr_root();
            println!("Your answer is: {}", answer);
        }
        else if choice == 8
        {
            let answer = math_functions::quadratic();
            println!("Zeros: {} and {} ", answer[0], answer[1])
        }

        println!("Are you done?");
        check_done = get_input().trim().parse().unwrap();

        if check_done.to_lowercase() == "yes"
        {
            done = true;
        }
    }
}
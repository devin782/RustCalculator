mod math_functions;
use math_functions::get_input;

fn main() 
{
    let mut done: bool = false;
    let mut check_done: String;

    while done != true {
        println!("Which calculation would you like to do?
        1:  Addition
        2:  Subtraction
        3:  Multiplication
        4:  Division
        5:  Square
        6:  Cube
        7:  Exponent
        8:  Square Root
        9:  Quadratic
        10: Sine
        11: Cosine
        12: Tangent
        13: Cosecant
        14: Secant
        15: Cotangent
        16: Derivative");
            
        let choice: u32 = get_input().trim().parse().unwrap();

        match choice
        {
            1 => 
            {
                let answer = math_functions::add();
                println!("Your answer is: {}", answer);
            }
            2 => 
            {
                let answer = math_functions::sub();
                println!("Your answer is: {}", answer);
            }
            3 => 
            {
                let answer = math_functions::mult();
                println!("Your answer is: {}", answer);
            }
            4 => 
            {
                let answer = math_functions::div();
                println!("Your answer is: {}", answer);
            }
            5 => 
            {
                let answer = math_functions::sqr();
                println!("Your answer is: {}", answer);
            }
            6 => 
            {
                let answer = math_functions::cube();
                println!("Your answer is: {}", answer);
            }
            7 =>
            {
                let answer = math_functions::exp();
                println!("Your answer is: {}", answer);
            }
            8 =>
            {
                let answer = math_functions::sqr_root();
                println!("Your answer is: {}", answer);
            }
            9 => 
            {
                let answer = math_functions::quadratic();
                println!("Zeros: {} and {} ", answer[0], answer[1])
            }
            10 =>
            {
                let answer = math_functions::sine();
                println!("Your answer is: {}", answer);
            }
            11 =>
            {
                let answer = math_functions::cosine();
                println!("Your answer is: {}", answer);
            }
            12 =>
            {
                let answer = math_functions::tan();
                println!("Your answer is: {}", answer);
            }
            13 =>
            {
                let answer = math_functions::cosec();
                println!("Your answer is: {}", answer);
            }
            14 =>
            {
                let answer = math_functions::sec();
                println!("Your answer is: {}", answer);
            }
            15 =>
            {
                let answer = math_functions::cotan();
                println!("Your answer is: {}", answer);
            }
            16 =>
            {
                let answer = math_functions::derivative();
                if answer[2] == 0.0
                {
                    println!("Result: {}x + {}", answer[0], answer[1]);
                }
                else if answer[3] == 0.0
                {
                    println!("Result: {}x^2 + {}x + {}", answer[0], answer[1], answer[2]);
                }
                else if answer[4] == 0.0
                {
                    println!("Result: {}x^3 + {}x^2 + {}x + {}", answer[0], answer[1], answer[2], answer[3]);
                }
            }
            _ => println!("Invalid Input")
        }

        println!("\nAre you done?");
        check_done = get_input().trim().parse().unwrap();

        if check_done.to_lowercase() == "yes"
        {
            done = true;
        }
    }
}
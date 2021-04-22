pub fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn add() -> i32
{
    println!("Input first value:");
    let x: i32 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: i32 = get_input().trim().parse().unwrap();

    let result = x + y;
    return result;
}
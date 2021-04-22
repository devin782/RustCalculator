pub fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn add() -> f32
{
    println!("Input first value:");
    let x: f32 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f32 = get_input().trim().parse().unwrap();

    let result = x + y;
    return result;
}

pub fn sub() -> f32
{
    println!("Input first value:");
    let x: f32 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f32 = get_input().trim().parse().unwrap();

    let result = x - y;
    return result;
}

pub fn mult() -> f32
{
    println!("Input first value:");
    let x: f32 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f32 = get_input().trim().parse().unwrap();

    let result = x * y;
    return result;
}

pub fn div() -> f32
{
    println!("Input first value:");
    let x: f32 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f32 = get_input().trim().parse().unwrap();

    let result = x / y;
    return result;
}
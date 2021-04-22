pub fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn add() -> f64
{
    println!("Input first value:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x + y;
    return result;
}

pub fn sub() -> f64
{
    println!("Input first value:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x - y;
    return result;
}

pub fn mult() -> f64
{
    println!("Input first value:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x * y;
    return result;
}

pub fn div() -> f64
{
    println!("Input first value:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Input second value:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x / y;
    return result;
}

pub fn sqr() -> f64
{
    println!("Input number:");
    let x: f64 = get_input().trim().parse().unwrap();
    let result = x*x;
    return result;
}

pub fn cube() -> f64
{
    println!("Input number:");
    let x: f64 = get_input().trim().parse().unwrap();
    let result = x*x*x;
    return result;
}

pub fn exp() -> f64
{
    let mut result: f64;

    println!("Input base number:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Input exponent:");
    let mut y: i64 = get_input().trim().parse().unwrap();

    result = x;
    while y > 1
    {
        result = result*x;
        y = y - 1;
    }

    return result;
}

pub fn sqr_root() -> f64
{
    println!("Input number:");
    let x: f64 = get_input().trim().parse().unwrap();
    let result = x.sqrt();
    return result;
}

pub fn quadratic() -> [f64; 2]
{
    println!("Input your a:");
    let mut a: f64 = get_input().trim().parse().unwrap();
    while a == 0.0
    {
        println!("Pick an a other than 0");
        a = get_input().trim().parse().unwrap();
    }
    
    println!("Input your b:");
    let mut b: f64 = get_input().trim().parse().unwrap();
    println!("Input your c:");
    let mut c: f64 = get_input().trim().parse().unwrap();
    while b == 0.0 && c == 0.0
    {
        println!("Can't have a zero for both b and c");
        println!("Input your b:");
        b = get_input().trim().parse().unwrap();
        println!("Input your c:");
        c = get_input().trim().parse().unwrap();
    }

    let zero_1 = (-b + (b*b - 4.0*a*c as f64).sqrt()) / (2.0*a);
    let zero_2 = (-b - (b*b - 4.0*a*c as f64).sqrt()) / (2.0*a);

    let result = [zero_1, zero_2];
    return result;
}

pub fn sine() -> f64
{
    println!("Give opposite length:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Give hypotenuse length:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x/y;
    return result;
}
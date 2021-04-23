pub fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn add(x: f64, y: f64) -> f64
{
    let result = x + y;
    return result;
}

pub fn sub(x: f64, y: f64) -> f64
{
    let result = x - y;
    return result;
}

pub fn mult(x: f64, y: f64) -> f64
{
    let result = x * y;
    return result;
}

pub fn div(x: f64, y: f64) -> f64
{
    let result = x / y;
    return result;
}

pub fn exp(x: f64, mut y: f64) -> f64
{
    let mut result: f64;

    result = x;
    while y > 1.0
    {
        result = result*x;
        y = y - 1.0;
    }

    return result;
}

pub fn root(x: f64, y: f64) -> f64
{
    let mut result: f64 = 1.0;
    let mut temp: f64;
    let precision = 0.000001;

    temp = result;
    while temp <= x
    {
        let mut power = y;
        temp = result;
        while power > 1.0
        {
            temp = temp*result;
            power = power - 1.0;
        }
        if temp <= x
        {
            result = result+1.0;
        } 
    }

    
    result = result - 1.0;
    temp = 0.0;
    while temp < x
    {
        let mut power = y;
        temp = result;
        while power > 1.0
        {
            temp = temp*result;
            power = power - 1.0;
        }
        if temp < x
        {
            result = result + precision;  
        }
    }
    
    return result;
}

pub fn quadratic(a: f64, b:f64, c: f64) -> [f64; 2]
{
    let zero_1 = (-b + (b*b - 4.0*a*c as f64).sqrt()) / (2.0*a);
    let zero_2 = (-b - (b*b - 4.0*a*c as f64).sqrt()) / (2.0*a);

    let result = [zero_1, zero_2];
    return result;
}

pub fn sine(x: f64, y:f64) -> f64
{
    let result = x/y;
    return result;
}

pub fn cosine() -> f64
{
    println!("Give adjacent length:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Give hypotenuse length:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x/y;
    return result;
}

pub fn tan() -> f64
{
    println!("Give opposite length:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Give adjacent length:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x/y;
    return result;
}

pub fn cosec(x: f64, y: f64) -> f64
{
    let result = x/y;
    return result;
}

pub fn sec() -> f64
{
    println!("Give hypotenuse length:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Give adjacent length:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x/y;
    return result;
}

pub fn cotan() -> f64
{
    println!("Give adjacent length:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Give opposite length:");
    let y: f64 = get_input().trim().parse().unwrap();

    let result = x/y;
    return result;
}

pub fn derivative() -> [f64; 5]
{
    let mut result = [0.0,0.0,0.0,0.0,0.0];

    println!("Is your highest power 4, 3, 2?");
    let power: i32 = get_input().trim().parse().unwrap();

    match power
    {
        2 =>
        {
            println!("Input leading coefficient");
            let mut a: f64 = get_input().trim().parse().unwrap();
            println!("Input your b:");
            let mut b: f64 = get_input().trim().parse().unwrap();
            println!("Input your c:");
            let mut c: f64 = get_input().trim().parse().unwrap();

            a = a*2.0;
            b = b*1.0;
            c = c*0.0;
            result[0] = a;
            result[1] = b;
            result[2] = c;
        }
        3 =>
        {
            println!("Input leading coefficient:");
            let mut a: f64 = get_input().trim().parse().unwrap();
            println!("Input your 2nd coefficient:");
            let mut b: f64 = get_input().trim().parse().unwrap();
            println!("Input your 3rd coefficient:");
            let mut c: f64 = get_input().trim().parse().unwrap();
            println!("Input your c:");
            let mut d: f64 = get_input().trim().parse().unwrap();

            a = a*3.0;
            b = b*2.0;
            c = c*1.0;
            d = d*0.0;
            result[0] = a;
            result[1] = b;
            result[2] = c;
            result[3] = d;
        }
        4 =>
        {
            println!("Input leading coefficient:");
            let mut a: f64 = get_input().trim().parse().unwrap();
            println!("Input your 2nd coefficient:");
            let mut b: f64 = get_input().trim().parse().unwrap();
            println!("Input your 3rd coefficient:");
            let mut c: f64 = get_input().trim().parse().unwrap();
            println!("Input your 4th coefficient:");
            let mut d: f64 = get_input().trim().parse().unwrap();
            println!("Input your c:");
            let mut e: f64 = get_input().trim().parse().unwrap();

            a = a*4.0;
            b = b*3.0;
            c = c*2.0;
            d = d*1.0;
            e = e*0.0;
            result[0] = a;
            result[1] = b;
            result[2] = c;
            result[3] = d;
            result[4] = e;
        }
        _ => println!("Invalid power")
    }
    return result;
}
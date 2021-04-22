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

pub fn cosec() -> f64
{
    println!("Give hypotenuse length:");
    let x: f64 = get_input().trim().parse().unwrap();
    println!("Give opposite length:");
    let y: f64 = get_input().trim().parse().unwrap();

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
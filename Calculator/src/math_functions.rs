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

pub fn cosec(x: f64, y: f64) -> f64
{
    let result = x/y;
    return result;
}

pub fn cosine(x: f64, y: f64) -> f64
{
    let result = x/y;
    return result;
}

pub fn sec(x: f64, y: f64) -> f64
{
    let result = x/y;
    return result;
}

pub fn tan(x: f64, y: f64) -> f64
{
    let result = x/y;
    return result;
}

pub fn cotan(x: f64, y: f64) -> f64
{
    let result = x/y;
    return result;
}

pub fn derivative(arr: & [f64]) -> [f64; 4]
{
    let mut result = [0.0,0.0,0.0,0.0];

    let size = arr.len();

    match size
    {
        2 =>
        {
            result[0] = arr[0]*1.0;
            result[1] = arr[1]*0.0;
        }
        3 =>
        {
            result[0] = arr[0]*2.0;
            result[1] = arr[1]*1.0;
            result[2] = arr[2]*0.0;
        }
        4 =>
        {
            result[0] = arr[0]*3.0;
            result[1] = arr[1]*2.0;
            result[2] = arr[2]*1.0;
            result[3] = arr[3]*0.0;
        }
        _ => println!("Invalid power")
    }
    return result;
}
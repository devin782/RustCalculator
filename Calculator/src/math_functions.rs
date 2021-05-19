use fltk::prelude::*;
use fltk::enums::*;
use fltk::app::*;
use fltk::window::*;
use fltk::button::*;
use fltk::output::*;
use fltk::input::*;
use fltk::dialog::*;

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

pub fn factorial(x: f64) -> f64
{
    let mut n = x;
    let mut result = x;

    while n > 1.0
    {
        n = n - 1.0;
        result = result*n;
    }

    return result;
}

pub fn permutation(x: f64, r: f64) -> f64
{
    let result = factorial(x)/(factorial(x - r));
    return result;
}

pub fn combination(x: f64, r: f64) -> f64
{
    let result = factorial(x)/(factorial(x - r)*factorial(r));
    return result;
}

pub fn pv(fv: f64, i: f64, n: f64) -> f64
{
    let result = fv/exp(1.0+i, n);
    return result;
}

pub fn fv(pv: f64, i: f64, n: f64) -> f64
{
    let result = pv*exp(1.0+i, n);
    return result;
}

pub fn rate(pv: f64, interest: f64, n: f64) -> f64
{
    let result = (100.0 * interest)/(pv * n);
    return result
}

pub fn breakeven(rev: f64, fixed: f64, variable: f64) -> f64
{
    let mut result = rev - variable;
    result = fixed/result;
    return result;
}

pub fn profit(rev: f64, fixed: f64, variable: f64) -> f64
{
    let result = rev - fixed - variable;
    return result;
}

pub fn avgcost(variable: f64, fixed: f64, activity: f64) -> f64
{
    let result = (variable + fixed)/activity;
    return result;
}

pub fn calc()
{
    //Set up application
    let app = App::default();
    //Set up window
    let mut win = Window::new(100, 100, 400, 300, "Calculator").center_screen();
    
    //Display text
    let out = "What calculation would you like?";
    let disp = Output::new(0, 0, 400, 20, "");
    disp.set_value(out);

    //Add buttons for calculations
    let mut add_but = Button::new(0, 20, 100, 40, "Addition");
    let mut sub_but = Button::new(0, 60, 100, 40, "Subtraction");
    let mut mult_but = Button::new(0, 100, 100, 40, "Multiplication");
    let mut div_but = Button::new(0, 140, 100, 40, "Division");
    let mut exp_but = Button::new(0, 180, 100, 40, "Exponent");
    let mut root_but = Button::new(0, 220, 100, 40, "Root");
    let mut quad_but = Button::new(0, 260, 100, 40, "Quadratic");
    let mut sine_but = Button::new(100, 20, 100, 40, "Sin/Cosec");
    let mut cosine_but = Button::new(100, 60, 100, 40, "Cos/Sec");
    let mut tangent_but = Button::new(100, 100, 100, 40, "Tan/Cotan");
    let mut der_but = Button::new(100, 140, 100, 40, "Derivative");
    let mut fac_but = Button::new(100, 180, 100, 40, "Factorial");
    let mut perm_but = Button::new(100, 220, 100, 40, "Permutation");
    let mut comb_but = Button::new(100, 260, 100, 40, "Combination");
    let mut pv_but = Button::new(200, 20, 100, 40, "Present Value");
    let mut fv_but = Button::new(200, 60, 100, 40, "Future Value");
    let mut rate_but = Button::new(200, 100, 100, 40, "Rate");
    let mut beven_but = Button::new(200, 140, 100, 40, "Break Even");
    let mut profit_but = Button::new(200, 180, 100, 40, "Profit");
    let mut avg_but = Button::new(200, 220, 100, 40, "Average Cost");
    let mut itb_but = Button::new(200, 260, 100, 40, "Int to Binary");
    let mut ith_but = Button::new(300, 20, 100, 40, "Int to Hex");

    //Handlers for the buttons

    add_but.handle(move |_add_but, ev: Event| { match ev {
        Event::Push => {
            //Open new window
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

            //Create inputs, output, and a result button
            let inp0 = FloatInput::new(100, 0, 200, 50, "X:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Y:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            //Handle the enter
            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    //Turn answer into a string
                    let answer = add(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    //Output answer
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    sub_but.handle(move |_sub_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Subtraction").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "X:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Y:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = sub(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    mult_but.handle(move |_mult_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Multiplication").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "X:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Y:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = mult(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});
    
    div_but.handle(move |_div_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Division").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "X:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Y:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = div(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    exp_but.handle(move |_exp_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Exponent").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "X:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Exponent:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = exp(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    root_but.handle(move |_root_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Root").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "X:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Root:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = root(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    quad_but.handle(move |_quad_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Quadratic").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "A:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "B:");
            let inp2 = FloatInput::new(100, 100, 200, 50, "C:");
            let result = Output::new(100, 150, 200, 50, "Zeros:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter");

              
                enter.handle(move |_enter, num: Event| { match num {
                    Event::Push => {
                        if inp0.value().parse::<f64>().unwrap() != 0.0 && !(inp1.value().parse::<f64>().unwrap() == 0.0 && inp2.value().parse::<f64>().unwrap() == 0.0)
                        {
                            let answer = quadratic(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap(), inp2.value().parse::<f64>().unwrap());
                            let ans = format!("{:.2}", answer[0]).to_string();
                            let ans2 = format!("{:.2}", answer[1]).to_string();
                            result.set_value(&(ans + " and " + &ans2));
                        }
                        else 
                        {
                            let _warning = alert(300, 300, "Please don't put '0' in 'a' or have a '0' in both 'b' and 'c'!");
                        }
                        true
                    },
                    _ => false,
                }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    sine_but.handle(move |_sine_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Sine/Cosecant").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Opposite:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Hypotenuse:");
            let s_result = Output::new(100, 100, 200, 50, "Sine:");
            let c_result = Output::new(100, 150, 200, 50, "Cosecant:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter for Sine");


            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let s_answer = sine(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    let c_answer = cosec(inp1.value().parse::<f64>().unwrap(), inp0.value().parse::<f64>().unwrap()).to_string();
                    if s_answer == "inf"
                    {
                        s_result.set_value("Division by zero");
                        c_result.set_value(&c_answer);
                    }
                    else if c_answer == "inf"
                    {
                        s_result.set_value(&s_answer);
                        c_result.set_value("Division by zero");
                    }
                    else 
                    {
                        s_result.set_value(&s_answer);
                        c_result.set_value(&c_answer);
                    }
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    cosine_but.handle(move |_cosine_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Cosine/Secant").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Adjacent:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Hypotenuse:");
            let c_result = Output::new(100, 100, 200, 50, "Cosine:");
            let s_result = Output::new(100, 150, 200, 50, "Secant:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter for Cosine");


            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let c_answer = cosine(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    let s_answer = sec(inp1.value().parse::<f64>().unwrap(), inp0.value().parse::<f64>().unwrap()).to_string();
                    if c_answer == "inf"
                    {
                        c_result.set_value("Division by zero");
                        s_result.set_value(&s_answer);
                    }
                    else if s_answer == "inf"
                    {
                        c_result.set_value(&c_answer);
                        s_result.set_value("Division by zero");
                    }
                    else 
                    {
                        c_result.set_value(&c_answer);
                        s_result.set_value(&s_answer);
                    }
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    tangent_but.handle(move |_tangent_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Tangent/Cotangent").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Opposite:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Adjacent:");
            let s_result = Output::new(100, 100, 200, 50, "Tangent:");
            let c_result = Output::new(100, 150, 200, 50, "Cotangent:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter for Tangent");


            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let s_answer = tan(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    let c_answer = cotan(inp1.value().parse::<f64>().unwrap(), inp0.value().parse::<f64>().unwrap()).to_string();
                    if s_answer == "inf"
                    {
                        s_result.set_value("Division by zero");
                        c_result.set_value(&c_answer);
                    }
                    else if c_answer == "inf"
                    {
                        s_result.set_value(&s_answer);
                        c_result.set_value("Division by zero");
                    }
                    else 
                    {
                        s_result.set_value(&s_answer);
                        c_result.set_value(&c_answer);
                    }
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    der_but.handle(move |_der_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 500, 300, "Derivative").center_screen();

            let inp0 = FloatInput::new(150, 0, 200, 50, "2, 3, or 4 variables?");
            let mut enter = Button::new(150, 50, 200, 50, "Enter");


            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    if inp0.value().parse::<f64>().unwrap() == 2.0
                    {
                        let mut d_win = Window::new(100, 100, 500, 300, "2 variables").center_screen();

                        let d_inp0 = FloatInput::new(150, 0, 200, 50, "Leading Coefficient:");
                        let d_inpc = FloatInput::new(150, 50, 200, 50, "C:");
                        let result = Output::new(150, 100, 200, 50, "Result:");
                        let mut d_enter = Button::new(150, 150, 200, 50, "Enter");

                        d_enter.handle(move |_d2_enter, d: Event| { match d {
                            Event::Push => {
                                let arr = [d_inp0.value().parse::<f64>().unwrap(), d_inpc.value().parse::<f64>().unwrap()];
                                let answer = derivative(&arr);
                                result.set_value(&answer[0].to_string());
                                true
                            },
                            _ => false,
                        }});

                        d_win.end();
                        d_win.show();
                        d_win.set_color(Color::White);
                    }
                    else if inp0.value().parse::<f64>().unwrap() == 3.0
                    {
                        let mut d_win = Window::new(100, 100, 500, 300, "3 variables").center_screen();

                        let d_inp0 = FloatInput::new(150, 0, 200, 50, "Leading Coefficient:");
                        let d_inp1 = FloatInput::new(150, 50, 200, 50, "2nd Coefficient:");
                        let d_inpc = FloatInput::new(150, 100, 200, 50, "C:");
                        let result = Output::new(150, 150, 200, 50, "Result:");
                        let mut d_enter = Button::new(150, 200, 200, 50, "Enter");

                        d_enter.handle(move |_d_enter, d: Event| { match d {
                            Event::Push => {
                                let arr = [d_inp0.value().parse::<f64>().unwrap(), d_inp1.value().parse::<f64>().unwrap(), d_inpc.value().parse::<f64>().unwrap()];
                                let answer = derivative(&arr);
                                result.set_value(&(answer[0].to_string() + "x + " + &answer[1].to_string()));
                                true
                            },
                            _ => false,
                        }});

                        d_win.end();
                        d_win.show();
                        d_win.set_color(Color::White);
                    }
                    else if inp0.value().parse::<f64>().unwrap() == 4.0
                    {
                        let mut d_win = Window::new(100, 100, 500, 300, "4 variables").center_screen();

                        let d_inp0 = FloatInput::new(150, 0, 200, 50, "Leading Coefficient:");
                        let d_inp1 = FloatInput::new(150, 50, 200, 50, "2nd Coefficient:");
                        let d_inp2 = FloatInput::new(150, 100, 200, 50, "3rd Coefficient:");
                        let d_inpc = FloatInput::new(150, 150, 200, 50, "C:");
                        let result = Output::new(150, 200, 200, 50, "Result:");
                        let mut d_enter = Button::new(150, 250, 200, 50, "Enter");

                        d_enter.handle(move |_d_enter, d: Event| { match d {
                            Event::Push => {
                                let arr = [d_inp0.value().parse::<f64>().unwrap(), d_inp1.value().parse::<f64>().unwrap(), d_inp2.value().parse::<f64>().unwrap(), d_inpc.value().parse::<f64>().unwrap()];
                                let answer = derivative(&arr);
                                result.set_value(&(answer[0].to_string() + "x^2 + " + &answer[1].to_string() + "x + " + &answer[2].to_string()));
                                true
                            },
                            _ => false,
                        }});

                        d_win.end();
                        d_win.show();
                        d_win.set_color(Color::White);
                    }
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    fac_but.handle(move |_fac_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Factorial").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Start Number:");
            let result = Output::new(100, 50, 200, 50, "Factored:");
            let mut enter = Button::new(100, 100, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = factorial(inp0.value().parse::<f64>().unwrap()).to_string();
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    perm_but.handle(move |_perm_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Permutation").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Start Number:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Picked:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = permutation(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    comb_but.handle(move |_comb_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Combination").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Start Number:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Picked:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = combination(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    result.set_value(&answer);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    pv_but.handle(move |_pv_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Present Value").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Future Value:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Rate(decimal):");
            let inp2 = FloatInput::new(100, 100, 200, 50, "Periods:");
            let result = Output::new(100, 150, 200, 50, "Present Value:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = pv(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap(), inp2.value().parse::<f64>().unwrap());
                    let ans = format!("{:.2}", answer).to_string();
                    result.set_value(&("$".to_string() + &ans));
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    fv_but.handle(move |_fv_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Future Value").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Present Value:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Rate(decimal):");
            let inp2 = FloatInput::new(100, 100, 200, 50, "Periods:");
            let result = Output::new(100, 150, 200, 50, "Future Value:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = fv(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap(), inp2.value().parse::<f64>().unwrap());
                    let ans = format!("{:.2}", answer).to_string();
                    result.set_value(&("$".to_string() + &ans));
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    rate_but.handle(move |_rate_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Rate").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Principle Value:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Interest:");
            let inp2 = FloatInput::new(100, 100, 200, 50, "Time:");
            let result = Output::new(100, 150, 200, 50, "Rate:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = rate(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap(), inp2.value().parse::<f64>().unwrap());
                    let ans = format!("{:.2}", answer).to_string();
                    result.set_value(&(ans + "%"));
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    beven_but.handle(move |_beven_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 500, 300, "Break Even").center_screen();

            let inp0 = FloatInput::new(150, 0, 200, 50, "Revenue/unit:");
            let inp1 = FloatInput::new(150, 50, 200, 50, "Fixed Cost:");
            let inp2 = FloatInput::new(150, 100, 200, 50, "Variable Cost/unit:");
            let result = Output::new(150, 150, 200, 50, "Break Even:");
            let mut enter = Button::new(150, 200, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = breakeven(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap(), inp2.value().parse::<f64>().unwrap());
                    let ans = format!("{:.2}", answer).to_string();
                    result.set_value(&ans);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    profit_but.handle(move |_profit_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Profit").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Revenue:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Fixed Cost:");
            let inp2 = FloatInput::new(100, 100, 200, 50, "Variable Cost:");
            let result = Output::new(100, 150, 200, 50, "Profit:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = profit(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap(), inp2.value().parse::<f64>().unwrap());
                    let ans = format!("{:.2}", answer).to_string();
                    result.set_value(&("$".to_string() + &ans));
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    avg_but.handle(move |_avg_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Average Cost").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Variable Cost:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Fixed Cost:");
            let inp2 = FloatInput::new(100, 100, 200, 50, "Activity:");
            let result = Output::new(100, 150, 200, 50, "Average Cost:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = avgcost(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap(), inp2.value().parse::<f64>().unwrap());
                    let ans = format!("{:.2}", answer).to_string();
                    result.set_value(&("$".to_string() + &ans));
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    itb_but.handle(move |_itb_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Integer to Binary").center_screen();

            let inp0 = IntInput::new(100, 0, 200, 50, "Integer:");
            let result = Output::new(100, 50, 200, 50, "Binary:");
            let mut enter = Button::new(100, 100, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = inp0.value().parse::<i64>().unwrap();
                    let ans = format!("{:b}", answer);
                    result.set_value(&ans);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    ith_but.handle(move |_ith_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Integer to Hex").center_screen();

            let inp0 = IntInput::new(100, 0, 200, 50, "Integer:");
            let result = Output::new(100, 50, 200, 50, "Hex:");
            let mut enter = Button::new(100, 100, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = inp0.value().parse::<i64>().unwrap();
                    let ans = format!("{:X}", answer);
                    result.set_value(&ans);
                    true
                },
                _ => false,
            }});

            eq_win.end();
            eq_win.show();
            eq_win.set_color(Color::White);
            true
        },
        _ => false,
    }});

    win.end();
    win.show();

    win.set_color(Color::White);

    app.run().unwrap();
}
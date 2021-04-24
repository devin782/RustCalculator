mod math_functions;
use math_functions::*;
use fltk::prelude::*;
use fltk::enums::*;
use fltk::app::*;
use fltk::window::*;
use fltk::button::*;
use fltk::output::*;
use fltk::input::*;
use fltk::dialog::*;


fn main() 
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
    let mut add_but = Button::new(0, 20, 100, 20, "Addition");
    let mut sub_but = Button::new(0, 40, 100, 20, "Subtraction");
    let mut mult_but = Button::new(0, 60, 100, 20, "Multiplication");
    let mut div_but = Button::new(0, 80, 100, 20, "Division");
    let mut exp_but = Button::new(0, 100, 100, 20, "Exponent");
    let mut root_but = Button::new(0, 120, 100, 20, "Root");
    let mut quad_but = Button::new(0, 140, 100, 20, "Quadratic");
    let mut sine_but = Button::new(0, 160, 100, 20, "Sin/Cosec");
    let mut cosine_but = Button::new(0, 180, 100, 20, "Cos/Sec");
    let mut tangent_but = Button::new(0, 200, 100, 20, "Tan/Cotan");
    let mut der_but = Button::new(0, 220, 100, 20, "Derivative");

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
                            result.set_value(&(answer[0].to_string() + " and " + &answer[1].to_string()));
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

    win.end();
    win.show();

    win.set_color(Color::White);

    app.run().unwrap();
}
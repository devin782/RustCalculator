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
    let app = App::default();
    let mut win = Window::new(100, 100, 400, 300, "Calculator").center_screen();
    
    let out = "What calculation would you like?";
    let disp = Output::new(0, 0, 400, 20, "");
    disp.set_value(out);

    let mut add_but = Button::new(0, 20, 100, 20, "Addition");
    let mut sub_but = Button::new(0, 40, 100, 20, "Subtraction");
    let mut mult_but = Button::new(0, 60, 100, 20, "Multiplication");
    let mut div_but = Button::new(0, 80, 100, 20, "Division");
    let mut exp_but = Button::new(0, 100, 100, 20, "Exponent");
    let mut root_but = Button::new(0, 120, 100, 20, "Root");
    let mut quad_but = Button::new(0, 140, 100, 20, "Quadratic");
    let mut sine_but = Button::new(0, 160, 100, 20, "Sine/Cosecant");

    add_but.handle(move |_add_but, ev: Event| { match ev {
        Event::Push => {
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "X:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Y:");
            let result = Output::new(100, 100, 200, 50, "Result:");
            let mut enter = Button::new(100, 150, 200, 50, "Enter");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let answer = add(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
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
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

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
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

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
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

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
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

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
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

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
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

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
            let mut eq_win = Window::new(100, 100, 400, 300, "Addition").center_screen();

            let inp0 = FloatInput::new(100, 0, 200, 50, "Opposite:");
            let inp1 = FloatInput::new(100, 50, 200, 50, "Hypotenuse:");
            let s_result = Output::new(100, 100, 200, 50, "Sine:");
            let c_result = Output::new(100, 150, 200, 50, "Cosecant:");
            let mut enter = Button::new(100, 200, 200, 50, "Enter for Sine");

            enter.handle(move |_enter, num: Event| { match num {
                Event::Push => {
                    let s_answer = sine(inp0.value().parse::<f64>().unwrap(), inp1.value().parse::<f64>().unwrap()).to_string();
                    let c_answer = cosec(inp1.value().parse::<f64>().unwrap(), inp0.value().parse::<f64>().unwrap()).to_string();
                    s_result.set_value(&s_answer);
                    c_result.set_value(&c_answer);
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
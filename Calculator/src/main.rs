use std::io;
use std::cmp::Ordering;

//Addition function
fn add(num1: f64, num2: f64) -> f64{
    let buff: f64 = num1 + num2;
    println!("{}", buff);
    buff
}

//Substraction function
fn sub(num1: f64, num2: f64) -> f64{
    let buff: f64 = num1 - num2;
    println!("{}", buff);
    buff
}

//Multiplication function
fn mul(num1: f64, num2: f64) -> f64{
    let buff: f64 = num1 * num2;
    println!("{}", buff);
    buff
}

//Dividing function
fn div(num1: f64, num2: f64) ->  f64{
    if num2 == 0.0 {
        println!("Can't divide by zero!");
        0.0
    } else {
        let buff: f64 = num1 / num2;
        println!("{}", buff);
        buff
    }
}

//Do square function
fn sqr(num: f64) -> f64{
    let buff: f64 = num * num;
    println!("{}", buff);
    buff
}

//Do power function
fn pow(num1: f64, num2: f64) -> f64{
    let mut buff: f64 = 1.0;
    for _ in 0..num2 as usize {
        buff = buff * num1;
    }

    println!("{}", buff);
    buff
}

//Do root function
fn root(num: f64) -> f64 {
    let mut i: f64 = 0.0;
    let mut result: f64 = 0.0;

    while i * i <= num {
        result = i;
        i += 0.001;
    }

    println!("{}", result);
    result
}

//Do logarithm function
fn log(num1: f64, num2: f64){

}

//Do logarithm base 10 function
fn log10(num1: f64){

}

//Do sinus function
fn sin(){

}

//Do cosinus function
fn cos(){

}

//Do tangens function
fn tn(){

}

//Do cotangents function
fn ctn(){

}

//Do factorial function
fn factorial(){

}

//Do tetration function
fn tetration(){

}

//Argument parser
fn arg_parse(input: String) -> f64 {
    let parts: Vec<&str> = input.split_whitespace().collect();

    let left_num: f64 = parts[0].parse().unwrap();
    let operator = parts[1];
    let right_num: f64 = parts[2].parse().unwrap();

    match operator {
        "+" => add( left_num, right_num),
        "-" => sub( left_num, right_num),
        "*" => mul( left_num, right_num),
        "/" => div( left_num, right_num),
        _ => panic!("Unknown operator"),
    }
}

//Main program function
fn main() {
    let mut string: String = String::new();

    let _ = io::stdin().read_line(&mut string);
    arg_parse(string);

}

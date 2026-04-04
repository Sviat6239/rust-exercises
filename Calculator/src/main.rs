use std::io;
use std::cmp::Ordering;


fn add(num1: f64, num2: f64){
    let buff: f64 = num1 + num2;
    println!("{}", buff);
}

fn sub(num1: f64, num2: f64){
    let buff: f64 = num1 - num2;
    println!("{}", buff);
}

fn mul(num1: f64, num2: f64){
    let buff: f64 = num1 * num2;
    println!("{}", buff);
}

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

fn sqr(num: f64){
    let buff: f64 = num * num;
    println!("{}", buff);
}

fn pow(num1: f64, num2: f64){
    let mut buff: f64 = 1.0;
    for _ in 0..num2 as usize {
        buff = buff * num1;
    }

    println!("{}", buff);
}

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


fn main() {
    root(23.0);
    add(2.0, 19.0);
    div(4.0, 53.0);
    sub(23.0, 17.0);
    mul(3.0, 11.0);
    pow(23.0, 11.0);
    sqr(14.074);
}

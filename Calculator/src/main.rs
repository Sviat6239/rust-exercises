use std::io;
use std::cmp::Ordering;


fn add(num1: i64, num2: i64){
    let buff: i64 = num1 + num2;
    println!("{}", buff);
}

fn sub(num1: i64, num2: i64){
    let buff: i64 = num1 - num2;
    println!("{}", buff);
}

fn mul(num1: i64, num2: i64){
    let buff: i64 = num1 * num2;
    println!("{}", buff);
}

fn div(num1: i64, num2: i64) ->  i64{
    if num2 == 0{
        println!("Can't divide by zero!");
        0
    } else {
        let buff = num1 / num2;
        println!("{}", buff);
        buff
    }
}

fn sqr(num: i64){
    let buff: i64 = num * num;
    println!("{}", buff);
}

fn pow(num1: i64, num2: i64){
    let mut buff: i64 = 1;
    for _ in 0..num2{
        buff = buff * num1;
    }

    println!("{}", buff);
}

fn root(num: i64) -> i64 {
    let mut i: i64 = 0;
    let mut result: i64 = 0;

    while i * i <= num {
        result = i;
        i += 1;
    }

    println!("{}", result);
    result
}


fn main() {
    root(23);
    add(2, 19);
    div(4, 53);
    sub(23, 17);
    mul(3, 11);
    pow(23, 11);
    sqr(14);
}

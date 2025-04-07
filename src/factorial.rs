use std::result::Result;

fn print_factorial(number: i64) {
    println!("Factorial is {number}");
}

pub fn get_factorial(number: i32) -> Result<(), &'static str> {
    //de reference number and cast into i64
    //factorials are going to get large
    let x:i64 = number.into();

    //return edge cases
    if x < 0 {
        return Err("number must be >= 0")
    }

    //algorithm
    let mut result: i64 = 1;
    println!("Calculating factorial of {x}");

    if x <= 1 {
    //bounce out if number is too big
        print_factorial(result);
        return Ok(())
    }else if x > 20 {
        return Err("number too big, will overflow.")
    }

    let mut count: i64 = x.clone();

    while count > 0{
        result = result*count;
        count = count -1;
    }

    print_factorial(result);
    //Return OK
    Ok(())
}
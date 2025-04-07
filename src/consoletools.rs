pub fn triangle(number: i32, invert: bool)->Result<(), &'static str> {
    for x in 0..number+1 {
        let mut ln  = String::from("");
        let mut x2 = x.clone();
        if invert {
            x2 = number - x;
        }
        for _i in 0..x2 {
            ln = ln + "*";
        }
        println!("{ln}");
    }
    Ok(())

}
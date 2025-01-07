use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::io::ErrorKind;
//use std::io::Error;

fn main() {
    println!("Hello, World!");
    load_vec();


}

fn load_vec() -> std::io::Result<()> {
    let mut res: u32 = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();
        let values: Vec<i32> = values.iter() 
            .map(|&s| s.parse::<i32>()) 
                .collect::<Result<Vec<_>, _>>()?;
        if check_line(values) == true {
            res += 1;
        }
    }
    return Ok(());
}

fn check_line(val: Vec<&str>) -> bool {
    let chk = val[0] - val[1];
    let chk: bool = if chk > 0 {true} else if chk < 0 {false} else {return false}; 
            for i in 0..val.len()-1 {`
        let t = val[i] - val[i+1];
        if t < 0 || t > 3 {
            return false;
        }
        // here check if it is incr or decr
        continue;
    }
    return true;
}

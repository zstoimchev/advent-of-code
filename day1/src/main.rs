use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::ErrorKind;
use std::io::Error;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    load_vec(&mut v1, &mut v2).expect("probljem loding vektors");
    
    println!("Part II: {}", calc_simil(&v1, &v2));
    println!("Part I:  {}", calc_res(&mut v1, &mut v2));
}

fn calc_simil(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut res = 0;

    for i in 0..v1.len() {
        let mut c = 0;
        for j in 0..v2.len() {
            if v1[i] == v2[j] {
                c += 1;
            }
        }
        res += c*v1[i];
    }

    return res;
}

fn calc_res(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> i32 {
    let mut res: i32 = 0;

    while v1.len() > 0 && v2.len() > 0 {
        let mut max1 = 0;
        let mut max2 = 0;
        for i in 1..v1.len() as usize {
            if v1[i] < v1[max1] {
                max1 = i;
            }
            if v2[i] < v2[max2] {
                max2 = i;
            }
        }
        res += (v1[max1] - v2[max2]).abs();
        v1.remove(max1);
        v2.remove(max2);
    }
    return res;
}

fn load_vec(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();

        if values.len() == 2 { 
            if let (Ok(val1), Ok(val2)) = (values[0].parse::<i32>(), values[1].parse::<i32>()) { 
                v1.push(val1); 
                v2.push(val2); 
            } else {
                return Err(Error::new(ErrorKind::Other, ""));
            } 
        }
    }
    return Ok(());
}

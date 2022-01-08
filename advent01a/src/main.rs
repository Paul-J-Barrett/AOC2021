use std::path::Path;
use std::io::BufReader;
use std::file::File;

fn main() {
    let mut count:i32=0;
    let mut previous:i32 = i32::MAX;
    //readfile in and split values on newlines input file is input.txt
    let mut _lines:Vec<String>=BufReader::new(File::open(Path::new("input.txt"))).unwrap().lines();
    for (i, v) in  _lines.into_iter().enumerate() {
        if i==0 { continue }
        if let Some(x)=Ok(Ok(v.parse::<i32>();

        prious=x;
    }
    //use itertools to find out how many increases there are from previous value 1st value is not an increase

}

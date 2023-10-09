use serde::{Serialize, Deserialize};
use std::fs::File;
use bincode::serialize_into;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TwoVecs {
    x: Vec<f32>,
    y: Vec<f32>,
}

// Minimal example of serializing to binary, writing to file, opening & reading the file, and deserializing the data.
// See 'big' branch for tests of larger data structure.

fn main() {
    // define a large amount of data in a TwoVecs struct
    let mut my_struct = TwoVecs{x : vec![], y : vec![]};

    for i in 0..1000000 {
        my_struct.x.push(i as f32);
        my_struct.y.push((i + 1) as f32);
    }

    // write the data to a file
    let mut file_to_write_to = BufWriter::new(File::create("big.dat").unwrap());
    serialize_into(&mut file_to_write_to, &my_struct).unwrap();

    // drop the open file from scope
    drop(file_to_write_to);

    // read the data back from the file
    let mut file_to_read_from = BufReader::new(File::open("big.dat").unwrap());
    let deserialized_struct : TwoVecs = bincode::deserialize_from(&mut file_to_read_from).unwrap();

    println!("{:?}", deserialized_struct.x[999999]);
}
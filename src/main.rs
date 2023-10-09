use serde::{Serialize, Deserialize};
use std::fs::File;
use bincode::serialize_into;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Pair {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MyStruct(Vec<Pair>);

// Minimal example of serializing to binary, writing to file, opening & reading the file, and deserializing the data.
// See 'big' branch for tests of larger data structure.

fn main() {
    // define some data in a MyStruct struct
    let my_struct = MyStruct(vec![Pair { x: 0.0, y: 8.0 }, Pair { x: 10.0, y: 20.5 }]);

    // write the data to a file
    let mut file_to_write_to = BufWriter::new(File::create("foo.dat").unwrap());
    serialize_into(&mut file_to_write_to, &my_struct).unwrap();

    // drop the open file from scope
    drop(file_to_write_to);

    // read the data back from the file
    let mut file_to_read_from = BufReader::new(File::open("foo.dat").unwrap());
    let deserialized_struct : MyStruct = bincode::deserialize_from(&mut file_to_read_from).unwrap();

    println!("{:?}", deserialized_struct)
}
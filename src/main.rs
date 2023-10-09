use serde::{Serialize, Deserialize};
use std::fs::File;
use bincode::serialize_into;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

// minimal example of serializing to binary, writing to file, opening & reading the file, and deserializing the data.
// see other branch for tests of larger data structure

fn main() {
    // define some data in a World struct
    let m = World(vec![Entity { x: 0.0, y: 8.0 }, Entity { x: 10.0, y: 20.5 }]);

    // write the data to a file
    let mut f = BufWriter::new(File::create("foo.dat").unwrap());
    serialize_into(&mut f, &m).unwrap();

    // drop the open file from scope
    drop(f);

    // read the data back from the file
    let mut g = BufReader::new(File::open("foo.dat").unwrap());
    let decoded : World = bincode::deserialize_from(&mut g).unwrap();

    println!("{:?}", decoded)
}
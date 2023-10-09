### Branches

`master` - minimal example of serializing to binary, writing to file, opening & reading the file, and deserializing the data.  
`big` - tests of larger data structure

Not benchmarked, but saving 2 vecs of 1 million floats, in a struct, to file ...
```Rust
struct TwoVecs {
    x: Vec<f32>,
    y: Vec<f32>,
}
```
... then reading the data back from file and deserializing, takes less than second, and the resulting file is just 8MB.
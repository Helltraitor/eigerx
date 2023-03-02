// Good stream solution
mod stream;

// Bad overflow-possible recuirsive solution
mod recursion;


fn main() {
    // Let assume that this vector is our stream
    // NOTE: Since Rust use [`Option::None`], 0 at the end is redundant
    let stream = vec![1, 5, 42, -376, 5, 19, 5, 3, 42, 2];

    println!("Max value of stream is {:?}", stream::from_stream(stream.clone().into_iter()));
    println!("Max value of stream is {:?}", recursion::from_stream(stream.into_iter()));
}

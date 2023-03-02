use std::cmp::Ordering;

/// Counts max value and its freqs value by value
/// Can process any value that can be compare
pub fn stream_max<T: Ord>(mut stream: impl Iterator<Item = T>) -> Option<(T, usize)> {
    fn stream_max_processor<T: Ord>(mut stream: impl Iterator<Item = T>, max: T, count: usize) -> Option<(T, usize)> {
        match stream.next() {
            // Stream is ended
            None => Some((max, count)),
            Some(value) => match max.cmp(&value) {
                Ordering::Less => stream_max_processor(stream, value, 1),
                Ordering::Greater => stream_max_processor(stream, max, count),
                Ordering::Equal => stream_max_processor(stream, max, count + 1),
            }
        }
    }

    match stream.next() {
        // Empty stream was given
        None => None,
        Some(value) => stream_max_processor(stream, value, 1)
    }
}

// Option is required since stream can be empty
pub fn from_stream<T: Ord>(stream: impl Iterator<Item = T>) -> Option<(T, usize)> {
    stream_max(stream)
}

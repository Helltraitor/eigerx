use std::cmp::Ordering;

// This struct is wrapper over stream element and it's count
#[derive(Debug)]
pub struct StreamStatistics<T: Ord> {
    value: T,
    count: usize
}

/// Counts max value and its freqs value by value
/// Can process any value that can be compare
pub fn stream_max<T: Ord>(value: T, stats: StreamStatistics<T>) -> StreamStatistics<T> {
    match stats.value.cmp(&value) {
        Ordering::Less => StreamStatistics { value, count: 1 },
        Ordering::Greater => stats,
        Ordering::Equal => StreamStatistics { value: stats.value, count: stats.count + 1 },
    }
}

// Option is required since stream can be empty
pub fn from_stream<T: Ord>(stream: impl Iterator<Item = T>) -> Option<(T, usize)> {
    let mut stats = None;
    // This is just receiver from the stream. It will work untill it get None (after `2` element)
    for item in stream {
        stats = match stats {
            None => Some(StreamStatistics { value: item, count: 0 }),
            Some(stats) => Some(stream_max(item, stats))
        }
    }

    stats.map(|StreamStatistics { value, count }| (value, count))
}

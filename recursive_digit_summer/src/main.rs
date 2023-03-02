/// Public interface to digit summer
fn recursive_digit_summer(number: usize) -> usize {
    /// Real function must operates with number as pieces: 100 -> 10 and 0
    /// So only one solution is two have decoraton that hides real recursive solution
    fn digit_summer(current: usize, sum: usize) -> usize {
        match current {
            0 => sum,
            other => other % 10 + digit_summer(other / 10, sum)
        }
    }

    digit_summer(number, 0)
}


fn main() {
    println!("Value {}, Result {}, Expect {}", 2347623, recursive_digit_summer(2347623), 27);
    println!("Value {}, Result {}, Expect {}", 86876757, recursive_digit_summer(86876757), 54);
}

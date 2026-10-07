const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];
}

fn average_temp(log: &Vec<i32>) -> f64 {
   // Check if log size > 0
   if log.is_empty() {
      return 0.0; // RETURN 0.0 if FALSE
   }
let log_size = log.len() as f64;
    let mut total: f64 = 0.0; // total: FLOAT = 0.0

    // for entry in log
    for &entry in log {
        total = total + entry as f64; // total = total + entry[cite: 3]
    }

    // RETURN total / log size[cite: 3]
    total / log_size
}

fn hottest_day(log: &Vec<i32>) -> usize {
    if log.is_empty() {
        return 0; // RETURN 0 if log size check fails[cite: 5]
    }

    let mut highest = log[0] as f64; // highest: FLOAT = log at 0[cite: 5]
    let mut index: usize = 0;       // index: USIZE = 0[cite: 5]

    for (i, &entry) in log.iter().enumerate() {
        if highest < entry as f64 { // highest < entry?[cite: 5]
            highest = entry as f64; // highest = entry[cite: 5]
            index = i;              // index = i[cite: 5]
        }
    }

    index // RETURN index[cite: 5]
}

fn count_above(log: &[i32], threshold: i32) -> i32 {
    let mut count = 0; // count: INTEGER

    for &entry in log { // for entry in log
        if entry > threshold { // entry > threshold -> TRUE
            count += 1; // count = count + 1
        }
    }

    count // RETURN count
}

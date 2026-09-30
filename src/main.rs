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

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}


fn main() {
    let mut log: Vec<Reading> = Vec::new();

    for (day, &high) in DAYS.iter().zip([72, 68, 75, 81, 79, 83, 76].iter()) {
        log.push(Reading {
            day: day.to_string(),
            high,
        });
    }

    for entry in &log {
    println!("{}: {}", entry.day, entry.high);
}

    println!("Average high: {:.1}", average_temp(&log));
    println!("Hottest day: {}", log[hottest_day(&log)].day);
    println!("Days above 75: {}", count_above(&log, 75));
}

fn average_temp(log: &Vec<Reading>) -> f64 {
   
   if log.is_empty() {
      return 0.0; 
   }
let log_size = log.len() as f64;
    let mut total: f64 = 0.0; 

    
    for entry in log {
        total = total + entry.high as f64; 
    }

   
    total / log_size
}

fn hottest_day(log: &Vec<Reading>) -> usize {
    if log.is_empty() {
        return 0; 
    }

    let mut highest = log[0].high as f64; 
    let mut index = 0;       

    for (i, entry) in log.iter().enumerate() {
        if highest < entry.high as f64 { 
            highest = entry.high as f64; 
            index = i;             
        }
    }

    index 
}

fn count_above(log: &Vec<Reading>, threshold: i32) -> i32 {
    let mut count = 0; 

    for entry in log {
        if entry.high > threshold {
            count += 1;
        }
    }

    count 
}




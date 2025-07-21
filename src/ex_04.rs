// struct Counter {
//     count: i32,
// }

// impl Counter {
//     fn increment(&self) -> i32 {
//          self.count += 1 ;
//          return self.count;
//     }
// }

// fn main() {
//     let mut counter = Counter { count: 0 };
//     counter.increment(); 
//     println!("Counter: {}", counter.count);
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_counter_count() {
//         let mut counter = Counter { count: 0 };
//         assert_eq!(counter.increment(), 1);
//         assert_eq!(counter.increment(), 1);
//     }
// } 
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

struct Cache {
    values: RefCell<HashMap<i32, bool>>,
    order: RefCell<VecDeque<i32>>,
    capacity: usize,
}

impl Cache {
    fn new(capacity: usize) -> Self {
        Cache {
            values: RefCell::new(HashMap::new()),
            order: RefCell::new(VecDeque::new()),
            capacity,
        }
    }
    fn is_prime(&self, num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        for i in 2..=(num / 2) {
            if num % i == 0 {
                return false;
            }
        }
        return true;
    }

    fn get_or_insert(&self, num: i32) -> bool {
        let mut cache = self.values.borrow_mut();
        let mut order = self.order.borrow_mut();

        if let Some(_) = cache.get(&num) {
            order.retain(|&x| x != num);
        } else {
            let result = self.is_prime(num);
            cache.insert(num, result);

            if order.len() >= self.capacity {
                let removed = order.pop_back().unwrap();
                cache.remove(&removed);
            }
        }

        order.push_front(num);
        cache[&num]
    }
}

fn main() {
    let cache = Cache::new(10); // maximum capacity

    loop {
        let mut input = String::new();
        println!("Write a number (or 'exit' to close): ");
        let _ = std::io::stdin().read_line(&mut input);

        if input.trim() == "exit" {
            break;
        }

        match input.trim().parse::<i32>() {
            Ok(num) => {
                let is_prime = cache.get_or_insert(num);
                println!("{} is prime: {}", num, is_prime);
            }
            Err(_) => println!("Not a valid number!"),
        }
    }
}

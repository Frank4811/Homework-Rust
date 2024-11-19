//Module 7
    //task 1 :3
    
    fn main() {
        let operation = |a: i32, b: i32| {
            // Your implementation here
            a*b
        };
    
        println!("Result: {}", operation(10, 5));
    }
     
     /*   
    //Module 7
    //task 2 :3
    fn track_changes() {
        let mut tracker = 0;
        let mut update = || {
            // Your implementation here
            tracker +=2;
            println!("Tracker: {}", tracker);
        };
    
        update();
        update();
    }
    
    fn main() {
        track_changes();
    }
        */
/*
    //Module 7
    // task 3 :3
    fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
        where
            F: Fn(i32) -> i32,
        {
            // Your implementation here
            vec.into_iter().map(f).collect()
        }

    fn main() {
        let numbers = vec![1, 2, 3];

        let doubled = process_vector(numbers.clone(), |x| {
            // Implement: multiply each number by 2
            x * 2
        });

        let replaced = process_vector(numbers, |x| {
            // Implement: if number > 2, replace with 0, else keep number
            if x > 2 {
                0
            } else {x}
        });

        println!("Doubled: {:?}", doubled);
        println!("Replaced: {:?}", replaced);
    }
        */
/*
    //Module 7
    // task 4
    use std::{thread, time::Duration};

    struct ComputeCache<T>
    where
        T: Fn() -> String,
    {
        computation: T,
        result: Option<String>,
    }

    impl<T> ComputeCache<T>
    where
        T: Fn() -> String,
    {
        fn new(computation: T) -> Self {
            ComputeCache {
                computation,
                result: None,
            }
        }

        fn get_result(&mut self) -> String {
            match &self.result {
                Some(r) => {
                    println!("Retrieved from cache instantly!");
                    r.clone()
                }
                None => {
                    println!("Computing (this will take 2 seconds)...");
                    thread::sleep(Duration::from_secs(2));
                    let r = (self.computation)();
                    self.result = Some(r.clone());
                    r
                }
            }
        }
    }

    fn main() {
        let mut cache = ComputeCache::new(|| {
            println!("Computing (this will take 2 seconds)...");
            thread::sleep(Duration::from_secs(2));
            "Hello, world!".to_string()
        });

        println!("First call:");
        println!("Result: {}", cache.get_result());
        
        println!("\nSecond call:");
        println!("Result (cached): {}", cache.get_result());
    }
*/
use std::process;

fn main() {
    if let Err(e) = workout::run() {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}

mod workout {
    use std::error::Error;
    use std::thread;
    use std::time::Duration;

    // fn simulated_expensive_calculation(intensity: u32) ->u32 {
    //     println!("Calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }

    pub fn run() -> Result<(), Box<dyn Error>> {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        // generate_workout(simulated_user_specified_value, simulated_random_number);
        let m = "demo";
        test_char_closure(m);
        Ok(())
    }

    use std::collections::HashMap;
    use std::ops::Fn;

    struct Cacher<K, V, T>
    where
        T: Fn(K) -> V,
        K: std::cmp::Eq + std::hash::Hash,
    {
        calculation: T,
        values: HashMap<K, V>,
    }

    use std::collections::hash_map::Entry;

    impl<K, V, T> Cacher<K, V, T>
    where
        T: Fn(K) -> V,
        K: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug,
        V: std::fmt::Debug,
    {
        fn new<E, F>(calculation: T) -> Cacher<E, F, T>
        where
            T: Fn(E) -> F,
            E: std::cmp::Eq + std::hash::Hash,
        {
            Cacher {
                calculation,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: &K) -> &V {
            /* eprintln!("arg: {:?}", *arg);
            let e = self.values.entry(*arg).or_insert((self.calculation)(*arg));
            e
            */
            if let Entry::Vacant(o) = self.values.entry(*arg) {
                o.insert((self.calculation)(*arg));
            }
            // HashMap 入参为Key, 闭包为Value
            &self.values[arg]
        }
    }
    // 类型为Fn(&str) -> u32
    fn test_char_closure(m: &str) {
        let mut cacher_value = Cacher::new(|m: &str| m.len());
        println!("this slice's length is {}", cacher_value.value(&m))
    }

    // 类型为Fn(u32) -> u32
    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_closure = Cacher::new(|intensity| {
            println!("Calculating slowly...");
            thread::sleep(Duration::from_secs(5));
            intensity
        });
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure.value(&intensity));
            println!("Next, do {} situps!", expensive_closure.value(&intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_closure.value(&intensity)
                );
            }
        }
    }
}

use std::time::Instant;

// #[derive(Copy, Clone)]
struct StructPrimeNumber {
    prime: i32,
    number: i32,
}



fn primenumber(max: i32) -> Vec<i32> {
    let mut result_array: Vec<i32> = Vec::new();
    let mut prime_array: Vec<StructPrimeNumber> = Vec::new();
    prime_array.push(StructPrimeNumber {
        prime: 2,
        number: 4,
    });
    result_array.push(2);
    
    for i in 3..(4.max(max)) {
        if prime_array[0].number > i {
            result_array.push(i);
            if i * i < max {
                prime_array.push(StructPrimeNumber {
                    prime: i,
                    number: i * i,
                })
            }
        } else {
            prime_array[0].number += prime_array[0].prime;
            let el: StructPrimeNumber = prime_array.remove(0);
            let index = prime_array.partition_point(|x| x.number < el.number);
            prime_array.insert(index, el);
            if prime_array[0].number <= i {
                prime_array[0].number += prime_array[0].prime;
                let el = prime_array.remove(0);
                let index = prime_array.partition_point(|x| x.number < el.number);
                prime_array.insert(index, el);
            }
        }
    }
    return result_array;
}

fn main() {
    let now = Instant::now();
    let p = primenumber(1000);
    let elapsed = now.elapsed();
    println!("{:?}", p);
    println!("Elapsed: {:.2?}", elapsed);
    println!("Hello, world!");
}

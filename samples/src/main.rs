use num::Integer;
use std::time::Instant;

//The divisor function from the reimplementation part of the project. 
fn divisors(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();

    for number in 2..n / 2 + 1 {
        if n % number == 0 {
            vec.push(number)
        }
    }
    return vec;
}

//The Primes function from the reimplementation part of the project.
//Used for benchmarking the concurrent version of Primes.
//Calculates in 1669 milliseconds.  
fn primes(n: u32) -> Vec<u32> {
    let now = Instant::now();

    let mut vec = Vec::new();

    for number in 2..n + 1 {
        if divisors(number).is_empty() {
            vec.push(number)
        }
    }
    println!("Primes: {}", now.elapsed().as_millis());
    return vec;
}

//Conccurent version of Primes.
//A thread will calculate half the number of primes in the background, while the main thread calculates the other half
//Calculates in 1129 milliseconds.
fn primesconcurrent(n: u32) -> Vec<u32> {
    let now = Instant::now();
    let handle1 = std::thread::spawn(move || {
        let mut vec = Vec::new();
        for number in 2..n / 2 {
            if divisors(number).is_empty() {
                vec.push(number)
            }
        }
        return vec;
    });
    let mut vec = Vec::new();
    for number in n / 2..n + 1 {
        if divisors(number).is_empty() {
            vec.push(number)
        }
    }
    let mut rvec = handle1.join().unwrap();
    rvec.append(&mut vec);
    println!("Concurrent: {}", now.elapsed().as_millis());
    return rvec;
}

//Implementation of the succ method from Haskell.
//Input can be of any integer type (i/u 32/64...)
//Does not return the result array, but manipulates the original array.
fn succ<T: Integer + Copy>(lst: &mut [T]) {
    for element in lst.iter_mut() {
        *element = *element + T::one();
    }
}

//Finds all indices of lst that match n.
//Returns None if no index matches n
//Returns Some, as well as the resulting indices, if there are matches, 
fn find_all(lst: &[u32], n: u32) -> Option<Vec<usize>> {
    let mut vec = Vec::new();
    let mut il = 0;
    for (i, e) in lst.iter().enumerate() {
        if *e == n {
            il = 1;
            vec.push(i);
        }
    }
    if il == 1 {
        return Some(vec);
    } else {
        return None;
    }
}

//Main Entry point of the function.
fn main() {

    let a = primes(10000);
    let b = primesconcurrent(10000);
    assert_eq!(a, b); //Checks that the 2 ararys are eqaul. 
    
    let mut lst = [1, 2, 3, 4, 5, 6, 7];
    succ(&mut lst); //Pass a reference to lst, therefore giving ownership to the succ function
    println!("{:?}", lst);//Prints all elements of lst
    
    let lst = [1, 1, 2, 3, 5, 6, 7];
    let result = find_all(&lst, 8);
    match result {
        Some(x) => println!("{:?}", x), //There are matching elements.
        None => println!("Value not found"), //No elements match 8
    }
}

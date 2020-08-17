use map_for::map_for; //Used to find the pythagorean triples.

//Takes an unsigned 32 bit integer and returns a list of its divisors.
fn divisors(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();

    for number in 2..n / 2 + 1 {
        if n % number == 0 {
            vec.push(number)
        }
    }
    return vec;
}

//Takes an unsigned 32bit integer and returns a list of all prime numbers upto it.
fn primes(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();

    for number in 2..n + 1 {
        if divisors(number).is_empty() {
            vec.push(number)
        }
    }
    return vec;
}

//Takes a String and a seperator and joins them using a buil-in Rust funtion.
fn join(sep: String, list: &[String]) -> String {
    return list.join(&sep);
}

//Takes an unsigned 32 bit integer and finds all pythogorean triples up to c == max.
fn pythagorean(max: u32) -> Vec<(u32, u32, u32)> {
    let mut vec = Vec::new();

    let iter = map_for! {
        move;
        a <- 1..max+1;
        b <- 1..max+1;
        c <- 1..max+1;
        if ((a*a)+(b*b)) == (c*c);
        => (a,b,c)
    };
    for x in iter {
        vec.push(x);
        let mut element = 0;
        while element < vec.len() {
            let (a, b, _) = x;
            let (i, j, _) = vec[element];
            if a == j && b == i {
                vec.remove(element);
            } else {
                element += 1;
            }
        }
    }
    return vec;
}

//Takes an unsigned 32 bit integer and returns the next value in the hailstone sequence.
fn hailstone(n: u32) -> u32 {
    if n % 2 == 0 {
        return n / 2;
    } else {
        return 3 * n + 1;
    }
}

//Takes an unsinged 32 bit ingeteger and returns all the values in the hailstone sequence starting with n.
fn hail_seq(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    let mut i = n;

    while i != 1 {
        vec.push(i);
        i = hailstone(i);
    }
    vec.push(1);
    return vec;
}

//Takes an array of Generic type T and sorts it.
fn mergesort<T: Ord + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    let len = arr.len();
    if mid == 0 {
        return;
    }
    mergesort(&mut arr[0..mid]);
    mergesort(&mut arr[mid..len]);

    let mut res = arr.to_vec();
    merge(&arr[0..mid], &arr[mid..arr.len()], &mut res[..]);
    arr.copy_from_slice(&res);
}

//Used in mergesort to merge 2 arrays together.
fn merge<T: Ord + Copy>(arr1: &[T], arr2: &[T], arr3: &mut [T]) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    while a < arr1.len() && b < arr2.len() {
        if arr1[a] < arr2[b] {
            arr3[c] = arr1[a];
            c += 1;
            a += 1;
        } else {
            arr3[c] = arr2[b];
            b += 1;
            c += 1;
        }
    }
    if a < arr1.len() {
        arr3[c..].copy_from_slice(&arr1[a..]);
    }
    if b < arr2.len() {
        arr3[c..].copy_from_slice(&arr2[b..]);
    }
}

//Calculates the fibonacci sequence.
fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c;
    for _ in 1..n {
        c = a + b;
        a = b;
        b = c;
    }

    return b;
}

//Main entry point of the program
//To run each function and print the result, remove the underscore(_) before the function name and uncomment the print statement after it.
fn main() {
    let _primes = primes(100);
    // println!("{:?}", primes);
    let _divs = divisors(30);
    // println!("{:?}", divs);
    let _joins = join(
        "+".to_string(),
        &["1".to_string(), "2".to_string(), "3".to_string()],
    );
    // println!("{}", joins);
    let _pyth = pythagorean(30);
    // println!("{:?}", pyth);

    let _hail = hailstone(31);
    // println!("{}", hail);

    let _hails = hail_seq(11);
    // println!("{:?}", hails);

    let mut arr = [6, 2, 4, 8, 9, 5, 3, 1, 7, 10]; //No underscore for this function, just uncomment the print statement to see results.
    mergesort(&mut arr);
    // println!("{:?}", arr);

    let _f = fib(47);
    // println!("{}", f);
}

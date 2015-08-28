use std::collections::HashMap;

fn problem1() {
    println!("\nProblem 1");
    let mut accum: i32 = 0;
    for i in 1..1000 {
        match i {
            x if i % 3 == 0 => accum += x,
            x if i % 5 == 0 => accum += x,
            _ => ()
        }
    }
    println!("sum of multiples is {}", accum);
}

fn fib(x: i32) -> i32 {
    if x < 2 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn problem2() {
    println!("\nProblem 2");
    let mut i = 2;
    let mut x;
    let mut accum = 0;
    loop {
        x = fib(i);
        if x >= 4000000 {
            break;
        }
        i += 1;
        if x % 2 == 0 {
            accum += x;
        }
    }
    println!("sum of even valued terms is {}",accum);
}

fn factor(x: i64) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut current = x;
    let mut i: i64 = 2;
    while current > 1 && i <= x/2 {
        if current % i == 0 {
            current = current / i;
            result.push(i);
            //println!("found factor {}, remaining: {}",i,current);
            i = 1;
        }
        i += 1;
    }
    if current > 1 {
        result.push(current);
    }
    return result;
}

fn problem3() {
    println!("\nProblem 3");
    let number: i64 = 600851475143;
    let result = factor(number);
    println!("prime factors of {} are:",number);
    for i in result {
        print!("{} ",i);
    }
}

fn is_palindrome(x: i32) -> bool {
    //first split out our digits
    let mut digits: Vec<i32> = Vec::new();
    let mut tmp = x;
    while tmp > 0 {
        digits.push(tmp % 10);
        tmp = tmp / 10;
    }
    let mut first = 0;
    let mut last = digits.len() - 1;
    while last > first {
        if digits[first] != digits[last] {
            return false;
        }
        first += 1;
        last -= 1;
    }
    return true;
}

fn problem4() {
    println!("\nProblem 4");
    let mut largest = 0;
    let mut lx = 0;
    let mut ly = 0;
    for x in 100..1000 {
        for y in 100..1000 {
            if is_palindrome(x * y) {
                if (x * y) > largest {
                    largest = x * y;
                    lx = x;
                    ly = y;
                }
            }
        }
    }
    println!("largest 3 digit product palindrome is {} * {} = {}",lx,ly,largest);
}

//  The lazy way to solve this would be to check every number until we find one that fits,
//  but that's slow and boring.
//
//  A much more elegant method is to determine it by finding the prime factorization of each
//  of the contributors, and then multiply the union of the factors. Thanks to problem 3 we
//  already have a foctor() function we can use.
fn problem5() {
    println!("\nProblem 5");
    let mut allFactors: HashMap<i64,u32> = HashMap::new();
    for i in 1..21 {
        //determine the prime factors and store the number of occurrences of each factor
        let mut factors: HashMap<i64,u32> = HashMap::new();
        for f in factor(i) {
            let e = factors.entry(f).or_insert(0);
            *e += 1;
        }
        // add the prime factorization of this number to our total
        for (key,val) in factors.iter() {
            let entry = allFactors.entry(*key).or_insert(*val);
            if *entry < *val {
                *entry = *val;
            }
        }
    }
    //now we have all the prime factors in the union, multiply them all to get the least
    //common multiple of all of our numbers, which is the answer to the question
    let mut lcm: i64 = 1;
    for (key,val) in allFactors.iter() {
        let contrib = i64::pow(*key,*val);
        lcm *= contrib;
    }
    println!("smallest number divisible by 1 to 20 is {}", lcm);
}

fn problem6() {
    println!("\nProblem 6");
    let mut sumsq: i64 = 0;
    let mut sum: i64 = 0;
    for i in 1..101 {
        sum += i;
        sumsq += i * i;
    }
    let sqsum = sum * sum;
    println!("{} - {} = {}", sqsum, sumsq, sqsum - sumsq);
}

fn main() {
    problem1();
    problem2();
    problem3();
    problem4();
    problem5();
    problem6();
}

use std::collections::HashMap;
use std::cmp::max;

pub fn problem1() {
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

pub fn problem2() {
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

pub fn problem3() {
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

pub fn problem4() {
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
pub fn problem5() {
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

pub fn problem6() {
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

pub fn problem7() {
    println!("\nProblem 7");
    let mut count = 0;
    let mut prime = 1;
    let target_prime = 10001;
    let mut i = 2;
    while count < target_prime {
        prime += 1;
        let mut is_prime = prime == 2 || prime % 2 == 1;
        let max = (f32::sqrt(prime as f32)+1.0f32) as i64;
        for i in 2..max {
            if prime % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            //print!("{} ",prime);
            count += 1;
        }
    }
    println!("the {} prime is {}", target_prime, prime);
}

pub fn problem8() {
    println!("\nProblem 8");
    let theNumber = "73167176531330624919225119674426574742355349194934\
96983520312774506326239578318016984801869478851843\
85861560789112949495459501737958331952853208805511\
12540698747158523863050715693290963295227443043557\
66896648950445244523161731856403098711121722383113\
62229893423380308135336276614282806444486645238749\
30358907296290491560440772390713810515859307960866\
70172427121883998797908792274921901699720888093776\
65727333001053367881220235421809751254540594752243\
52584907711670556013604839586446706324415722155397\
53697817977846174064955149290862569321978468622482\
83972241375657056057490261407972968652414535100474\
82166370484403199890008895243450658541227588666881\
16427171479924442928230863465674813919123162824586\
17866458359124566529476545682848912883142607690042\
24219022671055626321111109370544217506941658960408\
07198403850962455444362981230987879927244284909188\
84580156166097919133875499200524063689912560717606\
05886116467109405077541002256983155200055935729725\
71636269561882670428252483600823257530420752963450";
let mut digitVec = Vec::with_capacity(theNumber.bytes().len());
    for c in theNumber.bytes() {
        let digit = c - 0x30;
        digitVec.push(digit);
    }
    let mut maxval: i64 = 0;
    for i in 12..digitVec.len() {
        let mut accum: i64 = 1;
        for j in 0..13 {
            accum *= digitVec[i-j] as i64;
        }
        maxval = max(maxval,accum);
    }
    println!("The largest product of 13 adjacent digits is {}", maxval);
    //println!("The number:\n{}",theNumber);
}

pub fn problem9() {
    println!("\nProblem 9");
    for a in 1..1000 {
        for b in 1..1000 {
            if a + b > 999 {
                continue;
            }
            let c = 1000 - (a + b);
            let a2 = a*a;
            let b2 = b*b;
            let c2 = c*c;
            if a2 + b2 == c2 {
                println!("{}^2 + {}^2 = {}^2",a,b,c);
                println!("answer = {}", a*b*c);
                return;
            }
        }
    }
    unreachable!();
}

fn is_prime(x: i64) -> bool {
    let mut is_prime = x == 2 || x % 2 == 1;
    //a prime factor is bounded by sqrt(x)
    let max = (f32::sqrt(x as f32)+1.0f32) as i64;
    let mut i = 3;
    while is_prime && i < max {
        if (x % i) == 0 {
            is_prime = false;
        }
        i += 2;
    }
    is_prime
}

pub fn problem10() {
    println!("\nProblem 10");
    let limit = 2000000;
    let mut accum: i64 = 0;
    for i in 2..limit {
        if is_prime(i) {
            accum += i;
        }
    }
    println!("sum: {}",accum);
}

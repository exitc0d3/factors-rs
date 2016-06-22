/* Finn Fallowfield - 2016 */

fn factors_of(i:i64) -> Vec<i64> {
    let maxPrime: i64 = i/2;
    let mut factors: Vec<i64> = vec![1];

    for iter in 2..maxPrime {
        if (i%iter == 0) {
            factors.push(iter);
        }

        else { continue; }
    }

    factors.push(i);

    if (factors.len() == 2) {
        println!("{} is a prime number", i);
    }

    return factors;
}

fn main() {
    let mut v: Vec<i64> = factors_of(7);
    for i in &mut v {
        println!("{}", i);
    } 
}
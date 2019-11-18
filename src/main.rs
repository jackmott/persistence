use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub fn per_print(mut num: u128, steps: u8) -> u8 {
    println!("{}", num);
    let mut result = num % 10;
    num /= 10;
    while num != 0 {
        result = result * (num % 10);
        num /= 10;
    }

    if result < 10 {
        println!("{}", result);
        return steps;
    } else {
        return per_print(result, steps + 1);
    }
}

pub fn per(mut num: u128, steps: u8) -> u8 {
    let mut result = num % 10;
    num /= 10;
    while num != 0 {
        result = result * (num % 10);
        num /= 10;
    }

    if result < 10 {
        return steps;
    } else {
        return per(result, steps + 1);
    }
}

#[inline(always)]
fn get_num(digits: &Vec<u8>) -> u128 {
    let mut result: u128 = 0;
    for digit in digits {
        //no need to check for zero because we never pass any in
        result = result * 10 + *digit as u128;
    }
    return result;
}

#[inline(always)]
fn increment_digit(mut d: u8) -> u8 {
    d += 1;
    if d == 5 {
        6
    } else {
        d
    }
}

/*
//3 == max
[1,1]
[1,2]
[1,3]
[2,2]
[2,3]
[3,3]
->
[1,1,1]
[1,1,2]
[1,1,3]
[1,2,2]
[1,2,3]
[1,3,3]
[2,2,2]
[2,2,3]
[2,3,3]
[3,3,3]
->
*/

fn increment_digits(digits: &mut Vec<u8>) -> bool {
    unsafe {
        let mut i = digits.len() - 1;
        loop {
            if *digits.get_unchecked(i) < 9 {
                digits[i] = increment_digit(*digits.get_unchecked(i));
                for j in i + 1..digits.len() {
                    *digits.get_unchecked_mut(j) = *digits.get_unchecked(i);
                }
                return true;
            } else {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }
        return false;
    }
}

fn main() {
    let mut digit_groups  = Vec::with_capacity(39);
    for i in 1..40 {
        let mut digits = Vec::with_capacity(i);
        for _ in 0 .. i {
            digits.push(1);
        }
        digit_groups.push(digits);
    }

    //let mut now = Instant::now();
    let global_max = Mutex::new(0);
    digit_groups.par_chunks_mut(1).for_each(|digit_chunk| {
        let now = Instant::now();
        let digits = &mut digit_chunk[0];
        let mut max = 0;
        loop {
           
            let num = get_num(digits);
            let steps = per(num, 1);
            if steps > max {
                max = steps;
                let mut gmax = global_max.lock().unwrap();
                if max > *gmax {
                    *gmax = max;
                    per_print(num, 1);
                    println!("Digits{}, Steps:{}",digits.len(), steps);
                }                
                
            }
            if !increment_digits(digits) {
               println!("Digits {} complete in {} s",digits.len(),now.elapsed().as_secs_f32());
               break;
            }
        }
    });
}

/*
Estimate pi to 3dp using a Monte-Carlo method.
*/

extern crate rand;

use rand::Rng;
use std::f32;

fn main() {
    let mut rng = rand::thread_rng();
    let mut pi_estimate : f32 = 0.0;
    let tol : f32 = 0.001;  // Within 3dp
    let pi = f32::consts::PI;
    let mut number_in : i32 = 0;
    let mut total_number : i32 = 0;

    while  (pi_estimate - pi).abs() > tol {
        let x: f32 = rng.gen_range(0.0..1.0);
        let y: f32 = rng.gen_range(0.0..1.0);
        if x*x + y*y < 1.0 {
            number_in += 1;
        }
        total_number += 1;

        pi_estimate = 4.0*(number_in as f32 /total_number as f32);
        println!("Estimate is: {}", pi_estimate);
    }

    println!("Estimate of pi to 3dp is: {:.3}", pi_estimate);
    println!("{} iterations required.", total_number);
}

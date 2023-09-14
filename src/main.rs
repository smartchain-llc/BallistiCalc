#![allow(warnings)]
use ballistics::units::{Inch,Yard, LinearDistance, moa_adjustments_to_zero};

fn main() {
    let mut inchesFromBullseye = Inch::new(1.1);
    let targetDistance = Yard::new(200.);
    let adjustment = moa_adjustments_to_zero(&inchesFromBullseye, &targetDistance);
    println!("moa @ 200y: {:?}, adjustment for 2.1 inch offset: {}", &targetDistance.moa(), &adjustment.good_enough());
}   

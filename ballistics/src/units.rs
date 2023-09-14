#![allow(warnings)]

use std::cmp;

const INCH_YARD_RATIO: f32 = 0.0277;
const INCH_MOA_YARD_PERCENTAGE: f32 = 1.047; // 0.029 yards

pub trait LinearDistance {
    fn moa(&self) -> Inch;
    fn distance(&self) -> Self;
}

pub struct Yard { units: f32 }
impl Yard{
    pub fn new(distance: f32) -> Self {
        Self{ units: distance }
    }
}
impl LinearDistance for Yard {
    fn distance(&self) -> Self { Self { units: self.units } }
    fn moa(&self) -> Inch {
        Inch{ units: self.units / 100. * INCH_MOA_YARD_PERCENTAGE }
    }
}


#[derive(Debug)]
pub struct Inch { units: f32 }
impl Inch{
    pub fn new(distance: f32) -> Self {
        Self{ units: distance }
    }
}
impl LinearDistance for Inch {
    fn distance(&self) -> Self {
        Self{ units: self.units }
    }
    fn moa(&self) -> Inch {
        Inch { units: self.units * INCH_YARD_RATIO / 100. * INCH_MOA_YARD_PERCENTAGE }
    }
}

impl From<Inch> for Yard {
    fn from(value: Inch) -> Self {
        Self { units: value.units * INCH_YARD_RATIO }
    }
}

impl From<Yard> for Inch {
    fn from(value: Yard) -> Self {
        Self { units: value.units / INCH_YARD_RATIO}
    }
}


const ADJUSTMENT_ROUND: f32 = 0.5;

pub struct MOAAdjustment {
    amount: f32
}
impl MOAAdjustment {
    pub fn good_enough(&self) -> f32 {
        let frac = self.amount.fract();
        if frac > 0.5 {
            let top = self.amount.round();
            let bottom = self.amount.trunc() + ADJUSTMENT_ROUND;
            let deltaTop = (self.amount - top).abs();
            let deltaBottom = (self.amount - bottom).abs();
            let min = &deltaTop.min(deltaBottom);
            if *min == deltaBottom {
                return bottom;
            }
            return top;
        }
        else {
            let top = self.amount.trunc() + ADJUSTMENT_ROUND;
            let bottom = self.amount.round();
            let deltaTop = (self.amount - top).abs();
            let deltaBottom = (self.amount - bottom).abs();
            let min = &deltaTop.min(deltaBottom);
            if *min == deltaBottom {
                return bottom;
            }
            return top;
        }
    }
}
fn moa_at(distance: &Yard) -> Inch { Inch{ units: 0. } }
pub fn moa_adjustments_to_zero(inchOffset: &Inch, distance: &Yard) -> MOAAdjustment {
    MOAAdjustment { amount: inchOffset.units / distance.moa().units as f32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion_of_inch_to_yard() {
        let dInch: Inch = Inch{ units: 4_000. };
        let dYard: Yard = dInch.into();

        assert!((dYard.units - 111.111) <= 0.001);
    }
    #[test]
    fn conversion_of_yard_to_inch() {
        let dYard: Yard = Yard{ units: 400. };
        let dInch: Inch = dYard.into();

        assert!((dInch.units - 14440.433) <= 0.001)
    }
    #[test]
    fn yard_moa_calculations_are_accurate() {
        let mut distance: Yard = Yard{ units: 100. };
        assert_eq!(distance.moa().units, 1.047);

        distance.units = 325.;
        assert_eq!(distance.moa().units, (3.25 * 1.047));
    }
    #[test]
    fn inches_moa_calculations_are_accurate() {
        let mut distance: Inch = Yard{ units: 100. }.into();
        assert_eq!(distance.moa().units, 1.047);

        distance = Yard{ units: 375. }.into();
        assert_eq!(distance.moa().units, (3.75 * 1.047));
    }
    #[test]
    fn calculate_moa_adjustment_by_shot_offset() {
        // (4 * 1.047) 4.188 inches below @ 400y = 1MOA
        let mut offset: Inch = Inch { units: 4.188 };
        assert_eq!(moa_adjustments_to_zero(&offset, &Yard{ units: 400. }).amount, 1.);

        offset = Inch{ units: 2.1 };
        let adjustments = moa_adjustments_to_zero(&offset, &Yard{ units: 200. });
        assert_eq!(adjustments.good_enough(), 1.);

        offset = Inch{ units: 6. };
        let adjustments = moa_adjustments_to_zero(&offset, &Yard{ units: 400. });
        assert_eq!(adjustments.good_enough(), 1.5);

        offset = Inch { units: 1. };
        let adjustments = moa_adjustments_to_zero(&offset, &Yard{ units: 400. });
        assert_eq!(adjustments.good_enough(), 0.);
    }
}
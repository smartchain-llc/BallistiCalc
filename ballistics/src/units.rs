#![allow(warnings)]

struct Yard { units: f32 }
struct Inch { units: f32 }

impl From<Inch> for Yard {
    fn from(value: Inch) -> Self {
        Self { units: value.units * 0.02777778 }
    }
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
}
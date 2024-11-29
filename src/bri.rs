use uom::si::f64::*;

#[derive(Debug)]
struct Bri {
    value: f64,
}

impl Bri {
    pub fn new(height: Length, waist: Length) -> Self {
        // Waist measurement is a circumference, divide by pi to get diameter
        let width = waist.get::<uom::si::length::centimeter>() / std::f64::consts::PI;
        let height = height.get::<uom::si::length::centimeter>();
        Self {
            value: 364.2-(365.5*Self::eccentricity(height, width)),
        }
    }

    fn eccentricity(height: f64, width: f64) -> f64 {
        let a = height / 2.0;
        let b = width / 2.0;
        (a.powi(2) - b.powi(2)).abs().sqrt() / a
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;
    use uom::si::length::foot;
    use uom::si::length::inch;
    use uom::si::length::meter;
    use uom::si::mass::kilogram;
    use uom::si::mass::pound;
    use crate::bri::Bri;

    #[test]
    fn math() {
        let height = Length::new::<inch>(70.0);
        let waist = Length::new::<inch>(33.0);

        let height_val = height.get::<uom::si::length::centimeter>();
        let width_val = waist.get::<uom::si::length::centimeter>() / (2.0*std::f64::consts::PI);
        let bri = Bri::new(height, waist);
        assert_relative_eq!(bri.value, 2.8, epsilon = 0.1);

        let height = Length::new::<foot>(5.0) + Length::new::<inch>(10.0);
        let height_val = height.get::<uom::si::length::centimeter>();
        let bri2 = Bri::new(height, waist);
        assert_eq!(bri.value, bri2.value);
    }
}

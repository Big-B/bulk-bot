use uom::si::f64::*;

#[derive(Debug)]
struct Bri {
    value: f64,
}

impl Bri {
    fn new(height: Length, waist: Length) -> Self {
        let height = height.get::<uom::si::length::centimeter>();
        let waist = waist.get::<uom::si::length::centimeter>();
        let numerator = (waist / (2.0*std::f64::consts::PI)).powi(2);
        let denomenator = (height / 2.0).powi(2);
        Self {
            value: 364.2-(365.5*(1.0-(numerator/denomenator)).sqrt()),
        }
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

    #[test]
    fn math() {
        let height = Length::new::<inch>(70.0);
        let waist = Length::new::<inch>(33.0);
        let bri = Bri::new(height, waist);
        assert_relative_eq!(bri.value, 2.8, epsilon = 0.1);

        let height = Length::new::<foot>(5.0) + Length::new::<inch>(10.0);
        let bri2 = Bri::new(height, waist);
        assert_eq!(bri.value, bri2.value);
    }
}

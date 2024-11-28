use uom::si::f64::*;

#[derive(Debug, PartialEq)]
enum BmiClassification {
    SevereThinness,
    ModerateThinness,
    MildThinness,
    Normal,
    Overweight,
    ObeseClassI,
    ObeseClassII,
    ObeseClassIII,
}

#[derive(Debug)]
struct Bmi {
    value: f64,
}

impl Bmi {
    fn new(height: Length, weight: Mass) -> Self {
        let weight = weight.get::<uom::si::mass::kilogram>();
        let height = height.get::<uom::si::length::meter>();
        Self {
            value: weight / (height * height),
        }
    }

    fn get_classification(&self) -> BmiClassification {
        match self.value {
            ..16.0 => BmiClassification::SevereThinness,
            16.0..17.0 => BmiClassification::ModerateThinness,
            17.0..18.5 => BmiClassification::MildThinness,
            18.5..25.0 => BmiClassification::Normal,
            25.0..30.0 => BmiClassification::Overweight,
            30.0..35.0 => BmiClassification::ObeseClassI,
            35.0..40.0 => BmiClassification::ObeseClassII,
            40.0.. => BmiClassification::ObeseClassIII,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::length::foot;
    use uom::si::length::inch;
    use uom::si::length::meter;
    use uom::si::mass::kilogram;
    use uom::si::mass::pound;

    #[test]
    fn math() {
        let height = Length::new::<inch>(70.0);
        let weight = Mass::new::<pound>(160.0);
        let bmi = Bmi::new(height, weight);
        assert_eq!(bmi.get_classification(), BmiClassification::Normal);

        let height = Length::new::<meter>(1.778);
        let weight = Mass::new::<kilogram>(72.57);
        let bmi = Bmi::new(height, weight);
        assert_eq!(bmi.get_classification(), BmiClassification::Normal);

        let height = Length::new::<foot>(5.0) + Length::new::<inch>(10.0);
        let weight = Mass::new::<pound>(175.0);
        let bmi = Bmi::new(height, weight);
        assert_eq!(bmi.get_classification(), BmiClassification::Overweight);

        let height = Length::new::<foot>(6.0) + Length::new::<inch>(2.0);
        let weight = Mass::new::<pound>(175.0);
        let bmi = Bmi::new(height, weight);
        assert_eq!(bmi.get_classification(), BmiClassification::Normal);
    }

    #[test]
    fn classification_coverage() {
        let bmis = [
            Bmi { value: 0.0 },
            Bmi { value: 15.9999 },
            Bmi { value: 16.0 },
            Bmi { value: 17.0 },
            Bmi { value: 18.5 },
            Bmi { value: 25.0 },
            Bmi { value: 30.0 },
            Bmi { value: 35.0 },
            Bmi { value: 40.0 },
            Bmi { value: 40.0001 },
        ];
        assert_eq!(
            bmis[0].get_classification(),
            BmiClassification::SevereThinness
        );
        assert_eq!(
            bmis[1].get_classification(),
            BmiClassification::SevereThinness
        );
        assert_eq!(
            bmis[2].get_classification(),
            BmiClassification::ModerateThinness
        );
        assert_eq!(
            bmis[3].get_classification(),
            BmiClassification::MildThinness
        );
        assert_eq!(bmis[4].get_classification(), BmiClassification::Normal);
        assert_eq!(bmis[5].get_classification(), BmiClassification::Overweight);
        assert_eq!(bmis[6].get_classification(), BmiClassification::ObeseClassI);
        assert_eq!(
            bmis[7].get_classification(),
            BmiClassification::ObeseClassII
        );
        assert_eq!(
            bmis[8].get_classification(),
            BmiClassification::ObeseClassIII
        );
        assert_eq!(
            bmis[9].get_classification(),
            BmiClassification::ObeseClassIII
        );
    }
}

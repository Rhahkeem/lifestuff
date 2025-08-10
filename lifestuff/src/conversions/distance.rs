use lifestuff_types::conversions::distance::DistanceUnits;

#[cfg(test)]
mod tests {
    use super::*;
    use lifestuff_types::conversions::distance::DistanceUnits;

    #[test]
    fn test_distance_conversion_yards_to_inches() {
        let result = distance_conversions(&DistanceUnits::Yards, &DistanceUnits::Inches, &1.0);
        assert_eq!(result, 36.0);
    }

    #[test]
    fn test_distance_conversion_yards_to_kilometres() {
        let result = distance_conversions(&DistanceUnits::Yards, &DistanceUnits::Kilometres, &1000.0);
        assert!((result - 0.9144).abs() < 0.0001);
    }

    #[test]
    fn test_distance_conversion_yards_to_metres() {
        let result = distance_conversions(&DistanceUnits::Yards, &DistanceUnits::Metres, &1.0);
        assert!((result - 0.9144).abs() < 0.0001);
    }

    #[test]
    fn test_distance_conversion_yards_to_miles() {
        let result = distance_conversions(&DistanceUnits::Yards, &DistanceUnits::Miles, &1760.0);
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_distance_conversion_yards_to_feet() {
        let result = distance_conversions(&DistanceUnits::Yards, &DistanceUnits::Feet, &1.0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_distance_conversion_inches_to_yards() {
        let result = distance_conversions(&DistanceUnits::Inches, &DistanceUnits::Yards, &36.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_distance_conversion_inches_to_feet() {
        let result = distance_conversions(&DistanceUnits::Inches, &DistanceUnits::Feet, &12.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_distance_conversion_kilometres_to_metres() {
        let result = distance_conversions(&DistanceUnits::Kilometres, &DistanceUnits::Metres, &1.0);
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_distance_conversion_kilometres_to_miles() {
        let result = distance_conversions(&DistanceUnits::Kilometres, &DistanceUnits::Miles, &1.609344);
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_distance_conversion_metres_to_feet() {
        let result = distance_conversions(&DistanceUnits::Metres, &DistanceUnits::Feet, &1.0);
        assert!((result - 3.280839895).abs() < 0.0001);
    }

    #[test]
    fn test_distance_conversion_miles_to_yards() {
        let result = distance_conversions(&DistanceUnits::Miles, &DistanceUnits::Yards, &1.0);
        assert_eq!(result, 1760.0);
    }

    #[test]
    fn test_distance_conversion_miles_to_feet() {
        let result = distance_conversions(&DistanceUnits::Miles, &DistanceUnits::Feet, &1.0);
        assert_eq!(result, 5280.0);
    }

    #[test]
    fn test_distance_conversion_feet_to_inches() {
        let result = distance_conversions(&DistanceUnits::Feet, &DistanceUnits::Inches, &1.0);
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_distance_conversion_same_unit() {
        let result = distance_conversions(&DistanceUnits::Metres, &DistanceUnits::Metres, &5.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_distance_conversion_zero_value() {
        let result = distance_conversions(&DistanceUnits::Miles, &DistanceUnits::Kilometres, &0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_distance_conversion_negative_value() {
        let result = distance_conversions(&DistanceUnits::Metres, &DistanceUnits::Feet, &-10.0);
        assert!((result + 32.80839895).abs() < 0.0001);
    }
}

pub fn distance_conversions(from: &DistanceUnits, to: &DistanceUnits, val: &f64) -> f64 {
    match &from {
        DistanceUnits::Yards => match &to {
            DistanceUnits::Yards => val.to_owned(),
            DistanceUnits::Inches => val * 36.0,
            DistanceUnits::Kilometres => val * 0.0009144,
            DistanceUnits::Metres => val * 0.9144,
            DistanceUnits::Miles => val * 0.0005681818,
            DistanceUnits::Feet => val * 3.0,
        },
        DistanceUnits::Inches => match &to {
            DistanceUnits::Yards => val / 36.0,
            DistanceUnits::Inches => val.to_owned(),
            DistanceUnits::Kilometres => val * 0.0000254,
            DistanceUnits::Metres => val * 0.0254,
            DistanceUnits::Miles => val * 0.00001578283,
            DistanceUnits::Feet => val / 12.0,
        },
        DistanceUnits::Kilometres => match &to {
            DistanceUnits::Yards => val / 0.0009144,
            DistanceUnits::Inches => val * 39370.1,
            DistanceUnits::Kilometres => val.to_owned(),
            DistanceUnits::Metres => val * 1000.0,
            DistanceUnits::Miles => val / 1.609344,
            DistanceUnits::Feet => val * 3280.84,
        },
        DistanceUnits::Metres => match &to {
            DistanceUnits::Yards => val / 0.9144,
            DistanceUnits::Inches => val * 39.37007874,
            DistanceUnits::Kilometres => val / 1000.0,
            DistanceUnits::Metres => val.to_owned(),
            DistanceUnits::Miles => val / 1609.344,
            DistanceUnits::Feet => val * 3.280839895,
        },
        DistanceUnits::Miles => match &to {
            DistanceUnits::Yards => val * 1760.0,
            DistanceUnits::Inches => val * 63360.0,
            DistanceUnits::Kilometres => val * 1.609344,
            DistanceUnits::Metres => val * 1609.344,
            DistanceUnits::Miles => val.to_owned(),
            DistanceUnits::Feet => val * 5280.0,
        },
        DistanceUnits::Feet => match &to {
            DistanceUnits::Yards => val / 3.0,
            DistanceUnits::Inches => val * 12.0,
            DistanceUnits::Kilometres => val / 3280.84,
            DistanceUnits::Metres => val * 0.3048,
            DistanceUnits::Miles => val / 5280.0,
            DistanceUnits::Feet => val.to_owned(),
        },
    }
}

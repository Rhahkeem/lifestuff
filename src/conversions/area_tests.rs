use crate::conversions::area::{area_conversions, AreaUnits};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_conversion_acres_to_square_meters() {
        let from = AreaUnits::Acres;
        let to = AreaUnits::SquareMetres;
        let value = 1.0; // 1 acre
        let expected = 4046.856422; // Expected value in square meters
        let result = area_conversions(&from, &to, &value);
        assert!((result - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_conversion_square_meters_to_acres() {
        let from = AreaUnits::SquareMetres;
        let to = AreaUnits::Acres;
        let value = 4046.856422; // 1 acre in square meters
        let expected = 1.0; // Expected value in acres
        let result = area_conversions(&from, &to, &value);
        assert!((result - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_conversion_square_kilometers_to_square_meters() {
        let from = AreaUnits::SqKilometres;
        let to = AreaUnits::SquareMetres;
        let value = 1.0; // 1 square kilometer
        let expected = 1000000.0; // Expected value in square meters
        let result = area_conversions(&from, &to, &value);
        assert!((result - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_conversion_square_meters_to_square_kilometers() {
        let from = AreaUnits::SquareMetres;
        let to = AreaUnits::SqKilometres;
        let value = 1000000.0; // 1 square kilometer in square meters
        let expected = 1.0; // Expected value in square kilometers
        let result = area_conversions(&from, &to, &value);
        assert!((result - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_conversion_square_feet_to_square_inches() {
        let from = AreaUnits::SquareFeet;
        let to = AreaUnits::SqInches;
        let value = 1.0; // 1 square foot
        let expected = 144.0; // Expected value in square inches
        let result = area_conversions(&from, &to, &value);
        assert!((result - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_area_conversion_square_inches_to_square_feet() {
        let from = AreaUnits::SqInches;
        let to = AreaUnits::SquareFeet;
        let value = 144.0; // 1 square foot in square inches
        let expected = 1.0; // Expected value in square feet
        let result = area_conversions(&from, &to, &value);
        assert!((result - expected).abs() < f64::EPSILON);
    }
}

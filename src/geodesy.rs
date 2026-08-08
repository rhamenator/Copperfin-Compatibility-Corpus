//! Spherical geodesy used to replace the degree/radian and longitude-scaling
//! defects in the 1990s `NEXTLAT`, `NEXTLONG`, and related helpers.

use std::f64::consts::PI;

/// IUGG mean Earth radius in metres.
pub const MEAN_EARTH_RADIUS_METERS: f64 = 6_371_008.8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint {
    pub latitude_degrees: f64,
    pub longitude_degrees: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeodesyError {
    NonFiniteInput,
    LatitudeOutOfRange,
    NonPositiveRadius,
    NegativeDistance,
    CoincidentPoints,
}

impl GeoPoint {
    pub fn try_new(latitude_degrees: f64, longitude_degrees: f64) -> Result<Self, GeodesyError> {
        if !latitude_degrees.is_finite() || !longitude_degrees.is_finite() {
            return Err(GeodesyError::NonFiniteInput);
        }
        if !(-90.0..=90.0).contains(&latitude_degrees) {
            return Err(GeodesyError::LatitudeOutOfRange);
        }
        Ok(Self {
            latitude_degrees,
            longitude_degrees: normalize_longitude(longitude_degrees),
        })
    }
}

pub fn normalize_bearing(bearing_degrees: f64) -> f64 {
    bearing_degrees.rem_euclid(360.0)
}

pub fn normalize_longitude(longitude_degrees: f64) -> f64 {
    (longitude_degrees + 180.0).rem_euclid(360.0) - 180.0
}

fn validate_radius(radius: f64) -> Result<(), GeodesyError> {
    if !radius.is_finite() {
        return Err(GeodesyError::NonFiniteInput);
    }
    if radius <= 0.0 {
        return Err(GeodesyError::NonPositiveRadius);
    }
    Ok(())
}

/// Returns the point reached after following an initial compass bearing over a
/// spherical Earth. Bearings are degrees clockwise from true north.
pub fn destination_point(
    start: GeoPoint,
    distance: f64,
    bearing_degrees: f64,
    radius: f64,
) -> Result<GeoPoint, GeodesyError> {
    validate_radius(radius)?;
    if !distance.is_finite() || !bearing_degrees.is_finite() {
        return Err(GeodesyError::NonFiniteInput);
    }
    if distance < 0.0 {
        return Err(GeodesyError::NegativeDistance);
    }

    let latitude_1 = start.latitude_degrees.to_radians();
    let longitude_1 = start.longitude_degrees.to_radians();
    let bearing = normalize_bearing(bearing_degrees).to_radians();
    let angular_distance = distance / radius;

    let latitude_2 = (latitude_1.sin() * angular_distance.cos()
        + latitude_1.cos() * angular_distance.sin() * bearing.cos())
    .clamp(-1.0, 1.0)
    .asin();

    let longitude_2 = longitude_1
        + (bearing.sin() * angular_distance.sin() * latitude_1.cos())
            .atan2(angular_distance.cos() - latitude_1.sin() * latitude_2.sin());

    GeoPoint::try_new(latitude_2.to_degrees(), longitude_2.to_degrees())
}

/// Numerically stable great-circle distance using the haversine formula.
pub fn great_circle_distance(
    first: GeoPoint,
    second: GeoPoint,
    radius: f64,
) -> Result<f64, GeodesyError> {
    validate_radius(radius)?;
    let latitude_1 = first.latitude_degrees.to_radians();
    let latitude_2 = second.latitude_degrees.to_radians();
    let delta_latitude = latitude_2 - latitude_1;
    let delta_longitude = (second.longitude_degrees - first.longitude_degrees).to_radians();

    let haversine = (delta_latitude / 2.0).sin().powi(2)
        + latitude_1.cos() * latitude_2.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * haversine.sqrt().atan2((1.0 - haversine).max(0.0).sqrt());
    Ok(radius * central_angle)
}

/// Initial great-circle bearing in compass degrees.
pub fn initial_bearing(first: GeoPoint, second: GeoPoint) -> Result<f64, GeodesyError> {
    if first == second {
        return Err(GeodesyError::CoincidentPoints);
    }
    let latitude_1 = first.latitude_degrees.to_radians();
    let latitude_2 = second.latitude_degrees.to_radians();
    let delta_longitude = (second.longitude_degrees - first.longitude_degrees).to_radians();
    let y = delta_longitude.sin() * latitude_2.cos();
    let x = latitude_1.cos() * latitude_2.sin()
        - latitude_1.sin() * latitude_2.cos() * delta_longitude.cos();
    Ok(normalize_bearing(y.atan2(x) * 180.0 / PI))
}

pub fn cardinal_to_bearing(code: &str) -> Option<f64> {
    match code.trim().to_ascii_uppercase().as_str() {
        "N" => Some(0.0),
        "NE" => Some(45.0),
        "E" => Some(90.0),
        "SE" => Some(135.0),
        "S" => Some(180.0),
        "SW" => Some(225.0),
        "W" => Some(270.0),
        "NW" => Some(315.0),
        _ => None,
    }
}

pub fn bearing_to_cardinal(bearing_degrees: f64) -> &'static str {
    const CODES: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = ((normalize_bearing(bearing_degrees) + 22.5) / 45.0).floor() as usize % 8;
    CODES[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(lat: f64, lon: f64) -> GeoPoint {
        GeoPoint::try_new(lat, lon).unwrap()
    }

    #[test]
    fn one_degree_at_equator_has_expected_spherical_distance() {
        let distance =
            great_circle_distance(point(0.0, 0.0), point(0.0, 1.0), MEAN_EARTH_RADIUS_METERS)
                .unwrap();
        assert!((distance - 111_195.080_233_5).abs() < 0.001);
    }

    #[test]
    fn destination_and_inverse_round_trip() {
        let start = point(42.7325, -84.5555);
        let destination =
            destination_point(start, 25_000.0, 73.0, MEAN_EARTH_RADIUS_METERS).unwrap();
        let distance = great_circle_distance(start, destination, MEAN_EARTH_RADIUS_METERS).unwrap();
        let bearing = initial_bearing(start, destination).unwrap();
        assert!((distance - 25_000.0).abs() < 1e-6);
        assert!((bearing - 73.0).abs() < 1e-9);
    }

    #[test]
    fn compass_codes_are_normalized_without_silent_invalid_default() {
        assert_eq!(cardinal_to_bearing(" sw "), Some(225.0));
        assert_eq!(cardinal_to_bearing("unknown"), None);
        assert_eq!(bearing_to_cardinal(-1.0), "N");
        assert_eq!(bearing_to_cardinal(225.0), "SW");
    }
}

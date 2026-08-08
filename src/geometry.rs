//! Planar geometry helpers for polyline and local road-shape analysis.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryError {
    NonFiniteInput,
    CoincidentPoints,
    CollinearPoints,
    TooFewPoints,
}

pub fn distance(first: Point2, second: Point2) -> Result<f64, GeometryError> {
    if !first.x.is_finite()
        || !first.y.is_finite()
        || !second.x.is_finite()
        || !second.y.is_finite()
    {
        return Err(GeometryError::NonFiniteInput);
    }
    Ok((second.x - first.x).hypot(second.y - first.y))
}

pub fn polyline_length(points: &[Point2]) -> Result<f64, GeometryError> {
    if points.len() < 2 {
        return Err(GeometryError::TooFewPoints);
    }
    points
        .windows(2)
        .try_fold(0.0, |total, pair| Ok(total + distance(pair[0], pair[1])?))
}

/// Smallest change in heading at `middle`, from 0 to 180 degrees.
pub fn turn_angle_degrees(
    first: Point2,
    middle: Point2,
    last: Point2,
) -> Result<f64, GeometryError> {
    let incoming = (middle.x - first.x, middle.y - first.y);
    let outgoing = (last.x - middle.x, last.y - middle.y);
    let incoming_length = incoming.0.hypot(incoming.1);
    let outgoing_length = outgoing.0.hypot(outgoing.1);
    if incoming_length == 0.0 || outgoing_length == 0.0 {
        return Err(GeometryError::CoincidentPoints);
    }
    let cosine = ((incoming.0 * outgoing.0 + incoming.1 * outgoing.1)
        / (incoming_length * outgoing_length))
        .clamp(-1.0, 1.0);
    Ok(cosine.acos().to_degrees())
}

/// Radius of the unique circle through three non-collinear planar points.
pub fn circumradius(first: Point2, middle: Point2, last: Point2) -> Result<f64, GeometryError> {
    let a = distance(middle, last)?;
    let b = distance(first, last)?;
    let c = distance(first, middle)?;
    if a == 0.0 || b == 0.0 || c == 0.0 {
        return Err(GeometryError::CoincidentPoints);
    }
    let twice_area = ((middle.x - first.x) * (last.y - first.y)
        - (middle.y - first.y) * (last.x - first.x))
        .abs();
    if twice_area <= f64::EPSILON * a.max(b).max(c).powi(2) {
        return Err(GeometryError::CollinearPoints);
    }
    Ok((a * b * c) / (2.0 * twice_area))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_angle_geometry_is_stable() {
        let a = Point2 { x: 0.0, y: 0.0 };
        let b = Point2 { x: 1.0, y: 0.0 };
        let c = Point2 { x: 1.0, y: 1.0 };
        assert!((turn_angle_degrees(a, b, c).unwrap() - 90.0).abs() < 1e-12);
        assert!((circumradius(a, b, c).unwrap() - 2.0_f64.sqrt() / 2.0).abs() < 1e-12);
        assert!((polyline_length(&[a, b, c]).unwrap() - 2.0).abs() < 1e-12);
    }
}

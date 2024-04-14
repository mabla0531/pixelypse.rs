pub struct UtilFunctions {}

impl UtilFunctions {
    pub fn get_angle(point1: (f64, f64), point2: (f64, f64)) -> f64 {
        let delta = (point1.0 - point2.0, point1.1 - point2.1);
        let angle = libm::atan2(delta.1, delta.0);

        angle
    }
}

use sfml::system::Vector2f;

pub struct UtilFunctions {}

impl UtilFunctions {
    pub fn get_angle(point1: Vector2f, point2: Vector2f) -> f64 {
        let delta = Vector2f::new(point1.x - point2.x, point1.y - point2.y);

        libm::atan2(delta.y as f64, delta.x as f64)
    }
}

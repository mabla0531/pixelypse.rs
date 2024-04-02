use std::ops;

pub struct UtilFunctions {}

impl UtilFunctions {
    pub fn get_angle(point1: Point<f64>, point2: Point<f64>) -> f64 {
        let delta = (point1.x - point2.x, point1.y - point2.y);
        let angle = libm::atan2(delta.1, delta.0);

        angle
    }
}

#[derive(Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x,
            y,
        }
    }
}

impl<T> ops::Add<Point<T>> for Point<T> 
    where T: 
        std::ops::Add<Output = T> {
            
    type Output = Point<T>;

    fn add(self, _rhs: Point<T>) -> Point<T::Output> {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl<T> ops::Add<T> for Point<T>
    where 
        T: std::ops::Add<Output = T>,
        T: Copy {

    type Output = Point<T>;

    fn add(self, _rhs: T) -> Point<T::Output> {
        Point {
            x: self.x + _rhs,
            y: self.y + _rhs,
        }
    }
}

impl<T> ops::Sub<Point<T>> for Point<T> 
    where T: 
        std::ops::Sub<Output = T> {
            
    type Output = Point<T>;

    fn sub(self, _rhs: Point<T>) -> Point<T::Output> {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl<T> ops::Sub<T> for Point<T>
    where 
        T: std::ops::Sub<Output = T>,
        T: Copy {

    type Output = Point<T>;

    fn sub(self, _rhs: T) -> Point<T::Output> {
        Point {
            x: self.x - _rhs,
            y: self.y - _rhs,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Rect<T> {
    pub left: T,
    pub top: T, 
    pub right: T,
    pub bottom: T,
}

impl<T> Rect<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Rect {
            left, 
            top, 
            right, 
            bottom
        }
    }

    pub fn width(self) -> T 
        where 
            T: std::ops::Sub<Output = T> {
        self.right - self.left
    }

    pub fn height(self) -> T 
        where 
            T: std::ops::Sub<Output = T> {
        self.bottom - self.top
    }
}

impl<T> Rect<T>
    where 
        T: std::ops::Add, 
        T: std::cmp::PartialOrd<<T as std::ops::Add>::Output> {

    /// Checks whether a rectangle intersects another at any point
    /// # Examples
    /// ```
    /// assert_eq!(Rect::new(0, 0, 2, 2).contains(Rect::new(0, 0, 2, 2)), true);
    /// assert_eq!(Rect::new(0, 0, 1, 2).contains(Rect::new(0, 0, 2, 2)), false);
    /// ```
    pub fn intersects(self, rect2: Rect<T>) -> bool
        where 
            T: PartialOrd<T> {
        ( rect2.left > self.left &&  rect2.left < self.right &&    rect2.top > self.top &&    rect2.top < self.bottom) ||
        (rect2.right > self.left && rect2.right < self.right &&    rect2.top > self.top &&    rect2.top < self.bottom) ||
        ( rect2.left > self.left &&  rect2.left < self.right && rect2.bottom > self.top && rect2.bottom < self.bottom) ||
        (rect2.right > self.left && rect2.right < self.right && rect2.bottom > self.top && rect2.bottom < self.bottom)
        
    }

    /// Checks whether the bounds of the rectangle are fully surrounding 
    /// the point (left and top < point x, right and bottom > point y)
    /// # Examples
    /// ```
    /// assert_eq!(Rect::new(0, 0, 2, 2).contains(Point::new(1, 1)), true);
    /// assert_eq!(Rect::new(0, 0, 1, 2).contains(Point::new(1, 1)), false);
    /// ```
    pub fn contains(self, point: Point<T>) -> bool
        where 
            T: PartialOrd<T> {
        self.left < point.x && self.top < point.y && self.right > point.x && self.bottom > point.y
    }

    pub fn translate(&mut self, offset: Point<T>) 
        where 
            T: std::ops::Add<Output = T>,
            T: Copy {
        *self = Self {
            left: self.left + offset.x,
            top: self.top + offset.y,
            right: self.right + offset.x,
            bottom: self.bottom + offset.y,
        };
    }
}

impl From<Rect<f64>> for [f64; 4] {
    fn from(value: Rect<f64>) -> Self {
        [value.left, value.top, value.right - value.left, value.bottom - value.top]
    }
}
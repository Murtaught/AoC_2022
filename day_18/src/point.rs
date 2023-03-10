use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point3 {
    pub fn parse(s: &str) -> Point3 {
        let mut it = s.split(',').map(|s| s.trim());
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        let z = it.next().unwrap().parse().unwrap();
        assert_eq!(it.next(), None);
        Self { x, y, z }
    }

    pub fn neighbors(self) -> impl Iterator<Item = Point3> {
        [
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ]
            .into_iter()
            .map(move |d| self + d)
    }
}

impl From<(i64, i64, i64)> for Point3 {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Point3 { x, y, z }
    }
}

impl<T: Into<Point3>> Add<T> for Point3 {
    type Output = Point3;

    fn add(mut self, rhs: T) -> Point3 {
        self += rhs;
        self
    }
}

impl<T: Into<Point3>> AddAssign<T> for Point3 {
    fn add_assign(&mut self, rhs: T) {
        let rhs: Point3 = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

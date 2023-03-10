use std::cmp::{max, min};

use crate::point::Point3;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct World(Point3, Point3);

impl World {
    pub fn new() -> World {
        Self::default()
    }

    pub fn encase<'a, C>(container: C) -> World
    where
        C: IntoIterator<Item = &'a Point3>,
    {
        let mut world = World::new();
        for p in container {
            world.expand(p);
        }
        world.0 += (-1, -1, -1);
        world.1 += (1, 1, 1);
        world
    }

    pub fn expand(&mut self, p: &Point3) {
        self.0.x = min(self.0.x, p.x);
        self.0.y = min(self.0.y, p.y);
        self.0.z = min(self.0.z, p.z);
        self.1.x = max(self.1.x, p.x);
        self.1.y = max(self.1.y, p.y);
        self.1.z = max(self.1.z, p.z);
    }

    pub fn top_left(&self) -> Point3 {
        self.0
    }

    #[allow(dead_code)]
    pub fn bottom_right(&self) -> Point3 {
        self.1
    }

    pub fn contains(&self, p: &Point3) -> bool {
        (self.0.x..=self.1.x).contains(&p.x)
            && (self.0.y..=self.1.y).contains(&p.y)
            && (self.0.z..=self.1.z).contains(&p.z)
    }
}

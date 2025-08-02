use crate::{Intersection, Scene};
use glam::DVec3;

mod point;

pub use point::Point;

pub trait Light: Sync {
    fn color_at(&self, intersection: &Intersection, v: DVec3, scene: &Scene) -> DVec3;
}
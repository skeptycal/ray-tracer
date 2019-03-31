use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;

pub mod sphere;

pub trait Hitable<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>>;
}

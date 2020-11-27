use crate::meta::Coord;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Vector<T>
where
    T: Coord,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector<T>
where
    T: Coord,
{
    pub fn with_coords(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn i_hat() -> Self {
        Self::with_coords(T::one(), T::zero())
    }

    pub fn j_hat() -> Self {
        Self::with_coords(T::zero(), T::one())
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Self) -> T {
        self.x * other.y - other.x * self.y
    }
}

impl<T> fmt::Display for Vector<T>
where
    T: Coord + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> Neg for Vector<T>
where
    T: Coord,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::with_coords(-self.x, -self.y)
    }
}

impl<T> Add for Vector<T>
where
    T: Coord,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::with_coords(self.x + other.x, self.y + other.y)
    }
}

impl<T> Sub for Vector<T>
where
    T: Coord,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::with_coords(self.x - other.x, self.y - other.y)
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Coord,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self::with_coords(self.x * scalar, self.y * scalar)
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Coord,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self::with_coords(self.x / scalar, self.y / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_default_vector() {
        let v: Vector<i32> = Vector::default();
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
    }

    #[test]
    fn creates_vector_with_specified_coordinates() {
        let v = Vector::with_coords(42, 42);
        assert_eq!(v.x, 42);
        assert_eq!(v.y, 42);
    }

    #[test]
    fn creates_unit_vector_alongside_horizontal_axis() {
        let x_unit: Vector<i32> = Vector::i_hat();
        assert_eq!(x_unit, Vector::with_coords(1, 0));
    }

    #[test]
    fn creates_unit_vector_alongside_vertical_axis() {
        let y_unit: Vector<i32> = Vector::j_hat();
        assert_eq!(y_unit, Vector::with_coords(0, 1));
    }

    #[test]
    fn creates_string_representation_of_vector() {
        let v = Vector::with_coords(-42, 0);
        assert_eq!(v.to_string(), "[-42, 0]");
    }

    #[test]
    fn recognizes_two_equal_vectors() {
        let v = Vector::with_coords(42, 42);
        let w = Vector::with_coords(42, 42);
        assert!(v == w);
    }

    #[test]
    fn distinguishes_two_different_vectors() {
        let v = Vector::with_coords(42, 0);
        let w = Vector::with_coords(-1, -1);
        assert!(!(v == w));
        assert!(v != w);
    }

    #[test]
    fn negates_vector() {
        let v = Vector::with_coords(42, 42);
        assert_eq!(-v, Vector::with_coords(-42, -42));
    }

    #[test]
    fn adds_two_vectors() {
        let horizontal = Vector::with_coords(42, 0);
        let vertical = Vector::with_coords(0, 42);
        assert_eq!(horizontal + vertical, Vector::with_coords(42, 42));
    }

    #[test]
    fn subtracts_two_vectors() {
        let horizontal = Vector::with_coords(42, 0);
        let vertical = Vector::with_coords(0, 42);
        assert_eq!(horizontal - vertical, Vector::with_coords(42, -42));
    }

    #[test]
    fn multiplies_vector_by_scalar() {
        let v = Vector::with_coords(21, 12);
        assert_eq!(v * 2, Vector::with_coords(42, 24));
    }

    #[test]
    fn divides_vector_by_scalar() {
        let v = Vector::with_coords(42, 24);
        assert_eq!(v / 2, Vector::with_coords(21, 12));
    }

    #[test]
    fn divides_vector_with_integer_coords_by_scalar() {
        let v = Vector::with_coords(41, 24);
        assert_eq!(v / 2, Vector::with_coords(20, 12));
    }

    #[test]
    fn computes_dot_product_of_two_vectors() {
        let v = Vector::with_coords(1, 0);
        let w = Vector::with_coords(42, 42);
        assert_eq!(v.dot(w), 42);
    }

    #[test]
    fn computes_cross_product_of_two_vectors() {
        let v = Vector::with_coords(42, 2);
        let w = Vector::with_coords(3, 1);
        assert_eq!(v.cross(w), 36);
    }
}

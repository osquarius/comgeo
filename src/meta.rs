use num_traits::sign::Signed;

pub trait Coord: Signed + Copy {}

impl<T> Coord for T where T: Signed + Copy {}

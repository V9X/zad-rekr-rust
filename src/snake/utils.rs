#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]


pub enum Direction {
    up,
    down,
    right,
    left,
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::down => Direction::up,
            Direction::up => Direction::down,
            Direction::left => Direction::right,
            Direction::right => Direction::left,
        }
    }
}
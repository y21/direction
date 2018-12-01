//! Representations of directions
extern crate coord_2d;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub use coord_2d::Coord;
use std::mem;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Range};
use std::slice;

pub const NUM_DIRECTIONS: usize = 8;
pub const NUM_CARDINAL_DIRECTIONS: usize = 4;
pub const NUM_ORDINAL_DIRECTIONS: usize = 4;
pub const ALL_DIRECTIONS_BITMAP_RAW: u8 = 0xff;
pub const NO_DIRECTIONS_BITMAP_RAW: u8 = 0;
pub const ALL_CARDINAL_DIRECTION_BITMAP_RAW: u8 =
    (1 << Direction::North as usize) | (1 << Direction::East as usize)
        | (1 << Direction::South as usize) | (1 << Direction::West as usize);

pub const ALL_ORDINAL_DIRECTION_BITMAP_RAW: u8 =
    (1 << Direction::NorthEast as usize) | (1 << Direction::SouthEast as usize)
        | (1 << Direction::SouthWest as usize) | (1 << Direction::NorthWest as usize);

pub const ALL_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: ALL_DIRECTIONS_BITMAP_RAW,
};
pub const NO_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: NO_DIRECTIONS_BITMAP_RAW,
};
pub const ALL_CARDINAL_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: ALL_CARDINAL_DIRECTION_BITMAP_RAW,
};
pub const ALL_ORDINAL_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: ALL_ORDINAL_DIRECTION_BITMAP_RAW,
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum OrdinalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DirectionType {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection),
}

impl Direction {
    pub fn from_unit_coord(coord: Coord) -> Self {
        match coord {
            Coord { x: 1, y: 0 } => Direction::East,
            Coord { x: -1, y: 0 } => Direction::West,
            Coord { x: 0, y: 1 } => Direction::South,
            Coord { x: 0, y: -1 } => Direction::North,
            Coord { x: 1, y: 1 } => Direction::SouthEast,
            Coord { x: 1, y: -1 } => Direction::NorthEast,
            Coord { x: -1, y: 1 } => Direction::SouthWest,
            Coord { x: -1, y: -1 } => Direction::NorthWest,
            _ => panic!("Unexpected coord: {:?}", coord),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    pub fn coord(self) -> Coord {
        match self {
            Direction::North => Coord::new(0, -1),
            Direction::NorthEast => Coord::new(1, -1),
            Direction::East => Coord::new(1, 0),
            Direction::SouthEast => Coord::new(1, 1),
            Direction::South => Coord::new(0, 1),
            Direction::SouthWest => Coord::new(-1, 1),
            Direction::West => Coord::new(-1, 0),
            Direction::NorthWest => Coord::new(-1, -1),
        }
    }

    pub fn left90(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::NorthEast => Direction::NorthWest,
            Direction::East => Direction::North,
            Direction::SouthEast => Direction::NorthEast,
            Direction::South => Direction::East,
            Direction::SouthWest => Direction::SouthEast,
            Direction::West => Direction::South,
            Direction::NorthWest => Direction::SouthWest,
        }
    }

    pub fn right90(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }

    pub fn left45(self) -> Direction {
        match self {
            Direction::North => Direction::NorthWest,
            Direction::NorthEast => Direction::North,
            Direction::East => Direction::NorthEast,
            Direction::SouthEast => Direction::East,
            Direction::South => Direction::SouthEast,
            Direction::SouthWest => Direction::South,
            Direction::West => Direction::SouthWest,
            Direction::NorthWest => Direction::West,
        }
    }

    pub fn right45(self) -> Direction {
        match self {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        }
    }

    pub fn left135(self) -> Direction {
        match self {
            Direction::North => Direction::SouthWest,
            Direction::NorthEast => Direction::West,
            Direction::East => Direction::NorthWest,
            Direction::SouthEast => Direction::North,
            Direction::South => Direction::NorthEast,
            Direction::SouthWest => Direction::East,
            Direction::West => Direction::SouthEast,
            Direction::NorthWest => Direction::South,
        }
    }

    pub fn right135(self) -> Direction {
        match self {
            Direction::North => Direction::SouthEast,
            Direction::NorthEast => Direction::South,
            Direction::East => Direction::SouthWest,
            Direction::SouthEast => Direction::West,
            Direction::South => Direction::NorthWest,
            Direction::SouthWest => Direction::North,
            Direction::West => Direction::NorthEast,
            Direction::NorthWest => Direction::East,
        }
    }

    pub fn bitmap_raw(self) -> u8 {
        1 << self as usize
    }

    pub fn bitmap(self) -> DirectionBitmap {
        DirectionBitmap::new(self.bitmap_raw())
    }

    pub fn is_cardinal(self) -> bool {
        match self {
            Direction::North => true,
            Direction::NorthEast => false,
            Direction::East => true,
            Direction::SouthEast => false,
            Direction::South => true,
            Direction::SouthWest => false,
            Direction::West => true,
            Direction::NorthWest => false,
        }
    }

    pub fn is_ordinal(self) -> bool {
        match self {
            Direction::North => false,
            Direction::NorthEast => true,
            Direction::East => false,
            Direction::SouthEast => true,
            Direction::South => false,
            Direction::SouthWest => true,
            Direction::West => false,
            Direction::NorthWest => true,
        }
    }

    pub fn typ(self) -> DirectionType {
        match self {
            Direction::North => DirectionType::Cardinal(CardinalDirection::North),
            Direction::NorthEast => DirectionType::Ordinal(OrdinalDirection::NorthEast),
            Direction::East => DirectionType::Cardinal(CardinalDirection::East),
            Direction::SouthEast => DirectionType::Ordinal(OrdinalDirection::SouthEast),
            Direction::South => DirectionType::Cardinal(CardinalDirection::South),
            Direction::SouthWest => DirectionType::Ordinal(OrdinalDirection::SouthWest),
            Direction::West => DirectionType::Cardinal(CardinalDirection::West),
            Direction::NorthWest => DirectionType::Ordinal(OrdinalDirection::NorthWest),
        }
    }

    pub fn cardinal(self) -> Option<CardinalDirection> {
        match self {
            Direction::North => Some(CardinalDirection::North),
            Direction::East => Some(CardinalDirection::East),
            Direction::South => Some(CardinalDirection::South),
            Direction::West => Some(CardinalDirection::West),
            _ => None,
        }
    }

    pub fn ordinal(self) -> Option<OrdinalDirection> {
        match self {
            Direction::NorthEast => Some(OrdinalDirection::NorthEast),
            Direction::SouthEast => Some(OrdinalDirection::SouthEast),
            Direction::SouthWest => Some(OrdinalDirection::SouthWest),
            Direction::NorthWest => Some(OrdinalDirection::NorthWest),
            _ => None,
        }
    }
}

impl CardinalDirection {
    pub fn from_unit_coord(coord: Coord) -> Self {
        match coord {
            Coord { x: 1, y: 0 } => CardinalDirection::East,
            Coord { x: -1, y: 0 } => CardinalDirection::West,
            Coord { x: 0, y: 1 } => CardinalDirection::South,
            Coord { x: 0, y: -1 } => CardinalDirection::North,
            _ => panic!("Unexpected coord: {:?}", coord),
        }
    }

    pub fn direction(self) -> Direction {
        match self {
            CardinalDirection::North => Direction::North,
            CardinalDirection::East => Direction::East,
            CardinalDirection::South => Direction::South,
            CardinalDirection::West => Direction::West,
        }
    }

    pub fn opposite(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }

    pub fn coord(self) -> Coord {
        match self {
            CardinalDirection::North => Coord::new(0, -1),
            CardinalDirection::East => Coord::new(1, 0),
            CardinalDirection::South => Coord::new(0, 1),
            CardinalDirection::West => Coord::new(-1, 0),
        }
    }

    pub fn left90(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
        }
    }

    pub fn right90(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
        }
    }

    pub fn left45(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::NorthWest,
            CardinalDirection::East => OrdinalDirection::NorthEast,
            CardinalDirection::South => OrdinalDirection::SouthEast,
            CardinalDirection::West => OrdinalDirection::SouthWest,
        }
    }

    pub fn right45(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::NorthEast,
            CardinalDirection::East => OrdinalDirection::SouthEast,
            CardinalDirection::South => OrdinalDirection::SouthWest,
            CardinalDirection::West => OrdinalDirection::NorthWest,
        }
    }

    pub fn left135(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::SouthWest,
            CardinalDirection::East => OrdinalDirection::NorthWest,
            CardinalDirection::South => OrdinalDirection::NorthEast,
            CardinalDirection::West => OrdinalDirection::SouthEast,
        }
    }

    pub fn right135(self) -> OrdinalDirection {
        match self {
            CardinalDirection::North => OrdinalDirection::SouthEast,
            CardinalDirection::East => OrdinalDirection::SouthWest,
            CardinalDirection::South => OrdinalDirection::NorthWest,
            CardinalDirection::West => OrdinalDirection::NorthEast,
        }
    }
}

impl OrdinalDirection {
    pub fn from_unit_coord(coord: Coord) -> Self {
        match coord {
            Coord { x: 1, y: 1 } => OrdinalDirection::SouthEast,
            Coord { x: 1, y: -1 } => OrdinalDirection::NorthEast,
            Coord { x: -1, y: 1 } => OrdinalDirection::SouthWest,
            Coord { x: -1, y: -1 } => OrdinalDirection::NorthWest,
            _ => panic!("Unexpected coord: {:?}", coord),
        }
    }

    pub fn direction(self) -> Direction {
        match self {
            OrdinalDirection::NorthEast => Direction::NorthEast,
            OrdinalDirection::SouthEast => Direction::SouthEast,
            OrdinalDirection::SouthWest => Direction::SouthWest,
            OrdinalDirection::NorthWest => Direction::NorthWest,
        }
    }

    pub fn opposite(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::SouthWest,
            OrdinalDirection::SouthEast => OrdinalDirection::NorthWest,
            OrdinalDirection::SouthWest => OrdinalDirection::NorthEast,
            OrdinalDirection::NorthWest => OrdinalDirection::SouthEast,
        }
    }

    pub fn coord(self) -> Coord {
        match self {
            OrdinalDirection::NorthEast => Coord::new(1, -1),
            OrdinalDirection::SouthEast => Coord::new(1, 1),
            OrdinalDirection::SouthWest => Coord::new(-1, 1),
            OrdinalDirection::NorthWest => Coord::new(-1, -1),
        }
    }

    pub fn left90(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::NorthWest,
            OrdinalDirection::SouthEast => OrdinalDirection::NorthEast,
            OrdinalDirection::SouthWest => OrdinalDirection::SouthEast,
            OrdinalDirection::NorthWest => OrdinalDirection::SouthWest,
        }
    }

    pub fn right90(self) -> OrdinalDirection {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::SouthEast,
            OrdinalDirection::SouthEast => OrdinalDirection::SouthWest,
            OrdinalDirection::SouthWest => OrdinalDirection::NorthWest,
            OrdinalDirection::NorthWest => OrdinalDirection::NorthEast,
        }
    }

    pub fn left45(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::North,
            OrdinalDirection::SouthEast => CardinalDirection::East,
            OrdinalDirection::SouthWest => CardinalDirection::South,
            OrdinalDirection::NorthWest => CardinalDirection::West,
        }
    }

    pub fn right45(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::East,
            OrdinalDirection::SouthEast => CardinalDirection::South,
            OrdinalDirection::SouthWest => CardinalDirection::West,
            OrdinalDirection::NorthWest => CardinalDirection::North,
        }
    }

    pub fn left135(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::West,
            OrdinalDirection::SouthEast => CardinalDirection::North,
            OrdinalDirection::SouthWest => CardinalDirection::East,
            OrdinalDirection::NorthWest => CardinalDirection::South,
        }
    }

    pub fn right135(self) -> CardinalDirection {
        match self {
            OrdinalDirection::NorthEast => CardinalDirection::South,
            OrdinalDirection::SouthEast => CardinalDirection::West,
            OrdinalDirection::SouthWest => CardinalDirection::North,
            OrdinalDirection::NorthWest => CardinalDirection::East,
        }
    }

    pub fn from_cardinals(a: CardinalDirection, b: CardinalDirection) -> Option<Self> {
        match a {
            CardinalDirection::North => match b {
                CardinalDirection::East => return Some(OrdinalDirection::NorthEast),
                CardinalDirection::West => return Some(OrdinalDirection::NorthWest),
                _ => return None,
            },
            CardinalDirection::East => match b {
                CardinalDirection::North => return Some(OrdinalDirection::NorthEast),
                CardinalDirection::South => return Some(OrdinalDirection::SouthEast),
                _ => return None,
            },
            CardinalDirection::South => match b {
                CardinalDirection::East => return Some(OrdinalDirection::SouthEast),
                CardinalDirection::West => return Some(OrdinalDirection::SouthWest),
                _ => return None,
            },
            CardinalDirection::West => match b {
                CardinalDirection::North => return Some(OrdinalDirection::NorthWest),
                CardinalDirection::South => return Some(OrdinalDirection::SouthWest),
                _ => return None,
            },
        }
    }

    pub fn to_cardinals(self) -> (CardinalDirection, CardinalDirection) {
        use self::CardinalDirection::*;
        use self::OrdinalDirection::*;
        match self {
            NorthEast => (North, East),
            SouthEast => (East, South),
            SouthWest => (South, West),
            NorthWest => (West, North),
        }
    }

    pub fn cardinal_bitmap(self) -> DirectionBitmap {
        let (a, b) = self.to_cardinals();
        a.direction().bitmap() | b.direction().bitmap()
    }
}

impl From<CardinalDirection> for Direction {
    fn from(c: CardinalDirection) -> Self {
        c.direction()
    }
}

impl From<OrdinalDirection> for Direction {
    fn from(o: OrdinalDirection) -> Self {
        o.direction()
    }
}

macro_rules! make_direction_iter {
    ($col_name:ident, $iter_name:ident, $type:ident, $count:expr) => {
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone)]
        /// Iterator over all directions of the respectively-named type of direction
        pub struct $iter_name(Range<u8>);
        impl Iterator for $iter_name {
            type Item = $type;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|n| unsafe { mem::transmute(n) })
            }
        }

        /// Represents a collection of the respectively-named type of direction
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone, Copy)]
        pub struct $col_name;
        impl IntoIterator for $col_name {
            type Item = $type;
            type IntoIter = $iter_name;
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(0..$count as u8)
            }
        }
    };
}

// IntoIter implementations for iterating over all directions of a type. E.g.:
// for direction in CardinalDirections { ... }
make_direction_iter!{Directions, DirectionIter, Direction, NUM_DIRECTIONS}
make_direction_iter!{CardinalDirections, CardinalDirectionIter, CardinalDirection, NUM_CARDINAL_DIRECTIONS}
make_direction_iter!{OrdinalDirections, OrdinalDirectionIter, OrdinalDirection, NUM_ORDINAL_DIRECTIONS}

macro_rules! make_subdirection_iter {
    ($col_name:ident, $backing_col_name:ident, $iter_name:ident, $backing_iter_name:ident) => {
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone)]
        /// Iterator over a particular collection of `Direction`s
        pub struct $iter_name($backing_iter_name);
        impl Iterator for $iter_name {
            type Item = Direction;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|d| d.direction())
            }
        }

        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Debug, Clone, Copy)]
        /// Represents a particular collection of `Direction`s
        pub struct $col_name;
        impl IntoIterator for $col_name {
            type Item = Direction;
            type IntoIter = $iter_name;
            fn into_iter(self) -> Self::IntoIter {
                $iter_name($backing_col_name.into_iter())
            }
        }
    };
}

// IntoIter implementations for iterating over a subset of directions. E.g.:
// for direction in DirectionsCardinal { ... }
make_subdirection_iter!{
    DirectionsCardinal,
    CardinalDirections,
    DirectionCardinalIter,
    CardinalDirectionIter
}
make_subdirection_iter!{
    DirectionsOrdinal,
    OrdinalDirections,
    DirectionOrdinalIter,
    OrdinalDirectionIter
}

/// Set of directions implemented as a bitmap
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DirectionBitmap {
    pub raw: u8,
}

impl DirectionBitmap {
    pub fn new(raw: u8) -> Self {
        Self { raw }
    }

    pub fn empty() -> Self {
        NO_DIRECTIONS_BITMAP
    }

    pub fn all() -> Self {
        ALL_DIRECTIONS_BITMAP
    }

    pub fn all_cardinal() -> Self {
        ALL_CARDINAL_DIRECTIONS_BITMAP
    }
    pub fn all_ordinal() -> Self {
        ALL_ORDINAL_DIRECTIONS_BITMAP
    }

    pub fn has(self, direction: Direction) -> bool {
        self.raw & (1 << direction as usize) != 0
    }

    pub fn is_empty(self) -> bool {
        self.raw == NO_DIRECTIONS_BITMAP_RAW
    }

    pub fn is_full(self) -> bool {
        self.raw == ALL_DIRECTIONS_BITMAP_RAW
    }
}

impl Default for DirectionBitmap {
    fn default() -> Self {
        Self::empty()
    }
}

impl BitOr for DirectionBitmap {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        DirectionBitmap::new(self.raw | rhs.raw)
    }
}

impl BitOrAssign for DirectionBitmap {
    fn bitor_assign(&mut self, rhs: Self) {
        self.raw |= rhs.raw;
    }
}

impl BitAnd for DirectionBitmap {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        DirectionBitmap::new(self.raw & rhs.raw)
    }
}

impl BitAndAssign for DirectionBitmap {
    fn bitand_assign(&mut self, rhs: Self) {
        self.raw &= rhs.raw;
    }
}

impl From<CardinalDirection> for [i32; 2] {
    fn from(c: CardinalDirection) -> [i32; 2] {
        use self::CardinalDirection::*;
        match c {
            North => [0, -1],
            East => [1, 0],
            South => [0, 1],
            West => [-1, 0],
        }
    }
}
impl From<CardinalDirection> for (i32, i32) {
    fn from(c: CardinalDirection) -> (i32, i32) {
        use self::CardinalDirection::*;
        match c {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }
}

impl From<OrdinalDirection> for [i32; 2] {
    fn from(o: OrdinalDirection) -> [i32; 2] {
        use self::OrdinalDirection::*;
        match o {
            NorthWest => [-1, -1],
            NorthEast => [1, -1],
            SouthEast => [1, 1],
            SouthWest => [-1, 1],
        }
    }
}
impl From<OrdinalDirection> for (i32, i32) {
    fn from(o: OrdinalDirection) -> (i32, i32) {
        use self::OrdinalDirection::*;
        match o {
            NorthWest => (-1, -1),
            NorthEast => (1, -1),
            SouthEast => (1, 1),
            SouthWest => (-1, 1),
        }
    }
}

impl From<Direction> for [i32; 2] {
    fn from(d: Direction) -> [i32; 2] {
        use self::Direction::*;
        match d {
            North => [0, -1],
            East => [1, 0],
            South => [0, 1],
            West => [-1, 0],
            NorthWest => [-1, -1],
            NorthEast => [1, -1],
            SouthEast => [1, 1],
            SouthWest => [-1, 1],
        }
    }
}
impl From<Direction> for (i32, i32) {
    fn from(d: Direction) -> (i32, i32) {
        use self::Direction::*;
        match d {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
            NorthEast => (1, -1),
            SouthEast => (1, 1),
            SouthWest => (-1, 1),
        }
    }
}

pub type DirectionTableIter<'a, T> = slice::Iter<'a, T>;
pub type DirectionTableIterMut<'a, T> = slice::IterMut<'a, T>;

macro_rules! make_direction_table {
    ($table_type:ident, $direction_type:ident, $direction_iter:ident, $count:expr) => {
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $table_type<T> {
            values: [T; $count],
        }
        impl<T> $table_type<T> {
            unsafe fn new_uninitialized() -> Self {
                mem::uninitialized()
            }
            pub fn new_fn<F: FnMut($direction_type) -> T>(mut f: F) -> Self {
                let mut table = unsafe { Self::new_uninitialized() };
                for direction in $direction_iter {
                    table.set(direction, f(direction));
                }
                table
            }
            pub fn set(&mut self, direction: $direction_type, value: T) {
                self.values[direction as usize] = value;
            }
            pub fn get(&self, direction: $direction_type) -> &T {
                &self.values[direction as usize]
            }
            pub fn get_mut(&mut self, direction: $direction_type) -> &mut T {
                &mut self.values[direction as usize]
            }
            pub fn iter(&self) -> DirectionTableIter<T> {
                self.values.iter()
            }
            pub fn iter_mut(&mut self) -> DirectionTableIterMut<T> {
                self.values.iter_mut()
            }
        }
        impl<T: Clone> $table_type<T> {
            pub fn new_clone(value: T) -> Self {
                let mut table = unsafe { Self::new_uninitialized() };
                for direction in $direction_iter {
                    table.set(direction, value.clone());
                }
                table
            }
        }
        impl<T: Default> $table_type<T> {
            pub fn new_default() -> Self {
                let mut table = unsafe { Self::new_uninitialized() };
                for direction in $direction_iter {
                    table.set(direction, Default::default());
                }
                table
            }
        }
    };
}

make_direction_table!(DirectionTable, Direction, Directions, NUM_DIRECTIONS);
make_direction_table!(
    CardinalDirectionTable,
    CardinalDirection,
    CardinalDirections,
    NUM_CARDINAL_DIRECTIONS
);
make_direction_table!(
    OrdinalDirectionTable,
    OrdinalDirection,
    OrdinalDirections,
    NUM_ORDINAL_DIRECTIONS
);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn iteration() {
        use Direction::*;
        assert_eq!(
            CardinalDirections.into_iter().collect::<Vec<_>>(),
            vec![North, East, South, West]
        )
    }
}

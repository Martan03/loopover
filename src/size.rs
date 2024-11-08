use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub,
        SubAssign,
    },
};

use serde::{Deserialize, Serialize};
use termint::geometry::Coords;

/// A 2D vector implementing basic operations
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    /// Creates new [`Size`] containing given width and height
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            width: x,
            height: y,
        }
    }

    /// Saturating [`Size`] subtraction. Computes `self - rhs`, saturating at
    /// the numeric bounds instead of overlowing
    pub fn saturating_sub<T>(&self, rhs: T) -> Self
    where
        T: Into<Self>,
    {
        let rhs = rhs.into();
        Self {
            width: self.width.saturating_sub(rhs.width),
            height: self.height.saturating_sub(rhs.height),
        }
    }

    /// Checked [`Size`] subtraction. Computes `self - rhs`, returning `None`
    /// if overflow occured.
    pub fn checked_sub<T>(&self, rhs: T) -> Option<Self>
    where
        T: Into<Self>,
    {
        let rhs = rhs.into();
        Some(Self {
            width: self.width.checked_sub(rhs.width)?,
            height: self.height.checked_sub(rhs.height)?,
        })
    }

    /// Transpones [`Size`]
    pub fn transpone(&mut self) {
        (self.width, self.height) = (self.height, self.width);
    }

    /// Transpones [`Size`] and returns new [`Size`]
    pub fn inverse(&self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 3,
            height: 3,
        }
    }
}

impl Index<usize> for Size {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.width,
            1 => &self.height,
            _ => panic!("index {index} is out of bounds for Coords"),
        }
    }
}

impl IndexMut<usize> for Size {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.width,
            1 => &mut self.height,
            _ => panic!("index {index} is out of bounds for Coords"),
        }
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.width < other.width && self.height < other.height {
            Some(std::cmp::Ordering::Less)
        } else if self.width > other.width && self.height > other.height {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl Ord for Size {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (
            self.width < other.width && self.height < other.height,
            self.width > other.width && self.height > other.height,
        ) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl Add<Size> for Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        Size::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl AddAssign<Size> for Size {
    fn add_assign(&mut self, rhs: Size) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl Sub<Size> for Size {
    type Output = Size;

    fn sub(self, rhs: Size) -> Self::Output {
        Size::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl SubAssign<Size> for Size {
    fn sub_assign(&mut self, rhs: Size) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl Mul<Size> for Size {
    type Output = Size;

    fn mul(self, rhs: Size) -> Self::Output {
        Size::new(self.width * rhs.width, self.height * rhs.height)
    }
}

impl MulAssign<Size> for Size {
    fn mul_assign(&mut self, rhs: Size) {
        self.width *= rhs.width;
        self.height *= rhs.height;
    }
}

impl Div<Size> for Size {
    type Output = Size;

    fn div(self, rhs: Size) -> Self::Output {
        Size::new(self.width / rhs.width, self.height / rhs.height)
    }
}

impl DivAssign<Size> for Size {
    fn div_assign(&mut self, rhs: Size) {
        self.width /= rhs.width;
        self.height /= rhs.height;
    }
}

impl From<(usize, usize)> for Size {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl From<[usize; 2]> for Size {
    fn from([x, y]: [usize; 2]) -> Self {
        Self::new(x, y)
    }
}

impl From<Size> for Coords {
    fn from(value: Size) -> Self {
        Coords::new(value.width, value.height)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.width, self.height)
    }
}

use std::{
    convert::{
        TryInto,
    },
    ops::{
        Add,
        AddAssign,
        Div,
        Mul,
        Sub
    }
};

use super::{
    SignCheck,
    Zero
};

#[derive(PartialEq, Default, Debug, Copy, Clone)]
pub struct Size<T> where
  T: SignCheck + Zero<T> + Copy + PartialEq
{
    width: T,
    height: T
}

impl<T> Add for Size<T> where
  T: Add<Output = T> + SignCheck + Zero<T> + Copy + PartialEq
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            width: self.width + other.width,
            height: self.height + other.height
        }
    }
}

impl<T> AddAssign for Size<T> where
  T: Add<Output = T> + SignCheck + Zero<T> + Copy + PartialEq
{
    fn add_assign(&mut self, other: Self) {
        self.width = self.width + other.width;
        self.height = self.height + other.height;
    }
}

impl<T> AddAssign<T> for Size<T> where
  T: Add<Output = T> + SignCheck + Zero<T> + Copy + PartialEq
{
    fn add_assign(&mut self, other: T) {
        self.width = self.width + other;
        self.height = self.height + other;
    }
}

impl<T> Sub for Size<T> where
  T: Sub<Output = T> + SignCheck + Zero<T> + Copy + PartialEq
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T> Div for Size<T> where
  T: Div<Output = T> + SignCheck + Zero<T> + Copy + PartialEq
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

impl<T> Mul for Size<T> where
  T: Mul<Output = T> + SignCheck + Zero<T> + Copy + PartialEq
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl<T> From<(T, T)> for Size<T> where
  T: SignCheck + Zero<T> + Copy + PartialEq
{
    fn from(tuple: (T, T)) -> Self {
        Size {
            width: tuple.0,
            height: tuple.1
        }
    }
}

impl<T> Size<T> where
  T: SignCheck + Zero<T> + Default + Copy + PartialEq
{
    pub fn new(width: T, height: T) -> Result<Self, &'static str> {
        if width.is_negative() {
            return Err("Size doesn't accepts negative values. Width can't be negative.");
        }

        if height.is_negative() {
            return Err("Size doesn't accepts negative values. Height can't be negative.");
        }

        Ok(Size {
            width,
            height
        })
    }

    pub fn with(width: T, height: T) -> Self {
        Size::new(
            {
                if width.is_negative() {
                    T::zero()
                } else {
                    width
                }
            },
            {
                if height.is_negative() {
                    T::zero()
                } else {
                    height
                }
            }
        ).unwrap_or_default()
    }

    pub fn try_with<U: TryInto<T, Error = E>, E>(width: U, height: U) -> Result<Self, E> {
        let w_component = match width.try_into() {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        let h_component = match height.try_into() {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        Ok(Size::with(w_component, h_component))
    }
}

impl<T> Size<T> where
  T: SignCheck + Zero<T> + Copy + PartialEq
{
    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }

    pub fn mut_width(&mut self) -> &mut T {
        &mut self.width
    }

    pub fn mut_height(&mut self) -> &mut T {
        &mut self.height
    }

    pub fn set_width(&mut self, width: T) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: T) {
        self.height = height;
    }

    pub fn set(&mut self, width: T, height: T) {
        self.width = width;
        self.height = height;
    }
}

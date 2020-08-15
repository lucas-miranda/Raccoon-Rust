use std::{
    convert::{
        TryInto,
    },
    ops::{
        Add,
        AddAssign,
        Div,
        Index,
        IndexMut,
        Mul,
        Sub
    }
};

#[derive(PartialEq, Default, Debug, Copy, Clone)]
pub struct Vector2<T> where 
  T: Copy + PartialEq
{
    x: T,
    y: T
}

impl<T> Add for Vector2<T> where
  T: Add<Output = T> + Copy + PartialEq,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Vector2<T> where
  T: Add<Output = T> + Copy + PartialEq
{
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T> AddAssign<T> for Vector2<T> where
  T: Add<Output = T> + Copy + PartialEq
{
    fn add_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
    }
}

impl<T> Sub for Vector2<T> where
  T: Sub<Output = T> + Copy + PartialEq,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Div for Vector2<T> where
  T: Div<Output = T> + Copy + PartialEq,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> Mul for Vector2<T> where
  T: Mul<Output = T> + Copy + PartialEq,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Index<usize> for Vector2<T> where
  T: Copy + PartialEq
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            return &self.x;
        } else if index == 1 {
            return &self.y;
        }

        panic!("Index is out of range, accepted indexes are [0, 1]");
    }
}

impl<T> IndexMut<usize> for Vector2<T> where
  T: Copy + PartialEq
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            return &mut self.x;
        } else if index == 1 {
            return &mut self.y;
        }

        panic!("Index is out of range, accepted indexes are [0, 1]");
    }
}

impl<T> From<(T, T)> for Vector2<T> where
  T: Copy + PartialEq
{
    fn from(tuple: (T, T)) -> Self {
        Vector2 {
            x: tuple.0,
            y: tuple.1
        }
    }
}

impl<T> Vector2<T> where 
  T: Default + Copy + PartialEq
{
    pub fn new() -> Self {
        Vector2::<T>::default()
    }

    pub fn with(x: T, y: T) -> Self {
        Vector2 {
            x,
            y
        }
    }

    pub fn try_with<U: TryInto<T, Error = E>, E>(x: U, y: U) -> Result<Self, E> {
        let x_component = match x.try_into() {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        let y_component = match y.try_into() {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        Ok(Vector2::with(x_component, y_component))
    }
}

impl<T> Vector2<T> where
  T: Copy + PartialEq
{
    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn mut_x(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn mut_y(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

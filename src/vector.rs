use num_traits::{One, Zero, Float};
use std::default::Default;
use std::ops::{Add, Sub, Mul};

// Define a 2D vector struct
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

// Define a 3D vector struct
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Define a trait for vector operations
pub trait Vector<T>: Sized {
    // Create a vector with all components set to zero
    fn zeroes() -> Self;

    // Create a vector with all components set to one
    fn ones() -> Self;

    // Add two vectors
    fn add(&self, other: &Self) -> Self;

    // Subtract one vector from another
    fn sub(&self, other: &Self) -> Self;

    // Scale a vector by a scalar
    fn scale(&self, scalar: T) -> Self;

    // Compute the dot product of two vectors
    fn dot(&self, other: &Self) -> T;

    // Compute the magnitude of the vector
    fn magnitude(&self) -> T where T: Float;
}

// Implement the Vector trait for Vector2
impl<T> Vector<T> for Vector2<T>
where
    T: Default + One + Zero + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn zeroes() -> Self {
        Vector2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn ones() -> Self {
        Vector2 {
            x: T::one(),
            y: T::one(),
        }
    }

    fn add(&self, other: &Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn scale(&self, scalar: T) -> Self {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn dot(&self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }

    fn magnitude(&self) -> T
    where
        T: Float,
    {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Implement the Vector trait for Vector3
impl<T> Vector<T> for Vector3<T>
where
    T: Default + One + Zero + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn zeroes() -> Self {
        Vector3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn ones() -> Self {
        Vector3 {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }

    fn add(&self, other: &Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scale(&self, scalar: T) -> Self {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn dot(&self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    fn magnitude(&self) -> T
    where
        T: Float,
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}
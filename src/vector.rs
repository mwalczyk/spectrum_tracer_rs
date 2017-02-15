extern crate rand;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn origin() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random_in_unit_sphere() -> Vector {
        // rejection method for finding a random point in a
        // unit sphere: pick a point inside of the unit cube
        // and return if it is also inside of the unit sphere
        let mut rng = rand::thread_rng();
        let mut p = Vector::origin();
        loop {
            p = Vector {
                x: rng.next_f64(),
                y: rng.next_f64(),
                z: rng.next_f64(),
            } * 2.0 -
                Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            if p.squared_length() <= 1.0 {
                break;
            }
        }
        p
    }
}

// Vector + Vector
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Vector + f64
impl Add<f64> for Vector {
    type Output = Vector;

    fn add(self, other: f64) -> Vector {
        Vector {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

// Vector += Vector
impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vector - Vector
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Vector - f64
impl Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, other: f64) -> Vector {
        Vector {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

// Vector -= Vector
impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Vector * Vector
impl Mul for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// Vector * f64
impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// Vector *= f64
impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

// Vector / Vector
impl Div for Vector {
    type Output = Vector;

    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// Vector / f64
impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

// Vector /= f64
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

// -Vector
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub fn dot(lhs: Vector, rhs: Vector) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn cross(lhs: Vector, rhs: Vector) -> Vector {
    Vector {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: -(lhs.x * rhs.z - lhs.z * rhs.x),
        z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
}

#[test]
fn test_add() {
    let lhs = Vector {
        x: 0.0,
        y: 1.0,
        z: 2.0,
    };
    let rhs = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(lhs + rhs,
               Vector {
                   x: 1.0,
                   y: 3.0,
                   z: 5.0,
               });
}

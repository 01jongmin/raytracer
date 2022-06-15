use crate::utils::{ random_double, random_double_range };
use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Neg};
use core::fmt;
use std::iter::Sum;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\n", 
               (f64::sqrt(self.x) * 255.999) as i64, 
               (f64::sqrt(self.y) * 255.999) as i64, 
               (f64::sqrt(self.z) * 255.999) as i64)
    }
}

impl Vec3 {
    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1., 1.);
            if p.length_squared() >= 1. { continue; }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn rgb(&self) -> Vec<u8> {
        vec![(f64::sqrt(self.x()) * 255.999) as u8,
             (f64::sqrt(self.y()) * 255.999) as u8,
             (f64::sqrt(self.z()) * 255.999) as u8]
    }
}

impl Vec3 {
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * 2. * Vec3::dot(&v , &n)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn constant_new(constant: f64) -> Vec3 {
        Vec3 { x: constant, 
               y: constant, 
               z: constant,
            }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.div(self.length())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Vec3 {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x();
        self.y += other.y();
        self.z += other.z();
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Vec3 {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x();
        self.y -= other.y();
        self.z -= other.z();
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Vec3 {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}


impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x();
        self.y /= other.y();
        self.z /= other.z();
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Vec3 {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x();
        self.y *= other.y();
        self.z *= other.z();
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self 
         where
        I: Iterator<Item = Self>
    {
        iter.fold(Vec3::constant_new(0.0), |a, b| a + b)
    }
}

//impl std::iter::Iterator::sum for Vec3 {

//}
//pub fn build_point(x: f64, y: f64, z: f64) -> Vec3 {
        //Vec3 {
            //x,
            //y,
            //z,
            //// if there was w then assign 1
        //}
    //}

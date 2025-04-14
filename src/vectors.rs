use std::ops::{Add, Sub, Mul, Div, Neg};

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

type Ray = Vec3;

impl Ray {
    
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            x, y, z
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

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn print(&self) -> () {
        eprintln!("x: {}, y: {}, z: {} | len: {}", self.x, self.y, self.z, self.len());
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z
        } }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, s: f64) -> Vec3 {
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

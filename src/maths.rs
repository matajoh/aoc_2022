use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Rot3 {
    pub m: [[i32; 3]; 3],
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub<i32> for Vec2 {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        Vec2 {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div<i32> for Vec2 {
    type Output = Self;

    fn div(self, other: i32) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Add<i32> for Vec2 {
    type Output = Self;

    fn add(self, other: i32) -> Vec2 {
        Vec2 {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = i32;

    fn mul(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul for Rot3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut m = [[0; 3]; 3];
        for r in 0..3 {
            for c in 0..3 {
                for i in 0..3 {
                    m[r][c] += self.m[r][i] * other.m[i][c]
                }
            }
        }
        Rot3 { m }
    }
}

impl Mul<Vec3> for Rot3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.m[0][0] * other.x + self.m[0][1] * other.y + self.m[0][2] * other.z,
            y: self.m[1][0] * other.x + self.m[1][1] * other.y + self.m[1][2] * other.z,
            z: self.m[2][0] * other.x + self.m[2][1] * other.y + self.m[2][2] * other.z,
        }
    }
}

impl Rot3 {
    pub fn identity() -> Rot3 {
        Rot3 {
            m: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        }
    }
}

fn clip(a: i32, limit: i32) -> i32 {
    if a < -limit {
        -limit
    } else if a > limit {
        limit
    } else {
        a
    }
}

impl Vec2 {
    pub fn from(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn len(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

impl Vec3 {
    pub fn clip(&self, limit: i32) -> Vec3 {
        Vec3 {
            x: clip(self.x, limit),
            y: clip(self.y, limit),
            z: clip(self.z, limit),
        }
    }
}

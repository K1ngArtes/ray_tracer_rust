use std::ops;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[test]
fn add_vec3_test() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };

    let v3 = v1 + v2;

    assert_eq!(
        Vec3 {
            x: 5.0,
            y: 7.0,
            z: 9.0
        },
        v3
    );
}

#[test]
fn sub_vec3_test() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };

    let v3 = v1 - v2;

    assert_eq!(
        Vec3 {
            x: -3.0,
            y: -3.0,
            z: -3.0
        },
        v3
    );
}

#[test]
fn neg_vec3_test() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let v3 = -v1;

    assert_eq!(
        Vec3 {
            x: -1.0,
            y: -2.0,
            z: -3.0
        },
        v3
    );
}

#[test]
fn mul_vec3_test() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let v3 = 3.0 * v1;

    assert_eq!(
        Vec3 {
            x: 3.0,
            y: 6.0,
            z: 9.0
        },
        v3
    );
}

#[test]
fn div_vec3_test() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let v3 = v1 / 3.0;

    assert_eq!(
        Vec3 {
            x: v1.x / 3.0,
            y: v1.y / 3.0,
            z: v1.z / 3.0
        },
        v3
    );
}

#[test]
fn vec3_length_test() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    assert_eq!(3.7416573867739413, v1.length());
}

#[test]
fn vec3_dot_test() {
    let v1 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };

    let v3 = v1.dot(v2);
    let v4 = v2.dot(v1);

    assert_eq!(v3, v4);
    assert_eq!(
        Vec3 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z
        },
        v3
    );
}

#[test]
fn vec3_cross_test() {
    let v1 = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };
    let v2 = Vec3 {
        x: 5.0,
        y: 6.0,
        z: 7.0,
    };

    let v3 = v1.cross(v2);

    assert_eq!(
        Vec3 {
            x: -3.0,
            y: 6.0,
            z: -3.0
        },
        v3
    );
}
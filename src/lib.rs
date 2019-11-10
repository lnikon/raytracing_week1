//pub mod vec3 {

use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn empty() -> Vec3 {
        let v = Vec3 { e: [0.0, 0.0, 0.0] };
        v
    }

    pub fn build(e0: f64, e1: f64, e2: f64) -> Vec3 {
        let mut v = Vec3::empty();
        v.e[0] = e0;
        v.e[1] = e1;
        v.e[2] = e2;
        v
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        Vec3::x(&self)
    }

    pub fn g(&self) -> f64 {
        Vec3::y(&self)
    }

    pub fn b(&self) -> f64 {
        Vec3::z(&self)
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0].sqrt() + self.e[1].sqrt() + self.e[2].sqrt()
    }

    pub fn length(&self) -> f64 {
        Vec3::squared_length(&self).sqrt()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::build(
            self.e[0] + _rhs.e[0],
            self.e[1] + _rhs.e[1],
            self.e[2] + _rhs.e[2],
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::build(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        self + (-_rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::build(
            self.e[0] * _rhs.e[0],
            self.e[1] * _rhs.e[1],
            self.e[2] * _rhs.e[2],
        )
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3::build(
            self.e[0] / _rhs.e[0],
            self.e[1] / _rhs.e[1],
            self.e[2] / _rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::build(self.e[0] * _rhs, self.e[1] * _rhs, self.e[2] * _rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3::build(self.e[0] / _rhs, self.e[1] / _rhs, self.e[2] / _rhs)
    }
}

pub enum Coord {
    X,
    Y,
    Z,
}

impl Index<Coord> for Vec3 {
    type Output = f64;
    fn index(&self, coord: Coord) -> &Self::Output {
        match coord {
            Coord::X => &self.e[0],
            Coord::Y => &self.e[1],
            Coord::Z => &self.e[2],
        }
    }
}

impl IndexMut<Coord> for Vec3 {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        match coord {
            Coord::X => &mut self.e[0],
            Coord::Y => &mut self.e[1],
            Coord::Z => &mut self.e[2],
        }
    }
}

pub enum Channel {
    R,
    G,
    B,
}

impl Index<Channel> for Vec3 {
    type Output = f64;
    fn index(&self, channel: Channel) -> &Self::Output {
        match channel {
            Channel::R => &self.e[0],
            Channel::G => &self.e[1],
            Channel::B => &self.e[2],
        }
    }
}

impl IndexMut<Channel> for Vec3 {
    fn index_mut(&mut self, channel: Channel) -> &mut Self::Output {
        match channel {
            Channel::R => &mut self.e[0],
            Channel::G => &mut self.e[1],
            Channel::B => &mut self.e[2],
        }
    }
}
//}

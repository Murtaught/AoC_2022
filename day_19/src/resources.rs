use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Resources {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
        self.d += rhs.d;
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.a = self.a.saturating_sub(rhs.a);
        self.b = self.b.saturating_sub(rhs.b);
        self.c = self.c.saturating_sub(rhs.c);
        self.d = self.d.saturating_sub(rhs.d);
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> MulAssign<T> for Resources
where
    T: TryInto<u16>,
    T::Error: Debug,
{
    fn mul_assign(&mut self, x: T) {
        let x: u16 = x.try_into().unwrap();
        self.a *= x;
        self.b *= x;
        self.c *= x;
        self.d *= x;
    }
}

impl<T> Mul<T> for Resources
where
    T: TryInto<u16>,
    T::Error: Debug,
{
    type Output = Resources;

    fn mul(mut self, x: T) -> Self::Output {
        self *= x;
        self
    }
}

impl IntoIterator for Resources {
    type Item = u16;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.a, self.b, self.c, self.d].into_iter()
    }
}

impl Resources {
    pub fn can_afford(&self, rhs: &Resources) -> bool {
        self.a >= rhs.a
            && self.b >= rhs.b
            && self.c >= rhs.c
            && self.d >= rhs.d
    }

    pub fn ticks_needed(&self, inc: &Resources) -> Option<u8> {
        let mut ret = 0;
        for (have, inc) in self.into_iter().zip(inc.into_iter()) {
            if have == 0 {
                continue;
            }

            if inc == 0 {
                return None;
            }

            ret = ret.max((have + inc - 1) / inc);
        }
        ret.try_into().ok()
    }
}

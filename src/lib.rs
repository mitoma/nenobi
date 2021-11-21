use std::f64::consts::PI;

use num_traits::{Float, PrimInt};

pub trait EasingFunction<T: Float> {
    fn s_value(&self, x: T) -> T {
        Self::value(x)
    }

    fn value(x: T) -> T {
        if T::zero() >= x {
            T::zero()
        } else if T::one() <= x {
            T::one()
        } else {
            Self::inner_value(x)
        }
    }

    fn inner_value(x: T) -> T;
}

pub struct Liner {}

impl<T: Float> EasingFunction<T> for Liner {
    fn inner_value(x: T) -> T {
        x
    }
}

pub struct Sin {}

impl<T: Float> EasingFunction<T> for Sin {
    fn inner_value(x: T) -> T {
        vi::<i32, T>(1) - (x * vf(PI) / vi(2)).cos()
    }
}

pub struct Quad {}

impl<T: Float> EasingFunction<T> for Quad {
    fn inner_value(x: T) -> T {
        x * x
    }
}

pub struct Cubec {}

impl<T: Float> EasingFunction<T> for Cubec {
    fn inner_value(x: T) -> T {
        x * x * x
    }
}

pub struct Quart {}

impl<T: Float> EasingFunction<T> for Quart {
    fn inner_value(x: T) -> T {
        x * x * x * x
    }
}

pub struct Quint {}

impl<T: Float> EasingFunction<T> for Quint {
    fn inner_value(x: T) -> T {
        x * x * x * x * x
    }
}

pub struct Expo {}

impl<T: Float> EasingFunction<T> for Expo {
    fn inner_value(x: T) -> T {
        vi::<i32, T>(2).powf(vi::<i32, T>(10) * x - vi::<i32, T>(10))
    }
}

pub struct Circ {}

impl<T: Float> EasingFunction<T> for Circ {
    fn inner_value(x: T) -> T {
        vi::<i32, T>(1) - (vi::<i32, T>(1) - x.powi(2)).sqrt()
    }
}

pub struct Back {}

const BACK_C1: f64 = 1.70158;
const BACK_C3: f64 = BACK_C1 + 1.0;

impl<T: Float> EasingFunction<T> for Back {
    fn inner_value(x: T) -> T {
        vf::<T>(BACK_C3) * x * x * x - vf::<T>(BACK_C1) * x * x
    }
}

pub struct Elastic {}

const EXPO: f64 = 2.0 * PI / 3.0;

impl<T: Float> EasingFunction<T> for Elastic {
    fn inner_value(x: T) -> T {
        -vi::<i32, T>(2).powf(vi::<i32, T>(10) * x - vi::<i32, T>(10))
            * ((x * vi::<i32, T>(10) - vf::<T>(10.75)) * vf::<T>(EXPO)).sin()
    }
}

pub struct Bounce {}

const BOUNCE_N1: f64 = 7.5625;
const BOUNCE_D1: f64 = 2.75;

impl<T: Float> EasingFunction<T> for Bounce {
    fn inner_value(x: T) -> T {
        let x: T = vf::<T>(1.0) - x;
        let v1: T = vf(1.0);
        let v2: T = vf(2.0);
        let v2_5: T = vf(2.5);
        let n1: T = vf(BOUNCE_N1);
        let d1: T = vf(BOUNCE_D1);
        let result = if x < v1 / d1 {
            n1 * x * x
        } else if x < v2 / d1 {
            let x = x - (vf::<T>(1.5) / d1);
            n1 * x * x + vf(0.75)
        } else if x < v2_5 / d1 {
            let x = x - (vf::<T>(2.25) / d1);
            n1 * x * x + vf(0.9375)
        } else {
            let x = x - (vf::<T>(2.625) / d1);
            n1 * x * x + vf(0.984375)
        };
        v1 - result
    }
}

#[inline]
fn vf<T: Float>(x: f64) -> T {
    T::from(x).unwrap()
}

#[inline]
fn vi<T: PrimInt, U: Float>(x: T) -> U {
    U::from(x).unwrap()
}
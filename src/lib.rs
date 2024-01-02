#![feature(let_chains)]
pub mod array_access;

pub mod epx;

#[inline]
fn dot<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

fn kmul(x: [f32; 3], k: f32) -> [f32; 3] {
    x.map(|v| v * k)
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| a[i] + b[i])
}

/// Spherical linear interpolation between a and b.
pub fn slerp(a: &[f32; 3], b: &[f32; 3], t: f32) -> [f32; 3] {
    let cos_theta = dot(a, b);
    let theta = cos_theta.acos();
    let sin_theta = theta.sin();

    add(
        kmul(*a, (theta * (1. - t)).sin() / sin_theta),
        kmul(*b, (theta * t).sin() / sin_theta),
    )
}

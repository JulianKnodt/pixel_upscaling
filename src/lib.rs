#![feature(let_chains)]
pub mod array_access;

pub mod epx;

#[inline]
pub(crate) fn dot<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

#[inline]
pub(crate) fn option_dot<const N: usize>(a: Option<&[f32; N]>, b: Option<&[f32; N]>) -> f32 {
    let Some(a) = a else {
        return 0.;
    };
    let Some(b) = b else {
        return 0.;
    };

    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

fn kmul<const N: usize>(x: [f32; N], k: f32) -> [f32; N] {
    x.map(|v| v * k)
}

fn add<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    std::array::from_fn(|i| a[i] + b[i])
}

/// Spherical linear interpolation between a and b.
pub fn slerp<const N: usize>(a: &[f32; N], b: &[f32; N], t: f32) -> [f32; N] {
    let cos_theta = dot(a, b);
    let theta = cos_theta.acos();
    let sin_theta = theta.sin();

    add(
        kmul(*a, (theta * (1. - t)).sin() / sin_theta),
        kmul(*b, (theta * t).sin() / sin_theta),
    )
}

pub fn lerp<const N: usize>(a: &[f32; N], b: &[f32; N], t: f32) -> [f32; N] {
    add(kmul(*a, 1. - t), kmul(*b, t))
}

pub trait Upscale {
    /// Upscales the image in `data`, writing the output into `out`
    fn upscale<const N: usize>(
        data: &[[f32; N]],
        w: usize,
        h: usize,
        out: &mut Vec<[f32; N]>,
        sim: SimilarityKind,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityKind {
    Bool,
    Fuzzy,
}

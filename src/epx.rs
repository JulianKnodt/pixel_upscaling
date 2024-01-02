use crate::{lerp, option_dot, SimilarityKind, Upscale};

use crate::array_access::{idx_xy, Slice2D};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Epx;

impl Upscale for Epx {
    fn upscale<const CHANNELS: usize>(
        data: &[[f32; CHANNELS]],
        width: usize,
        height: usize,
        out: &mut Vec<[f32; CHANNELS]>,
        sim: SimilarityKind,
    ) {
        epx(data, width, height, out, sim)
    }
}

#[inline]
pub fn epx<const CHANNELS: usize>(
    data: &[[f32; CHANNELS]],
    width: usize,
    height: usize,
    out: &mut Vec<[f32; CHANNELS]>,
    sim: SimilarityKind,
) {
    let pix = Slice2D::new(data, width, height);

    let out_w = width * 2;
    let out_h = height * 2;
    out.resize(out_w * out_h, [0.; CHANNELS]);

    macro_rules! rule {
      ($v: ident if $($e0: ident = $e1: ident),* , $(!$n0: ident = $n1: ident),* else $o: ident) => {
        match sim {
          SimilarityKind::Bool => {
            if $($e0 == $e1 &&)* $($n0 != $n1 &&)* true && let Some(v) = $v {
              *v
            } else {
              *$o
            }
          }
          SimilarityKind::Fuzzy => {
            if let Some(v) = $v {
                let sim = $(option_dot($e0, $e1) *)* $((1. - option_dot($n0, $n1).max(0.)) *)* 1.;
                let sim = sim.max(0.).min(1.);
                lerp($o, v, sim)
            } else {
                *$o
            }
          }
        }

      }
    }

    for y in 0..height {
        for x in 0..width {
            let og = pix.get(x, y);
            let [a, b, c, d] = pix.adj(x, y);

            let x2 = x * 2;
            let y2 = y * 2;
            assert_eq!(out[idx_xy(x2 + 1, y2 + 1, out_w)], [0.; CHANNELS]);
            assert_eq!(out[idx_xy(x2, y2, out_w)], [0.; CHANNELS]);

            // NOTE: flipped y here
            out[idx_xy(x2, y2 + 1, out_w)] = rule!(a if a = c, !c = d, !a = b else og);
            out[idx_xy(x2 + 1, y2 + 1, out_w)] = rule!(b if a = b, !a = c, !b=d else og);
            out[idx_xy(x2, y2, out_w)] = rule!(c if d = c, !d = b, !c=a else og);
            out[idx_xy(x2 + 1, y2, out_w)] = rule!(d if b = d, !b = a, !d = c else og);
        }
    }
}

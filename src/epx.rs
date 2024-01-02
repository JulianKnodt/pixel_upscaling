use crate::array_access::{idx_xy, Slice2D};

fn similar<const N: usize>(a: Option<&[f32; N]>, b: Option<&[f32; N]>) -> bool {
    return a == b;
    /*
    let (a, b) = match (a, b) {
        (None, None) => return true,
        (None, Some(_)) | (Some(_), None) => return false,
        (Some(a), Some(b)) => (a, b),
    };
    let total_diff = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f32>();
    total_diff < 1e-3
    */
}

#[inline]
pub fn epx<const CHANNELS: usize>(
    data: &[[f32; CHANNELS]],
    width: usize,
    height: usize,
) -> Vec<[f32; CHANNELS]> {
    let pix = Slice2D::new(data, width, height);

    let out_w = width * 2;
    let out_h = height * 2;
    let mut out = vec![[0.; CHANNELS]; out_w * out_h];

    macro_rules! rule {
      ($v: ident if $($e0: ident = $e1: ident),* , $(!$n0: ident = $n1: ident),* else $o: ident) => {
        if $(similar($e0,$e1) &&)* $(!similar($n0, $n1) &&)* true && let Some(v) = $v {
          *v
        } else {
          *$o
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
    out
}

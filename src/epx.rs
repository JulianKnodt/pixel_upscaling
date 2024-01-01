use crate::array_access::{idx_xy, Slice2D};

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
        if $($e0 == $e1 &&)* $($n0 != $n1 &&)* true && let Some(v) = $v {
          *v
        } else {
          *$o
        }
      }
    }

    for y in 0..height {
        for x in 0..width {
            let og = pix.get(x, y);
            let [a, b, d, c] = pix.adj(x, y);

            out[idx_xy(x * 2, y * 2, out_w)] = rule!(a if a = c, !c = d, !a = b else og);
            out[idx_xy(x * 2 + 1, y * 2, out_w)] = rule!(b if a = b, !a = c, !b=d else og);
            out[idx_xy(x * 2, y * 2 + 1, out_w)] = rule!(c if c = d, !d = b, !c=a else og);
            out[idx_xy(x * 2 + 1, y * 2 + 1, out_w)] = rule!(d if b = d, !b = a, !d = c else og);
        }
    }
    out
}

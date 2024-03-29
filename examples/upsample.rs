use clap::Parser;

use hq2x::epx::epx;
use hq2x::SimilarityKind;

pub fn main() {
    let args = Args::parse();
    let img = image::open(args.img).unwrap().into_rgb32f();
    let w = img.width() as usize;
    let h = img.height() as usize;
    let data = img
        .enumerate_pixels()
        .map(|(_, _, c)| c.0)
        .collect::<Vec<_>>();
    assert_eq!(data.len(), w * h);
    let sim = if args.fuzzy {
        SimilarityKind::Fuzzy
    } else {
        SimilarityKind::Bool
    };
    let mut out_img = vec![];
    epx::<3>(&data, w, h, &mut out_img, sim);

    let out_w = w as u32 * 2;
    let out_img = image::ImageBuffer::from_fn(out_w, h as u32 * 2, |x, y| {
        let data = out_img[(x + y * out_w) as usize];
        image::Rgb(data.map(|v| (v * 255.) as u8))
    });

    out_img
        .save(args.out)
        .unwrap_or_else(|e| panic!("Could not save image: {e}"));
}

#[derive(Parser)]
struct Args {
    #[arg(long, short, value_parser, required = true)]
    img: String,

    #[arg(long, short, value_parser, required = true)]
    out: String,

    #[arg(long, short, value_parser)]
    fuzzy: bool,
}

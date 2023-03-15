use lanime_curves::{point::Point, Curve};

fn main() {
    #[rustfmt::skip]
    let curve = &[
        Point { x: -1.0, y: 0.0 },
        Point { x: -1.2, y: 1.1 },
        Point { x: 0.7, y: 1.0 },
        Point { x: 1.0, y: 0.0 },
    ];

    let mut t = 0.;
    while t <= 1.0 {
        println!("{:?}", Curve::interpolate(&curve, t));
        t += 0.1;
    }
    //pollster::block_on(lanime_renderer::run());
}

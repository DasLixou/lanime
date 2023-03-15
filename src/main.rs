use lanime_curves::{point::Point, quadratic::QuadraticCurve, Curve};

fn main() {
    let linear = QuadraticCurve {
        start: Point { x: 0., y: 2. },
        control: Point::ZERO,
        end: Point { x: 2., y: 0. },
    };

    #[rustfmt::skip]
    let linear = &[
        Point { x: 0.0, y: 2.0 },
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
    ];

    let mut t = 0.;
    while t <= 1.0 {
        println!("{:?}", Curve::interpolate(&linear, t));
        t += 0.1;
    }
    //pollster::block_on(lanime_renderer::run());
}

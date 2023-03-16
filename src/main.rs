use lanime_animation::AnimationCurve;

fn main() {
    let curve = AnimationCurve::ease_in_out();
    println!("x: 0.4, y: {}", curve.curve_y(0.4));
    pollster::block_on(lanime_renderer::run());
}

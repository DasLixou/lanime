use lanime::prelude::{nodes::Text, AnimationCurve, Res::*, Scene, SceneDescriptor, Transform};

fn main() {
    let curve = AnimationCurve::ease_in_out();
    println!("x: 0.4, y: {}", curve.curve_y(0.4));

    let _scene = example();

    pollster::block_on(lanime_renderer::run());
}

fn example() -> Scene {
    let mut scene = Scene::builder();

    scene.node(Text {
        text: Value("Hello, world!"),
        transform: Value(Transform {
            x: Value(12.),
            ..Transform::DEFAULT
        }),
    });

    scene.build(&SceneDescriptor {
        label: Some("Example"),
    })
}

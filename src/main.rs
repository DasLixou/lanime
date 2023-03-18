use lanime::prelude::{nodes::Text, AnimationCurve, Res::*, Scene, SceneDescriptor, Transform};
use lanime_nodes::Render;

fn main() {
    let curve = AnimationCurve::ease_in_out();
    println!("x: 0.4, y: {}", curve.curve_y(0.4));

    let _scene = example();

    pollster::block_on(lanime_renderer::run());
}

fn example() -> Scene {
    let mut scene = Scene::builder();

    let text = scene.node(Text {
        text: Value("Hello, world!"),
        transform: Value(Transform {
            x: Value(12.),
            ..Transform::DEFAULT
        }),
    });

    scene.node(Render::new(&[&text])); // this can later be used to do post processing effects

    scene.build(&SceneDescriptor {
        label: Some("Example"),
    })
}

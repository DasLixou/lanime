use lanime::prelude::{nodes::Text, AnimationCurve, Scene, SceneDescriptor, Transform};
use lanime_core::{bindable_field::Lens, Resource};
use lanime_nodes::Render;

fn main() {
    let curve = AnimationCurve::ease_in_out();
    println!("x: 0.4, y: {}", curve.curve_y(0.4));

    let _scene = example();

    pollster::block_on(lanime_renderer::run());
}

fn example() -> Scene {
    let mut scene = Scene::new(&SceneDescriptor {
        label: Some("Example"),
    });

    let text = scene.node(Text {
        text: "Hello, world!",
        transform: Transform::DEFAULT,
    });

    let text_y = scene.node(Resource(0.));

    scene.bind(text_y, Text::transform.then(Transform::y), &text);

    scene.node(Render::new(&[&text])); // this can later be used to do post processing effects

    scene
}

use lanime::prelude::{
    nodes::{Render, Text},
    AnimationCurve, IntoNodeIdx, Lens, NodeRef, Scene, SceneDescriptor, Transform,
};
use lanime_core::{res::ClosureResource, SceneContext};

fn main() {
    let curve = AnimationCurve::ease_in_out();
    println!("x: 0.4, y: {}", curve.curve_y(0.4));

    let (text, mut scene) = example();
    for frame in 0..60 {
        scene.update(text.idx(), SceneContext { frame });
        scene.debug::<Text>(text.idx());
    }

    pollster::block_on(lanime_renderer::run());
}

fn example() -> (NodeRef<Text<'static>>, Scene) {
    let mut scene = Scene::new(&SceneDescriptor {
        label: Some("Example"),
    });

    let text = scene.node(Text {
        text: "Hello, world!",
        transform: Transform::DEFAULT,
    });

    let text_y = scene.node(ClosureResource(|cx| (cx.frame as f32 / 23.0).sin()));

    scene.bind(&text_y, Text::transform.then(Transform::y), &text);

    scene.node(Render::new(&[&text])); // this node can later be used to do post processing effects

    (text, scene)
}

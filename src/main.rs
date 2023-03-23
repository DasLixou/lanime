use lanime::prelude::{
    nodes::{Render, Text},
    res::ClosureResource,
    AnimationCurve, Lens, NodeRef, Scene, SceneDescriptor, Transform,
};
use lanime_core::Position;

fn main() {
    let curve = AnimationCurve::ease_in_out();
    println!("x: 0.4, y: {}", curve.curve_y(0.4));

    let (mut scene, text) = example();

    let scene_ptr = &mut scene as *mut Scene;

    unsafe {
        pollster::block_on(lanime_renderer::run(&mut *scene_ptr, text));
    }
}

fn example() -> (Scene, NodeRef<Text<'static>>) {
    let mut scene = Scene::new(&SceneDescriptor {
        label: Some("Example"),
    });

    let text = scene.node(Text {
        text: "L A N I M E",
        ..Text::DEFAULT
    });

    let text_y = scene.node(ClosureResource(|cx| (cx.frame as f32 / 60.0).sin()));

    scene.bind(
        &text_y,
        Text::transform.then(Transform::position).then(Position::y),
        &text,
    );

    scene.node(Render::new(&[&text])); // this node can later be used to do post processing effects

    (scene, text)
}

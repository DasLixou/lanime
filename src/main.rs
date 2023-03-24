use lanime::prelude::{
    layout::{Transform, Vector3Fields},
    nodes::Text,
    res::ClosureResource,
    AnimationCurve, Lens, NodeRef, Scene, SceneDescriptor,
};

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

    let text_y = scene.node(ClosureResource(|cx| (cx.frame as f32 / 120.0).sin() / 2.));

    scene.bind(
        &text_y,
        Text::transform
            .then(Transform::position)
            .then(Vector3Fields::y),
        &text,
    );

    (scene, text)
}

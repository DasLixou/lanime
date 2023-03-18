use lanime::prelude::{
    nodes::{Rectangle, Resource},
    AnimationCurve, IntoRes, Res, Scene, SceneDescriptor,
};

fn main() {
    let curve = AnimationCurve::ease_in_out();
    println!("x: 0.4, y: {}", curve.curve_y(0.4));

    let mut scene = Scene::create(&SceneDescriptor {
        label: Some("Example"),
    });

    let height = scene.node(Resource(720.));

    let x = Rectangle {
        width: Res::Owned(1280.),
        height: height.res(),
    };

    let _rectangle = scene.node(x);

    pollster::block_on(lanime_renderer::run());
}

use lanime_bindfields::BindFields;
use lanime_core::{layout::Transform, Node, Vector2, Vector3};
use lanime_renderer::{context::RenderContext, NodeRender};

use wgpu_text::{
    font::FontRef,
    section::{Layout, Section},
    TextBrush,
};

#[derive(BindFields)]
pub struct Text<'s> {
    pub text: &'s str,
    pub transform: Transform,
    pub internal: Option<(TextBrush<FontRef<'s>>, Section<'s>)>,
}

impl Text<'_> {
    pub const DEFAULT: Self = Self {
        text: "",
        transform: Transform::DEFAULT,
        internal: None,
    };
}

impl Node for Text<'static> {}

impl NodeRender for Text<'static> {
    fn init(&mut self, cx: &RenderContext) {
        let brush = wgpu_text::BrushBuilder::using_font_bytes(include_bytes!("DejaVuSans.ttf"))
            .unwrap()
            .build(cx.device, cx.config);

        let section = Section::default()
            .add_text(
                wgpu_text::section::Text::new(self.text)
                    .with_scale(25.)
                    .with_color([1., 1., 1., 1.]),
            )
            .with_layout(
                Layout::default()
                    .h_align(wgpu_text::section::HorizontalAlign::Center)
                    .v_align(wgpu_text::section::VerticalAlign::Center),
            )
            .with_bounds((
                self.transform.size.x.min(cx.config.width as f32),
                self.transform.size.y.min(cx.config.height as f32),
            ));

        self.internal = Some((brush, section));
    }

    fn render(&mut self, cx: &RenderContext) {
        let (brush, section) = self.internal.as_mut().unwrap();

        let screen_pos = {
            let screen = Vector2::new(cx.config.width as f32, cx.config.height as f32);

            let top_left = (self.transform.position.truncate() + Vector2::new(1., 1.)) / 2.;

            Vector2::new(top_left.x * screen.x, top_left.y * screen.y)
        };

        section.screen_position.0 = screen_pos.x;
        section.screen_position.1 = screen_pos.y;

        brush.queue(section as &Section);
        let text_buffer = brush.draw(cx.device, cx.view.unwrap(), cx.queue);
        cx.queue.submit([text_buffer]);
    }
}

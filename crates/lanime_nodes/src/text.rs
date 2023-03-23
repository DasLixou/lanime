use lanime_bindfields::BindFields;
use lanime_core::{Node, Transform};
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
            .with_layout(Layout::default().h_align(wgpu_text::section::HorizontalAlign::Center))
            .with_bounds((cx.config.width as f32, cx.config.height as f32))
            .with_screen_position((cx.config.width as f32 * 0.5, 0.));

        self.internal = Some((brush, section));
    }

    fn render(&mut self, cx: &RenderContext) {
        let (brush, section) = self.internal.as_mut().unwrap();

        section.screen_position.1 = self.transform.position.x * 20. + 50.;

        brush.queue(section as &Section);
        let text_buffer = brush.draw(cx.device, cx.view.unwrap(), cx.queue);
        cx.queue.submit([text_buffer]);
    }
}

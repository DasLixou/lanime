pub struct RenderContext<'c> {
    pub surface_format: &'c wgpu::TextureFormat,
    pub device: &'c wgpu::Device,
    pub queue: &'c wgpu::Queue,
    pub config: &'c wgpu::SurfaceConfiguration,
    pub view: Option<&'c wgpu::TextureView>,
}

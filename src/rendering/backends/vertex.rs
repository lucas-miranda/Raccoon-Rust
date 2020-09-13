

pub trait VertexPosition<D> {
    fn position(&self) -> &D;
}

pub trait VertexUV<D> {
    fn uv(&self) -> &D;
}

pub struct StandardVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2]
}

impl VertexPosition<[f32; 2]> for StandardVertex {
    fn position(&self) -> &[f32; 2] {
        &self.position
    }
}

impl VertexUV<[f32; 2]> for StandardVertex {
    fn uv(&self) -> &[f32; 2] {
        &self.uv
    }
}

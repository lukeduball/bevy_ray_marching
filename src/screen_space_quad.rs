use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}};

#[derive(Debug, Copy, Clone)]
pub struct ScreenSpaceQuad {
    //Scale of the quad in screen space with 1,1 taking the whole screen
    pub scale: Vec2,
}

impl Default for ScreenSpaceQuad {
    fn default() -> Self {
        Self::new(Vec2::ONE)
    }
}

impl ScreenSpaceQuad {
    fn new(scale: Vec2) -> Self {
        Self { scale }
    }
}

//Creating the custom vertex data for our ScreenSpaceQuad
//Note: the normal and uv attribute had to be included for this to work. Seems to be some Bevy limitation
impl From<ScreenSpaceQuad> for Mesh {
    fn from(screen_space_quad: ScreenSpaceQuad) -> Self {
        let vertices = vec![[-1.0*screen_space_quad.scale.x, -1.0*screen_space_quad.scale.y, 0.0],
                            [-1.0*screen_space_quad.scale.x,  1.0*screen_space_quad.scale.y, 0.0],
                            [ 1.0*screen_space_quad.scale.x, -1.0*screen_space_quad.scale.y, 0.0],
                            [ 1.0*screen_space_quad.scale.x,  1.0*screen_space_quad.scale.y, 0.0]];

        let indices = Indices::U32(vec![0, 2, 1, 2, 3, 1]);

        let normals = vec![[0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0]];

        let uvs = vec![[0.0, 0.0],
                        [0.0, 1.0],
                        [1.0, 0.0],
                        [1.0,1.0]];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
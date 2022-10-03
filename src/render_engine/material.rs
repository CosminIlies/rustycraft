use crate::Shader;

pub struct Material{
    pub shader:Shader,
    pub diff_tex:glium::texture::SrgbTexture2d
}

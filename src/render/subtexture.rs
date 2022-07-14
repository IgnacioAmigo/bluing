use super::texture::{Texture};
use glm;

pub struct Subtexture<'a> {
    texture:  &'a Texture,
    normalized_rect: glm::Vec4, // todo: should this struct be renamed/aliased into Rect? we want the data in the format of x, y, width, height and not x1, y1, x2, y2
}

impl<'a> Subtexture<'a> {
    pub fn get_normalized_rect(&self) -> glm::Vec4 {
        self.normalized_rect
    }

    pub fn texture(&self) -> &Texture{ self.texture }

    pub fn from_texture(texture: &'a Texture, start_coords_normalized:glm::Vec2, dimensions_normalized:glm::Vec2) -> Subtexture {
        Subtexture {texture, normalized_rect:glm::vec4(start_coords_normalized.x, start_coords_normalized.y, dimensions_normalized.x,dimensions_normalized.y)}
    }

    pub fn from_tiles(texture: &'a Texture, tile_index_x: u16,tile_index_y : u16, tile_dimensions :glm::Vec2) -> Subtexture {
        let start_coords_normalized = glm::vec2(tile_index_x as f32 * tile_dimensions.x / texture.width_f(), tile_index_y as f32 * tile_dimensions.y / texture.height_f());
        let dimensions_normalized = glm::vec2(tile_dimensions.x / texture.width_f(), tile_dimensions.y / texture.height_f());

        Subtexture {texture, normalized_rect: glm::vec4(start_coords_normalized.x, start_coords_normalized.y, dimensions_normalized.x,dimensions_normalized.y)}
    }
}
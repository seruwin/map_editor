use graphics::*;
use crate::resource::*;
use crate::collection::TEXTURE_SIZE;

pub struct MapView {
    pub maps: Vec<Map>,
    pub link_map_selection: Vec<Image>,
    pub selection_preview: Image,
    preview_pos: Vec2,
    preview_size: Vec2,
}

impl MapView {
    pub fn new(resource: &TextureAllocation, renderer: &mut GpuRenderer) -> Self {
        let mut maps = Vec::with_capacity(9);
        let mut link_map_selection = Vec::with_capacity(8);
        
        // Create 9 maps for our view of the main map and the surrounding maps
        for count in 0..9 {
            let mut map = Map::new(renderer, TEXTURE_SIZE);

            // Set default position of each view
            // Note: Index '0' is the main view on the center
            // while the other view are for surrounding maps
            match count {
                1 => { map.pos = Vec2::new(215.0, 719.0); }, // Top Left
                2 => { map.pos = Vec2::new(257.0, 719.0); }, // Top
                3 => { map.pos = Vec2::new(899.0, 719.0); }, // Top Right
                4 => { map.pos = Vec2::new(215.0, 77.0); }, // Left
                5 => { map.pos = Vec2::new(899.0, 77.0); }, // Right
                6 => { map.pos = Vec2::new(215.0, 35.0); }, // Bottom Left
                7 => { map.pos = Vec2::new(257.0, 35.0); }, // Bottom
                8 => { map.pos = Vec2::new(899.0, 35.0); }, // Bottom Right
                _ => { map.pos = Vec2::new(257.0, 77.0); }, // Center / Main
            }

            map.can_render = true;
            maps.push(map);
        };

        // We add the link selection overlay above the link map as a selecting effect
        for count in 0..8 {
            let mut image = Image::new(Some(resource.white), renderer, 1);

            // We set the link selection image at the same position as the linked map
            // We add +1 on the count as the linked map started on index 1 instead of 0
            image.pos = Vec3::new(maps[count + 1].pos.x, maps[count + 1].pos.y, 4.0);
            match count {
                0 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 2.0, TEXTURE_SIZE as f32 * 2.0);}, // Top Left
                1 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 32.0, TEXTURE_SIZE as f32 * 2.0);}, // Top
                2 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 2.0, TEXTURE_SIZE as f32 * 2.0);}, // Top Right
                3 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 2.0, TEXTURE_SIZE as f32 * 32.0);}, // Left
                4 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 2.0, TEXTURE_SIZE as f32 * 32.0);}, // Right
                5 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 2.0, TEXTURE_SIZE as f32 * 2.0);}, // Bottom Left
                6 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 32.0, TEXTURE_SIZE as f32 * 2.0);}, // Bottom
                7 => { image.hw = Vec2::new(TEXTURE_SIZE as f32 * 2.0, TEXTURE_SIZE as f32 * 2.0); }, // Bottom Right
                _ => {},
            }
            image.uv = Vec4::new(2.0, 2.0, 17.0, 17.0);
            image.color = Color::rgba(0, 0, 0, 130);
            
            link_map_selection.push(image);
        }

        let mut selection_preview = Image::new(Some(resource.white), renderer, 1);

        // Setup the selection preview image settings
        selection_preview.pos = Vec3::new(maps[0].pos.x, maps[0].pos.y, 4.0);
        selection_preview.hw = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
        selection_preview.uv = Vec4::new(2.0, 2.0, 17.0, 17.0);
        selection_preview.color = Color::rgba(0, 0, 150, 150);

        Self {
            maps,
            link_map_selection,
            selection_preview,
            preview_pos: Vec2::new(0.0, 0.0),
            preview_size: Vec2::new(1.0, 1.0),
        }
    }

    // This function create an effect when we are hovering on the linked map
    pub fn hover_linked_selection(&mut self, pos: Vec2) {
        for selection in &mut self.link_map_selection {
            let is_within_pos =
                pos.x >= selection.pos.x
                    && pos.x <= selection.pos.x + selection.hw.x
                    && pos.y >= selection.pos.y
                    && pos.y <= selection.pos.y + selection.hw.y as f32;
    
            if is_within_pos {
                if selection.color != Color::rgba(0, 0, 0, 0) {
                    selection.color = Color::rgba(0, 0, 0, 0);
                    selection.changed = true;
                }
            } else {
                if selection.color != Color::rgba(0, 0, 0, 130) {
                    selection.color = Color::rgba(0, 0, 0, 130);
                    selection.changed = true;
                }
            }
        }
    }

    pub fn set_tile_group(&mut self, set_pos: Vec2, layer: u32, tileset: &Map, start_pos: Vec2, selection_size: Vec2) {
        for x in 0..selection_size.x as u32 {
            for y in 0..selection_size.y as u32 {
                // We load the tile data from the tileset
                let tiledata = tileset.get_tile((start_pos.x as u32 + x, start_pos.y as u32 + y, layer));

                // Make sure we only add tile that are not empty
                if tiledata.texture_id > 0 {
                    // Make sure we wont set map outside the map size limit
                    if (set_pos.x as u32 + x) < 32 && (set_pos.y as u32 + y) < 32 {
                        self.maps[0].set_tile((set_pos.x as u32 + x, set_pos.y as u32 + y, layer), tiledata);
                    }
                }
            }
        }
    }

    pub fn hover_selection_preview(&mut self, set_pos: Vec2) {
        if self.preview_pos != set_pos && set_pos.x < 32.0 && set_pos.y < 32.0 {
            self.preview_pos = set_pos;
            self.selection_preview.pos = Vec3::new(self.maps[0].pos.x + set_pos.x * TEXTURE_SIZE as f32, 
                                                    self.maps[0].pos.y + set_pos.y * TEXTURE_SIZE as f32, 
                                                    4.0);
            self.adjust_selection_preview();
            self.selection_preview.changed = true;
        }
    }
    
    pub fn change_selection_preview_size(&mut self, size: Vec2) {
        self.preview_size = size;
        self.adjust_selection_preview();
        self.selection_preview.changed = true;
    }

    // This function ensure that the selection preview does not show outside the map boundary
    fn adjust_selection_preview(&mut self) {
        let max_size = Vec2::new(32.0, 32.0);
    
        let clamped_x = (self.preview_pos.x + self.preview_size.x).min(max_size.x);
        let clamped_y = (self.preview_pos.y + self.preview_size.y).min(max_size.y);

        let new_size = Vec2::new(clamped_x - self.preview_pos.x, clamped_y - self.preview_pos.y);

        self.selection_preview.hw = Vec2::new(new_size.x * TEXTURE_SIZE as f32, new_size.y * TEXTURE_SIZE as f32);
    }
}
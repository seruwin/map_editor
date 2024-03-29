use graphics::*;
use crate::resource::*;
use crate::collection::TEXTURE_SIZE;

pub const MAX_TILE_X: u32 = 10;
pub const MAX_TILE_Y: u32 = 20;

pub struct Tileset {
    pub map: Map,
    pub selected_tile: usize,
    pub selection: Image,
    pub select_start: Vec2,
    pub select_size: Vec2,
}

impl Tileset {
    pub fn new(resource: &TextureAllocation, renderer: &mut GpuRenderer) -> Self {
        let mut tilesheet = Tileset {
            map: Map::new(renderer, TEXTURE_SIZE),
            selected_tile: 0,
            selection: Image::new(Some(resource.white.allocation), renderer, 1),
            select_start: Vec2::new(0.0, (MAX_TILE_Y - 1) as f32),
            select_size: Vec2::new(1.0, 1.0),
        };

        // Loop throughout all texture and place them on the map based on their texture location
        for tiledata in &resource.tilesheet[tilesheet.selected_tile].tile.tiles {
            let (id, x, y) = (tiledata.id, tiledata.x / TEXTURE_SIZE, (MAX_TILE_Y - (tiledata.y / TEXTURE_SIZE) - 1));
            // We make sure that we only set those that are not empty tile
            if id > 0 {
                tilesheet.map.set_tile(
                    (x, y, 0),
                    TileData {
                        texture_id: id,
                        texture_layer: 0,
                        color: Color::rgba(255, 255, 255, 255),
                    },
                );
            }
        }
        // Adjust tileset position on interface
        tilesheet.map.pos = Vec2::new(11.0, 369.0);
        tilesheet.map.can_render = true;

        // Setup tile selection image settings
        // We set the selected tile at the very first tile
        tilesheet.selection.pos = Vec3::new(tilesheet.map.pos.x, tilesheet.map.pos.y + ((MAX_TILE_Y - 1) * TEXTURE_SIZE) as f32, 9.0);
        tilesheet.selection.hw = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
        tilesheet.selection.uv = Vec4::new(2.0, 2.0, 17.0, 17.0);
        tilesheet.selection.color = Color::rgba(80, 0, 0, 130);

        tilesheet
    }

    pub fn set_selection(&mut self, start: Vec2, end: Vec2) -> Vec2 {
        // Let's arrange the start pos and end pos to make sure start pos consist the smallest value
        let start_pos = Vec2::new(
            if start.x > end.x { end.x } else { start.x },
            if start.y > end.y { end.y } else { start.y },
        );
        let end_pos = Vec2::new(
            if start.x > end.x { start.x } else { end.x },
            if start.y > end.y { start.y } else { end.y },
        );

        // Set data that will be use when placing tile on map
        self.select_start = start_pos;
        self.select_size = (end_pos - start_pos) + 1.0;

        // Adjust selection position and size
        self.selection.pos = Vec3::new(self.map.pos.x + (start_pos.x * TEXTURE_SIZE as f32), 
                                        self.map.pos.y + (start_pos.y * TEXTURE_SIZE as f32), 4.0);
        self.selection.hw = self.select_size * TEXTURE_SIZE as f32;
        self.selection.changed = true;

        self.select_size
    }

    pub fn change_tileset(&mut self, resource: &TextureAllocation, tileset_index: usize) {
        if self.selected_tile == tileset_index {
            return;
        }
        self.selected_tile = tileset_index;
        
        // Clear Tileset
        (0..MAX_TILE_X).for_each(|x| {
            (0..MAX_TILE_Y).for_each(|y| {
                self.map.set_tile(
                    (x, y, 0),
                    TileData::default(),
                );
            });
        });

        // Loop throughout all texture and place them on the map based on their texture location
        for tiledata in &resource.tilesheet[tileset_index].tile.tiles {
            let (id, x, y) = (tiledata.id, tiledata.x / TEXTURE_SIZE, (MAX_TILE_Y - (tiledata.y / TEXTURE_SIZE) - 1));
            // We make sure that we only set those that are not empty tile
            if id > 0 {
                self.map.set_tile(
                    (x, y, 0),
                    TileData {
                        texture_id: id,
                        texture_layer: 0,
                        color: Color::rgba(255, 255, 255, 255),
                    },
                );
            }
        }
    }
}
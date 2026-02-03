use std::collections::HashMap;
use crate::math::Vec2;

#[derive(Debug, Clone)]
pub struct TileSet {
    first_gid: u32,
    name: String,
    tile_size: Vec2,
    spacing: u32,
    margin: u32,
    image: String,
    image_size: Vec2,
    tile_count: u32,
    columns: u32,
}

impl TileSet {
    pub fn new() -> TileSet {
        TileSet {
            first_gid: 1,
            name: String::new(),
            tile_size: Vec2::new(64.0, 64.0),
            spacing: 0,
            margin: 0,
            image: String::new(),
            image_size: Vec2::ZERO,
            tile_count: 0,
            columns: 0,
        }
    }

    pub fn get_first_gid(&self) -> u32 {
        self.first_gid
    }

    pub fn set_first_gid(&mut self, first_gid: u32) {
        self.first_gid = first_gid;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_tile_size(&self) -> Vec2 {
        self.tile_size
    }

    pub fn set_tile_size(&mut self, size: Vec2) {
        self.tile_size = size;
    }

    pub fn get_image(&self) -> &str {
        &self.image
    }

    pub fn set_image(&mut self, image: &str) {
        self.image = image.to_string();
    }

    pub fn rect_for_gid(&self, gid: u32) -> Rect {
        let gid = gid - self.first_gid;
        let row = gid / self.columns;
        let col = gid % self.columns;
        let x = self.margin as f32 + col as f32 * (self.tile_size.x + self.spacing as f32);
        let y = self.margin as f32 + row as f32 * (self.tile_size.y + self.spacing as f32);
        Rect::new(x, y, self.tile_size.x, self.tile_size.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect { x, y, width, height }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }
}

#[derive(Debug, Clone)]
pub struct TileMapInfo {
    filename: String,
    pub map_size: Vec2,
    pub tile_size: Vec2,
    orientation: MapOrientation,
    layers: Vec<LayerInfo>,
    tile_sets: Vec<TileSet>,
    properties: HashMap<String, String>,
    object_groups: Vec<ObjectGroup>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapOrientation {
    ORTHOGONAL,
    ISOMETRIC,
    STAGGERED,
    HEXAGONAL,
}

#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub name: String,
    pub size: Vec2,
    opacity: f32,
    visible: bool,
    tiles: Vec<u32>,
    properties: HashMap<String, String>,
}

impl LayerInfo {
    pub fn new(name: &str, width: u32, height: u32) -> LayerInfo {
        LayerInfo {
            name: name.to_string(),
            size: Vec2::new(width as f32, height as f32),
            opacity: 1.0,
            visible: true,
            tiles: vec![0; (width * height) as usize],
            properties: HashMap::new(),
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> u32 {
        let index = (y * self.size.x as u32 + x) as usize;
        self.tiles.get(index).cloned().unwrap_or(0)
    }

    pub fn set_tile(&mut self, x: u32, y: u32, gid: u32) {
        let index = (y * self.size.x as u32 + x) as usize;
        if index < self.tiles.len() {
            self.tiles[index] = gid;
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectGroup {
    name: String,
    color: String,
    objects: Vec<TileMapObject>,
}

#[derive(Debug, Clone)]
pub struct TileMapObject {
    id: i32,
    name: String,
    object_type: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rotation: f32,
    visible: bool,
    properties: HashMap<String, String>,
}

impl TileMapObject {
    pub fn new() -> TileMapObject {
        TileMapObject {
            id: 0,
            name: String::new(),
            object_type: String::new(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            rotation: 0.0,
            visible: true,
            properties: HashMap::new(),
        }
    }
}

impl TileMapInfo {
    pub fn new() -> TileMapInfo {
        TileMapInfo {
            filename: String::new(),
            map_size: Vec2::ZERO,
            tile_size: Vec2::ZERO,
            orientation: MapOrientation::ORTHOGONAL,
            layers: Vec::new(),
            tile_sets: Vec::new(),
            properties: HashMap::new(),
            object_groups: Vec::new(),
        }
    }

    pub fn create_with_file(file: &str) -> Option<TileMapInfo> {
        Some(TileMapInfo::new())
    }

    pub fn get_tile_size(&self) -> Vec2 {
        self.tile_size
    }

    pub fn get_map_size(&self) -> Vec2 {
        self.map_size
    }

    pub fn add_layer(&mut self, layer: LayerInfo) {
        self.layers.push(layer);
    }

    pub fn get_layers(&self) -> &Vec<LayerInfo> {
        &self.layers
    }

    pub fn add_tileset(&mut self, tileset: TileSet) {
        self.tile_sets.push(tileset);
    }

    pub fn get_tilesets(&self) -> &Vec<TileSet> {
        &self.tile_sets
    }

    pub fn get_layer_by_name(&self, name: &str) -> Option<&LayerInfo> {
        self.layers.iter().find(|l| l.name == name)
    }

    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tileset_new() {
        let tileset = TileSet::new();
        assert_eq!(tileset.get_first_gid(), 1);
        assert_eq!(tileset.get_tile_size(), Vec2::new(64.0, 64.0));
    }

    #[test]
    fn test_tileset_name() {
        let mut tileset = TileSet::new();
        tileset.set_name("Terrain");
        assert_eq!(tileset.get_name(), "Terrain");
    }

    #[test]
    fn test_tileset_first_gid() {
        let mut tileset = TileSet::new();
        tileset.set_first_gid(10);
        assert_eq!(tileset.get_first_gid(), 10);
    }

    #[test]
    fn test_rect_new() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.get_x(), 10.0);
        assert_eq!(rect.get_y(), 20.0);
        assert_eq!(rect.get_width(), 100.0);
        assert_eq!(rect.get_height(), 200.0);
    }

    #[test]
    fn test_layer_info_new() {
        let layer = LayerInfo::new("Background", 5, 5);
        assert_eq!(layer.name, "Background");
        assert_eq!(layer.size, Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_layer_get_set_tile() {
        let mut layer = LayerInfo::new("Test", 3, 3);
        layer.set_tile(1, 1, 42);
        assert_eq!(layer.get_tile(1, 1), 42);
    }

    #[test]
    fn test_tilemap_info_new() {
        let info = TileMapInfo::new();
        assert_eq!(info.map_size, Vec2::ZERO);
        assert_eq!(info.tile_size, Vec2::ZERO);
    }

    #[test]
    fn test_tilemap_info_add_layer() {
        let mut info = TileMapInfo::new();
        let layer = LayerInfo::new("Layer1", 10, 10);
        info.add_layer(layer);
        assert_eq!(info.get_layers().len(), 1);
    }

    #[test]
    fn test_tilemap_info_add_tileset() {
        let mut info = TileMapInfo::new();
        let tileset = TileSet::new();
        info.add_tileset(tileset);
        assert_eq!(info.get_tilesets().len(), 1);
    }

    #[test]
    fn test_get_layer_by_name() {
        let mut info = TileMapInfo::new();
        let layer = LayerInfo::new("Ground", 10, 10);
        info.add_layer(layer);
        let found = info.get_layer_by_name("Ground");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Ground");
    }
}

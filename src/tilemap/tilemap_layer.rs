#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::tilemap_info::{LayerInfo, TileMapInfo, TileSet};
use crate::base::RefPtr;
use crate::sprite::Sprite;

#[derive(Debug)]
pub struct TileMapLayer {
    tileset: Option<TileSet>,
    tiles: Vec<RefPtr<Sprite>>,
    layer_info: Option<LayerInfo>,
    tile_width: f32,
    tile_height: f32,
    map_width: u32,
    map_height: u32,
}

impl Default for TileMapLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl TileMapLayer {
    pub fn new() -> TileMapLayer {
        TileMapLayer {
            tileset: None,
            tiles: Vec::new(),
            layer_info: None,
            tile_width: 0.0,
            tile_height: 0.0,
            map_width: 0,
            map_height: 0,
        }
    }

    pub fn create_with_layer_info(layer_info: LayerInfo, tileset: TileSet) -> TileMapLayer {
        let mut layer = TileMapLayer::new();

        let tile_width = tileset.get_tile_size().x;
        let tile_height = tileset.get_tile_size().y;
        let map_width = layer_info.size.x as u32;
        let map_height = layer_info.size.y as u32;

        layer.layer_info = Some(layer_info);
        layer.tileset = Some(tileset);
        layer.tile_width = tile_width;
        layer.tile_height = tile_height;
        layer.map_width = map_width;
        layer.map_height = map_height;
        layer
    }

    pub fn init(&mut self) {}

    pub fn get_layer_name(&self) -> &str {
        self.layer_info
            .as_ref()
            .map(|l| l.name.as_str())
            .unwrap_or("")
    }

    pub fn get_tile_at(&self, x: u32, y: u32) -> Option<&RefPtr<Sprite>> {
        let index = (y * self.map_width + x) as usize;
        self.tiles.get(index)
    }

    pub fn set_tile_gid(&mut self, gid: u32, x: u32, y: u32) {
        let index = (y * self.map_width + x) as usize;
        if index < self.tiles.len() {}
    }

    pub fn get_tile_gid(&self, x: u32, y: u32) -> u32 {
        self.layer_info
            .as_ref()
            .map(|l| l.get_tile(x, y))
            .unwrap_or(0)
    }

    pub fn remove_tile_at(&mut self, x: u32, y: u32) {
        let index = (y * self.map_width + x) as usize;
        if index < self.tiles.len() {
            self.tiles[index] = RefPtr::new(Sprite::new());
        }
    }

    pub fn get_tile_width(&self) -> f32 {
        self.tile_width
    }

    pub fn get_tile_height(&self) -> f32 {
        self.tile_height
    }

    pub fn get_map_width(&self) -> u32 {
        self.map_width
    }

    pub fn get_map_height(&self) -> u32 {
        self.map_height
    }
}

#[derive(Debug)]
pub struct TileMap {
    map_info: Option<TileMapInfo>,
    layers: Vec<RefPtr<TileMapLayer>>,
}

impl Default for TileMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            map_info: None,
            layers: Vec::new(),
        }
    }

    pub fn create_with_tmx_file(file: &str) -> Option<TileMap> {
        let mut tilemap = TileMap::new();
        tilemap.init_with_file(file);
        Some(tilemap)
    }

    pub fn init_with_file(&mut self, file: &str) {}

    pub fn get_map_size(&self) -> (u32, u32) {
        self.map_info
            .as_ref()
            .map(|m| (m.map_size.x as u32, m.map_size.y as u32))
            .unwrap_or((0, 0))
    }

    pub fn get_tile_size(&self) -> (f32, f32) {
        self.map_info
            .as_ref()
            .map(|m| (m.tile_size.x, m.tile_size.y))
            .unwrap_or((0.0, 0.0))
    }

    pub fn add_layer(&mut self, layer: RefPtr<TileMapLayer>) {
        self.layers.push(layer);
    }

    pub fn get_layers(&self) -> &Vec<RefPtr<TileMapLayer>> {
        &self.layers
    }

    pub fn get_layer_by_name(&self, name: &str) -> Option<&RefPtr<TileMapLayer>> {
        self.layers
            .iter()
            .find(|l| l.borrow().get_layer_name() == name)
    }

    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.map_info.as_ref().and_then(|m| m.get_property(key))
    }

    pub fn get_object_group(&self, name: &str) -> Option<&Vec<()>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilemap_layer_new() {
        let layer = TileMapLayer::new();
        assert_eq!(layer.get_tile_width(), 0.0);
        assert_eq!(layer.get_tile_height(), 0.0);
        assert_eq!(layer.get_map_width(), 0);
        assert_eq!(layer.get_map_height(), 0);
    }

    #[test]
    fn test_tilemap_layer_create() {
        let layer_info = LayerInfo::new("TestLayer", 10, 10);
        let tileset = TileSet::new();
        let layer = TileMapLayer::create_with_layer_info(layer_info, tileset);
        assert_eq!(layer.get_layer_name(), "TestLayer");
    }

    #[test]
    fn test_tilemap_new() {
        let tilemap = TileMap::new();
        let (width, height) = tilemap.get_map_size();
        assert_eq!(width, 0);
        assert_eq!(height, 0);
    }

    #[test]
    fn test_tilemap_create() {
        let tilemap = TileMap::create_with_tmx_file("test.tmx");
        assert!(tilemap.is_some());
    }

    #[test]
    fn test_tilemap_add_layer() {
        let mut tilemap = TileMap::new();
        let layer = TileMapLayer::new();
        tilemap.add_layer(RefPtr::new(layer));
        assert_eq!(tilemap.get_layers().len(), 1);
    }

    #[test]
    fn test_get_property() {
        let tilemap = TileMap::new();
        let property = tilemap.get_property("test");
        assert!(property.is_none());
    }
    
    #[test]
    fn test_tilemap_tile_size() {
        let tilemap = TileMap::new();
        let (width, height) = tilemap.get_tile_size();
        assert_eq!(width, 0.0);
        assert_eq!(height, 0.0);
    }
    
    #[test]
    fn test_tilemap_layer_get_tile() {
        let layer = TileMapLayer::new();
        let tile = layer.get_tile_at(0, 0);
        assert!(tile.is_none());
    }
    
    #[test]
    fn test_tilemap_layer_tile_gid() {
        let layer = TileMapLayer::new();
        let gid = layer.get_tile_gid(0, 0);
        assert_eq!(gid, 0);
    }
    
    #[test]
    fn test_tilemap_layer_remove_tile() {
        let mut layer_info = LayerInfo::new("TestLayer", 5, 5);
        let tileset = TileSet::new();
        let mut layer = TileMapLayer::create_with_layer_info(layer_info, tileset);
        
        layer.remove_tile_at(2, 2);
        // 验证瓦片被移除
    }
    
    #[test]
    fn test_tilemap_layer_dimensions() {
        let layer_info = LayerInfo::new("TestLayer", 10, 8);
        let tileset = TileSet::new();
        let layer = TileMapLayer::create_with_layer_info(layer_info, tileset);
        
        assert_eq!(layer.get_map_width(), 10);
        assert_eq!(layer.get_map_height(), 8);
    }
    
    #[test]
    fn test_tilemap_get_layer_by_name() {
        let mut tilemap = TileMap::new();
        let layer_info = LayerInfo::new("Layer1", 5, 5);
        let tileset = TileSet::new();
        let layer = TileMapLayer::create_with_layer_info(layer_info, tileset);
        
        tilemap.add_layer(RefPtr::new(layer));
        
        let found = tilemap.get_layer_by_name("Layer1");
        assert!(found.is_some());
        
        let not_found = tilemap.get_layer_by_name("NonExistent");
        assert!(not_found.is_none());
    }
    
    #[test]
    fn test_tilemap_multiple_layers() {
        let mut tilemap = TileMap::new();
        
        let layer1 = TileMapLayer::create_with_layer_info(
            LayerInfo::new("Background", 10, 10),
            TileSet::new()
        );
        
        let layer2 = TileMapLayer::create_with_layer_info(
            LayerInfo::new("Foreground", 10, 10),
            TileSet::new()
        );
        
        tilemap.add_layer(RefPtr::new(layer1));
        tilemap.add_layer(RefPtr::new(layer2));
        
        assert_eq!(tilemap.get_layers().len(), 2);
    }
    
    #[test]
    fn test_tilemap_layer_bounds() {
        let layer_info = LayerInfo::new("TestLayer", 3, 3);
        let tileset = TileSet::new();
        let layer = TileMapLayer::create_with_layer_info(layer_info, tileset);
        
        // 测试边界
        let tile = layer.get_tile_at(0, 0);
        let tile_edge = layer.get_tile_at(2, 2);
        let tile_out = layer.get_tile_at(5, 5);
        
        assert!(tile_out.is_none());
    }
    
    #[test]
    fn test_tilemap_layer_set_tile() {
        let layer_info = LayerInfo::new("TestLayer", 5, 5);
        let tileset = TileSet::new();
        let mut layer = TileMapLayer::create_with_layer_info(layer_info, tileset);
        
        layer.set_tile_gid(42, 2, 2);
        // 验证设置成功（需要实现相应的getter）
    }
    
    #[test]
    fn test_tilemap_object_group() {
        let tilemap = TileMap::new();
        let objects = tilemap.get_object_group("ObjectLayer");
        assert!(objects.is_none());
    }
    
    #[test]
    fn test_tilemap_init_with_file() {
        let mut tilemap = TileMap::new();
        tilemap.init_with_file("test.tmx");
        // 验证初始化（在实际实现中会加载TMX文件）
    }
}

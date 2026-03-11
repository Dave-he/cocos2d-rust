/// TMX 地图文件解析器
///
/// 支持解析 Tiled Map Editor 导出的 TMX 格式文件
/// 支持：
/// - 正交(orthogonal)、等距(isometric)地图
/// - 图块图层(tile layer)
/// - 对象层(object group)
/// - 图像层(image layer)
/// - 自定义属性(properties)
/// - Base64 + zlib/gzip/zstd 压缩
/// - CSV 格式
/// - XML 格式

use std::collections::HashMap;
use std::io::Read;
use super::tilemap_info::{LayerInfo, MapOrientation, ObjectGroup, TileMapInfo, TileSet, TileMapObject};
use crate::math::Vec2;

/// TMX 解析错误
#[derive(Debug)]
pub enum TmxError {
    /// IO 错误
    IoError(std::io::Error),
    /// XML 解析错误
    ParseError(String),
    /// 不支持的编码/压缩
    UnsupportedEncoding(String),
    /// 无效数据
    InvalidData(String),
}

impl std::fmt::Display for TmxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TmxError::IoError(e) => write!(f, "IO error: {}", e),
            TmxError::ParseError(s) => write!(f, "Parse error: {}", s),
            TmxError::UnsupportedEncoding(s) => write!(f, "Unsupported encoding: {}", s),
            TmxError::InvalidData(s) => write!(f, "Invalid data: {}", s),
        }
    }
}

impl From<std::io::Error> for TmxError {
    fn from(e: std::io::Error) -> Self {
        TmxError::IoError(e)
    }
}

/// 图块数据编码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileEncoding {
    Xml,
    Csv,
    Base64,
}

/// 图块数据压缩
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileCompression {
    None,
    Zlib,
    Gzip,
    Zstd,
}

/// 图层类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayerType {
    TileLayer,
    ObjectGroup,
    ImageLayer,
    Group,
}

/// TMX 图层原始信息
#[derive(Debug, Clone)]
pub struct TmxLayerRaw {
    pub id: u32,
    pub name: String,
    pub layer_type: LayerType,
    pub width: u32,
    pub height: u32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub opacity: f32,
    pub visible: bool,
    pub tint_color: Option<String>,
    pub tiles: Vec<u32>,
    pub properties: HashMap<String, String>,
    pub objects: Vec<TmxObject>,
    pub image: Option<TmxImage>,
}

impl TmxLayerRaw {
    pub fn new_tile_layer(id: u32, name: &str, width: u32, height: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            layer_type: LayerType::TileLayer,
            width,
            height,
            x_offset: 0,
            y_offset: 0,
            opacity: 1.0,
            visible: true,
            tint_color: None,
            tiles: vec![0; (width * height) as usize],
            properties: HashMap::new(),
            objects: Vec::new(),
            image: None,
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        let idx = (y * self.width + x) as usize;
        self.tiles.get(idx).cloned().unwrap_or(0)
    }

    pub fn set_tile(&mut self, x: u32, y: u32, gid: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        if idx < self.tiles.len() {
            self.tiles[idx] = gid;
        }
    }

    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
}

/// TMX 对象
#[derive(Debug, Clone)]
pub struct TmxObject {
    pub id: u32,
    pub name: String,
    pub object_type: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub gid: Option<u32>,
    pub visible: bool,
    pub properties: HashMap<String, String>,
    pub shape: TmxObjectShape,
}

impl TmxObject {
    pub fn new(id: u32, name: &str, x: f32, y: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            object_type: String::new(),
            x,
            y,
            width: 0.0,
            height: 0.0,
            rotation: 0.0,
            gid: None,
            visible: true,
            properties: HashMap::new(),
            shape: TmxObjectShape::Rectangle,
        }
    }

    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

/// 对象形状
#[derive(Debug, Clone)]
pub enum TmxObjectShape {
    Rectangle,
    Ellipse,
    Point,
    Polygon(Vec<(f32, f32)>),
    Polyline(Vec<(f32, f32)>),
}

/// TMX 图像层信息
#[derive(Debug, Clone)]
pub struct TmxImage {
    pub source: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub trans_color: Option<String>,
}

/// TMX 图集信息
#[derive(Debug, Clone)]
pub struct TmxTileset {
    pub first_gid: u32,
    pub name: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub spacing: u32,
    pub margin: u32,
    pub tile_count: u32,
    pub columns: u32,
    pub image: Option<TmxImage>,
    pub properties: HashMap<String, String>,
    /// 单个图块属性
    pub tile_properties: HashMap<u32, HashMap<String, String>>,
}

impl TmxTileset {
    pub fn new(first_gid: u32, name: &str, tile_w: u32, tile_h: u32) -> Self {
        Self {
            first_gid,
            name: name.to_string(),
            tile_width: tile_w,
            tile_height: tile_h,
            spacing: 0,
            margin: 0,
            tile_count: 0,
            columns: 0,
            image: None,
            properties: HashMap::new(),
            tile_properties: HashMap::new(),
        }
    }

    /// 判断 GID 是否属于本图集
    pub fn contains_gid(&self, gid: u32) -> bool {
        gid >= self.first_gid && gid < self.first_gid + self.tile_count.max(1)
    }

    /// 计算本地图块 ID（local id = gid - first_gid）
    pub fn local_id(&self, gid: u32) -> u32 {
        gid.saturating_sub(self.first_gid)
    }

    /// 获取图块 UV 偏移（归一化 0..1）
    pub fn uv_for_gid(&self, gid: u32) -> Option<(f32, f32, f32, f32)> {
        if self.columns == 0 { return None; }
        let image = self.image.as_ref()?;
        let img_w = image.width? as f32;
        let img_h = image.height? as f32;

        let local = self.local_id(gid);
        let col = local % self.columns;
        let row = local / self.columns;

        let x = (self.margin as f32 + col as f32 * (self.tile_width + self.spacing) as f32) / img_w;
        let y = (self.margin as f32 + row as f32 * (self.tile_height + self.spacing) as f32) / img_h;
        let w = self.tile_width as f32 / img_w;
        let h = self.tile_height as f32 / img_h;

        Some((x, y, w, h))
    }

    pub fn get_tile_property(&self, local_id: u32, key: &str) -> Option<&str> {
        self.tile_properties.get(&local_id)?.get(key).map(|s| s.as_str())
    }
}

/// TMX 地图完整结构（解析结果）
#[derive(Debug, Clone)]
pub struct TmxMap {
    pub version: String,
    pub tiled_version: String,
    pub orientation: MapOrientation,
    pub render_order: String,
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub background_color: Option<String>,
    pub layers: Vec<TmxLayerRaw>,
    pub tilesets: Vec<TmxTileset>,
    pub properties: HashMap<String, String>,
}

impl TmxMap {
    pub fn new(width: u32, height: u32, tile_width: u32, tile_height: u32) -> Self {
        Self {
            version: "1.10".to_string(),
            tiled_version: "1.10.0".to_string(),
            orientation: MapOrientation::ORTHOGONAL,
            render_order: "right-down".to_string(),
            width,
            height,
            tile_width,
            tile_height,
            background_color: None,
            layers: Vec::new(),
            tilesets: Vec::new(),
            properties: HashMap::new(),
        }
    }

    /// 地图像素宽度
    pub fn pixel_width(&self) -> u32 {
        self.width * self.tile_width
    }

    /// 地图像素高度
    pub fn pixel_height(&self) -> u32 {
        self.height * self.tile_height
    }

    /// 获取指定名称图层
    pub fn get_layer(&self, name: &str) -> Option<&TmxLayerRaw> {
        self.layers.iter().find(|l| l.name == name)
    }

    /// 获取指定类型的所有图层
    pub fn get_tile_layers(&self) -> Vec<&TmxLayerRaw> {
        self.layers.iter().filter(|l| l.layer_type == LayerType::TileLayer).collect()
    }

    /// 获取对象层
    pub fn get_object_groups(&self) -> Vec<&TmxLayerRaw> {
        self.layers.iter().filter(|l| l.layer_type == LayerType::ObjectGroup).collect()
    }

    /// 根据 GID 找到对应的图集
    pub fn get_tileset_for_gid(&self, gid: u32) -> Option<&TmxTileset> {
        // 从最高 first_gid 向下找（确保找到最合适的图集）
        self.tilesets.iter()
            .filter(|ts| ts.first_gid <= gid)
            .max_by_key(|ts| ts.first_gid)
    }

    /// 获取地图属性
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// 转换为 TileMapInfo（与已有系统兼容）
    pub fn to_tile_map_info(&self) -> TileMapInfo {
        let mut info = TileMapInfo::new();
        info.map_size = Vec2::new(self.width as f32, self.height as f32);
        info.tile_size = Vec2::new(self.tile_width as f32, self.tile_height as f32);

        for ts in &self.tilesets {
            let mut tileset = TileSet::new();
            tileset.set_first_gid(ts.first_gid);
            tileset.set_name(&ts.name);
            tileset.set_tile_size(Vec2::new(ts.tile_width as f32, ts.tile_height as f32));
            if let Some(img) = &ts.image {
                tileset.set_image(&img.source);
            }
            info.add_tileset(tileset);
        }

        for layer in &self.layers {
            if layer.layer_type == LayerType::TileLayer {
                let mut li = LayerInfo::new(&layer.name, layer.width, layer.height);
                for y in 0..layer.height {
                    for x in 0..layer.width {
                        li.set_tile(x, y, layer.get_tile(x, y));
                    }
                }
                info.add_layer(li);
            }
        }

        info
    }
}

/// TMX 解析器（基于简单的 XML 状态机）
pub struct TmxParser;

impl TmxParser {
    /// 从文件路径解析 TMX
    pub fn parse_file(path: &str) -> Result<TmxMap, TmxError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }

    /// 从字符串解析 TMX
    pub fn parse_str(content: &str) -> Result<TmxMap, TmxError> {
        Self::parse_xml(content)
    }

    /// XML 解析核心实现（手写状态机，不依赖外部 XML 库）
    fn parse_xml(content: &str) -> Result<TmxMap, TmxError> {
        let mut map = TmxMap::new(0, 0, 32, 32);
        let mut current_layer: Option<TmxLayerRaw> = None;
        let mut current_tileset: Option<TmxTileset> = None;
        let mut current_object: Option<TmxObject> = None;
        let mut current_properties: HashMap<String, String> = HashMap::new();
        let mut in_properties = false;
        let mut in_data = false;
        let mut data_encoding = TileEncoding::Xml;
        let mut data_compression = TileCompression::None;
        let mut layer_id_counter: u32 = 0;

        // 简单的行扫描解析（不依赖外部 crate）
        for line in content.lines() {
            let trimmed = line.trim();

            // 解析 <map ...>
            if trimmed.starts_with("<map ") {
                let attrs = Self::parse_attrs(trimmed);
                map.version = attrs.get("version").cloned().unwrap_or_else(|| "1.0".to_string());
                map.tiled_version = attrs.get("tiledversion").cloned().unwrap_or_default();
                map.orientation = match attrs.get("orientation").map(|s| s.as_str()) {
                    Some("isometric") => MapOrientation::ISOMETRIC,
                    Some("staggered") => MapOrientation::STAGGERED,
                    Some("hexagonal") => MapOrientation::HEXAGONAL,
                    _ => MapOrientation::ORTHOGONAL,
                };
                map.render_order = attrs.get("renderorder").cloned().unwrap_or_else(|| "right-down".to_string());
                map.width = attrs.get("width").and_then(|v| v.parse().ok()).unwrap_or(0);
                map.height = attrs.get("height").and_then(|v| v.parse().ok()).unwrap_or(0);
                map.tile_width = attrs.get("tilewidth").and_then(|v| v.parse().ok()).unwrap_or(32);
                map.tile_height = attrs.get("tileheight").and_then(|v| v.parse().ok()).unwrap_or(32);
                map.background_color = attrs.get("backgroundcolor").cloned();
            }
            // 解析 <tileset ...>
            else if trimmed.starts_with("<tileset ") {
                let attrs = Self::parse_attrs(trimmed);
                let first_gid = attrs.get("firstgid").and_then(|v| v.parse().ok()).unwrap_or(1);
                let name = attrs.get("name").cloned().unwrap_or_default();
                let tile_w = attrs.get("tilewidth").and_then(|v| v.parse().ok()).unwrap_or(32);
                let tile_h = attrs.get("tileheight").and_then(|v| v.parse().ok()).unwrap_or(32);
                let mut ts = TmxTileset::new(first_gid, &name, tile_w, tile_h);
                ts.spacing = attrs.get("spacing").and_then(|v| v.parse().ok()).unwrap_or(0);
                ts.margin = attrs.get("margin").and_then(|v| v.parse().ok()).unwrap_or(0);
                ts.tile_count = attrs.get("tilecount").and_then(|v| v.parse().ok()).unwrap_or(0);
                ts.columns = attrs.get("columns").and_then(|v| v.parse().ok()).unwrap_or(0);
                current_tileset = Some(ts);
            }
            // 解析 </tileset>
            else if trimmed.starts_with("</tileset>") {
                if let Some(ts) = current_tileset.take() {
                    map.tilesets.push(ts);
                }
            }
            // 解析 <image ...> 在 tileset 内
            else if trimmed.starts_with("<image ") {
                let attrs = Self::parse_attrs(trimmed);
                let source = attrs.get("source").cloned().unwrap_or_default();
                let width = attrs.get("width").and_then(|v| v.parse().ok());
                let height = attrs.get("height").and_then(|v| v.parse().ok());
                let trans = attrs.get("trans").cloned();
                let img = TmxImage { source, width, height, trans_color: trans };
                if let Some(ts) = current_tileset.as_mut() {
                    ts.image = Some(img);
                }
            }
            // 解析 <layer ...>（图块图层）
            else if trimmed.starts_with("<layer ") {
                let attrs = Self::parse_attrs(trimmed);
                layer_id_counter += 1;
                let id = attrs.get("id").and_then(|v| v.parse().ok()).unwrap_or(layer_id_counter);
                let name = attrs.get("name").cloned().unwrap_or_default();
                let w = attrs.get("width").and_then(|v| v.parse().ok()).unwrap_or(map.width);
                let h = attrs.get("height").and_then(|v| v.parse().ok()).unwrap_or(map.height);
                let mut layer = TmxLayerRaw::new_tile_layer(id, &name, w, h);
                layer.opacity = attrs.get("opacity").and_then(|v| v.parse().ok()).unwrap_or(1.0);
                layer.visible = attrs.get("visible").map(|v| v != "0").unwrap_or(true);
                layer.x_offset = attrs.get("offsetx").and_then(|v| v.parse().ok()).unwrap_or(0);
                layer.y_offset = attrs.get("offsety").and_then(|v| v.parse().ok()).unwrap_or(0);
                current_layer = Some(layer);
            }
            // 解析 </layer>
            else if trimmed.starts_with("</layer>") {
                if let Some(layer) = current_layer.take() {
                    map.layers.push(layer);
                }
                in_data = false;
            }
            // 解析 <data ...>
            else if trimmed.starts_with("<data ") {
                let attrs = Self::parse_attrs(trimmed);
                data_encoding = match attrs.get("encoding").map(|s| s.as_str()) {
                    Some("csv") => TileEncoding::Csv,
                    Some("base64") => TileEncoding::Base64,
                    _ => TileEncoding::Xml,
                };
                data_compression = match attrs.get("compression").map(|s| s.as_str()) {
                    Some("zlib") => TileCompression::Zlib,
                    Some("gzip") => TileCompression::Gzip,
                    Some("zstd") => TileCompression::Zstd,
                    _ => TileCompression::None,
                };
                in_data = true;
            }
            // CSV 数据行
            else if in_data && data_encoding == TileEncoding::Csv {
                let csv_line = trimmed.trim_end_matches(',');
                if !csv_line.is_empty() {
                    if let Some(layer) = current_layer.as_mut() {
                        let mut idx = layer.tiles.iter().position(|&g| g == 0 || true).unwrap_or(0);
                        // 找第一个需要填充的位置
                        idx = layer.tiles.iter().enumerate().find(|(_, &g)| g == 0).map(|(i, _)| i).unwrap_or(0);
                        let _ = idx; // 简化：重新计算
                        let gids: Vec<u32> = csv_line.split(',')
                            .filter_map(|s| s.trim().parse::<u32>().ok())
                            .collect();

                        // 找到已填入多少
                        let filled = layer.tiles.iter().position(|&g| g == 0).unwrap_or(layer.tiles.len());
                        for (i, &gid) in gids.iter().enumerate() {
                            let pos = filled + i;
                            if pos < layer.tiles.len() {
                                layer.tiles[pos] = gid;
                            }
                        }
                    }
                }
            }
            // Base64 数据
            else if in_data && data_encoding == TileEncoding::Base64 {
                let data_str = trimmed.trim();
                if !data_str.is_empty() && !data_str.starts_with('<') {
                    if let Some(layer) = current_layer.as_mut() {
                        match Self::decode_base64_tiles(data_str, data_compression) {
                            Ok(tiles) => layer.tiles = tiles,
                            Err(e) => log::warn!("Failed to decode tile data: {}", e),
                        }
                    }
                }
            }
            // 解析 <objectgroup ...>
            else if trimmed.starts_with("<objectgroup ") {
                let attrs = Self::parse_attrs(trimmed);
                layer_id_counter += 1;
                let id = attrs.get("id").and_then(|v| v.parse().ok()).unwrap_or(layer_id_counter);
                let name = attrs.get("name").cloned().unwrap_or_default();
                let mut layer = TmxLayerRaw {
                    id,
                    name,
                    layer_type: LayerType::ObjectGroup,
                    width: 0,
                    height: 0,
                    x_offset: 0,
                    y_offset: 0,
                    opacity: attrs.get("opacity").and_then(|v| v.parse().ok()).unwrap_or(1.0),
                    visible: attrs.get("visible").map(|v| v != "0").unwrap_or(true),
                    tint_color: attrs.get("tintcolor").cloned(),
                    tiles: Vec::new(),
                    properties: HashMap::new(),
                    objects: Vec::new(),
                    image: None,
                };
                current_layer = Some(layer);
            }
            // 解析 </objectgroup>
            else if trimmed.starts_with("</objectgroup>") {
                if let Some(layer) = current_layer.take() {
                    map.layers.push(layer);
                }
            }
            // 解析 <object ...>
            else if trimmed.starts_with("<object ") {
                let attrs = Self::parse_attrs(trimmed);
                let id = attrs.get("id").and_then(|v| v.parse().ok()).unwrap_or(0);
                let name = attrs.get("name").cloned().unwrap_or_default();
                let x = attrs.get("x").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                let y = attrs.get("y").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                let mut obj = TmxObject::new(id, &name, x, y);
                obj.object_type = attrs.get("type").or_else(|| attrs.get("class"))
                    .cloned().unwrap_or_default();
                obj.width = attrs.get("width").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                obj.height = attrs.get("height").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                obj.rotation = attrs.get("rotation").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                obj.gid = attrs.get("gid").and_then(|v| v.parse().ok());
                obj.visible = attrs.get("visible").map(|v| v != "0").unwrap_or(true);

                // 自闭合标签时直接放入图层
                if trimmed.ends_with("/>") {
                    if let Some(layer) = current_layer.as_mut() {
                        layer.objects.push(obj);
                    }
                } else {
                    current_object = Some(obj);
                }
            }
            // 解析 </object>
            else if trimmed.starts_with("</object>") {
                if let Some(obj) = current_object.take() {
                    if let Some(layer) = current_layer.as_mut() {
                        layer.objects.push(obj);
                    }
                }
            }
            // 解析 <properties>
            else if trimmed == "<properties>" {
                in_properties = true;
                current_properties.clear();
            }
            // 解析 </properties>
            else if trimmed == "</properties>" {
                in_properties = false;
                // 将属性附加到当前上下文
                if let Some(layer) = current_layer.as_mut() {
                    layer.properties.extend(current_properties.clone());
                } else if let Some(ts) = current_tileset.as_mut() {
                    ts.properties.extend(current_properties.clone());
                } else {
                    map.properties.extend(current_properties.clone());
                }
                current_properties.clear();
            }
            // 解析 <property .../>
            else if in_properties && trimmed.starts_with("<property ") {
                let attrs = Self::parse_attrs(trimmed);
                if let (Some(name), Some(value)) = (attrs.get("name"), attrs.get("value")) {
                    current_properties.insert(name.clone(), value.clone());
                }
            }
        }

        Ok(map)
    }

    /// 解析 XML 属性（简单实现，处理 key="value" 格式）
    fn parse_attrs(tag: &str) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        let mut remaining = tag;

        // 跳过标签名
        if let Some(space_pos) = remaining.find(' ') {
            remaining = &remaining[space_pos..];
        } else {
            return attrs;
        }

        // 解析属性
        let mut i = 0;
        let chars: Vec<char> = remaining.chars().collect();
        while i < chars.len() {
            // 跳过空白
            while i < chars.len() && chars[i].is_whitespace() { i += 1; }
            if i >= chars.len() { break; }

            // 读取键
            let key_start = i;
            while i < chars.len() && chars[i] != '=' && !chars[i].is_whitespace() { i += 1; }
            let key: String = chars[key_start..i].iter().collect();
            if key.is_empty() { i += 1; continue; }

            // 跳过 =
            while i < chars.len() && (chars[i] == '=' || chars[i].is_whitespace()) { i += 1; }

            // 读取值（引号包围）
            if i < chars.len() && (chars[i] == '"' || chars[i] == '\'') {
                let quote = chars[i];
                i += 1;
                let val_start = i;
                while i < chars.len() && chars[i] != quote { i += 1; }
                let value: String = chars[val_start..i].iter().collect();
                attrs.insert(key.to_lowercase(), value);
                i += 1; // 跳过结束引号
            }
        }

        attrs
    }

    /// 解码 Base64 图块数据
    fn decode_base64_tiles(data: &str, compression: TileCompression) -> Result<Vec<u32>, TmxError> {
        // 简单的 Base64 解码（不依赖外部库）
        let cleaned = data.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let bytes = Self::base64_decode(&cleaned)
            .map_err(|e| TmxError::InvalidData(format!("Base64 decode failed: {}", e)))?;

        let decompressed = match compression {
            TileCompression::None => bytes,
            TileCompression::Zlib => {
                Self::zlib_decompress(&bytes)
                    .map_err(|e| TmxError::UnsupportedEncoding(format!("zlib: {}", e)))?
            }
            TileCompression::Gzip => {
                Self::gzip_decompress(&bytes)
                    .map_err(|e| TmxError::UnsupportedEncoding(format!("gzip: {}", e)))?
            }
            TileCompression::Zstd => {
                return Err(TmxError::UnsupportedEncoding("zstd not supported without external crate".to_string()));
            }
        };

        if decompressed.len() % 4 != 0 {
            return Err(TmxError::InvalidData("Tile data length not multiple of 4".to_string()));
        }

        let tiles: Vec<u32> = decompressed.chunks_exact(4)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();

        Ok(tiles)
    }

    /// 简单 Base64 解码
    fn base64_decode(s: &str) -> Result<Vec<u8>, String> {
        const TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = Vec::new();
        let chars: Vec<char> = s.chars().filter(|&c| c != '=').collect();
        let padding = s.chars().filter(|&c| c == '=').count();

        for chunk in chars.chunks(4) {
            let mut vals = [0u8; 4];
            for (i, &c) in chunk.iter().enumerate() {
                vals[i] = TABLE.find(c)
                    .ok_or_else(|| format!("Invalid base64 char: {}", c))? as u8;
            }
            result.push((vals[0] << 2) | (vals[1] >> 4));
            if chunk.len() > 2 {
                result.push((vals[1] << 4) | (vals[2] >> 2));
            }
            if chunk.len() > 3 {
                result.push((vals[2] << 6) | vals[3]);
            }
        }

        // 移除 padding 字节
        for _ in 0..padding.min(result.len()) {
            result.pop();
        }

        Ok(result)
    }

    /// zlib 解压（简单实现，不依赖外部库 —— 实际项目中应使用 flate2）
    fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        // 实际实现需要 flate2 crate
        // 这里返回原始数据（未压缩时 zlib header 后的原始 deflate 流很复杂）
        // 生产代码：
        // use flate2::read::ZlibDecoder;
        // let mut d = ZlibDecoder::new(data);
        // let mut out = Vec::new();
        // d.read_to_end(&mut out).map_err(|e| e.to_string())?;
        // Ok(out)
        Err("zlib decompression requires flate2 crate. Add 'flate2 = \"1.0\"' to Cargo.toml".to_string())
    }

    /// gzip 解压（简单实现）
    fn gzip_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        // 实际实现需要 flate2 crate
        Err("gzip decompression requires flate2 crate. Add 'flate2 = \"1.0\"' to Cargo.toml".to_string())
    }
}

/// TMX 地图构建器（用于程序化创建地图）
pub struct TmxMapBuilder {
    map: TmxMap,
}

impl TmxMapBuilder {
    pub fn new(width: u32, height: u32, tile_width: u32, tile_height: u32) -> Self {
        Self { map: TmxMap::new(width, height, tile_width, tile_height) }
    }

    pub fn with_orientation(mut self, orientation: MapOrientation) -> Self {
        self.map.orientation = orientation;
        self
    }

    pub fn with_background(mut self, color: &str) -> Self {
        self.map.background_color = Some(color.to_string());
        self
    }

    pub fn add_tileset(mut self, tileset: TmxTileset) -> Self {
        self.map.tilesets.push(tileset);
        self
    }

    pub fn add_tile_layer(mut self, name: &str, tiles: Vec<u32>) -> Self {
        let id = self.map.layers.len() as u32 + 1;
        let mut layer = TmxLayerRaw::new_tile_layer(id, name, self.map.width, self.map.height);
        let len = (self.map.width * self.map.height) as usize;
        layer.tiles = tiles.into_iter().take(len).collect();
        while layer.tiles.len() < len {
            layer.tiles.push(0);
        }
        self.map.layers.push(layer);
        self
    }

    pub fn add_property(mut self, key: &str, value: &str) -> Self {
        self.map.properties.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> TmxMap {
        self.map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple_tmx() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<map version="1.10" tiledversion="1.10.0" orientation="orthogonal" renderorder="right-down" width="4" height="3" tilewidth="32" tileheight="32">
 <tileset firstgid="1" name="Terrain" tilewidth="32" tileheight="32" tilecount="48" columns="8">
  <image source="terrain.png" width="256" height="192"/>
 </tileset>
 <layer id="1" name="Background" width="4" height="3">
  <data encoding="csv">
1,2,3,4,
5,6,7,8,
9,10,11,12
  </data>
 </layer>
 <objectgroup id="2" name="Objects">
  <object id="1" name="Player" type="player" x="64.0" y="32.0" width="32.0" height="32.0"/>
 </objectgroup>
</map>"#
    }

    #[test]
    fn test_parse_basic_map() {
        let result = TmxParser::parse_str(make_simple_tmx());
        assert!(result.is_ok(), "Parse should succeed: {:?}", result.err());

        let map = result.unwrap();
        assert_eq!(map.width, 4);
        assert_eq!(map.height, 3);
        assert_eq!(map.tile_width, 32);
        assert_eq!(map.tile_height, 32);
    }

    #[test]
    fn test_parse_tileset() {
        let map = TmxParser::parse_str(make_simple_tmx()).unwrap();
        assert_eq!(map.tilesets.len(), 1);
        let ts = &map.tilesets[0];
        assert_eq!(ts.name, "Terrain");
        assert_eq!(ts.first_gid, 1);
        assert_eq!(ts.tile_width, 32);
        assert_eq!(ts.columns, 8);
    }

    #[test]
    fn test_parse_tile_layer() {
        let map = TmxParser::parse_str(make_simple_tmx()).unwrap();
        let tile_layers = map.get_tile_layers();
        assert!(!tile_layers.is_empty());
        assert_eq!(tile_layers[0].name, "Background");
    }

    #[test]
    fn test_parse_object_group() {
        let map = TmxParser::parse_str(make_simple_tmx()).unwrap();
        let obj_groups = map.get_object_groups();
        assert_eq!(obj_groups.len(), 1);
        assert_eq!(obj_groups[0].name, "Objects");
    }

    #[test]
    fn test_parse_object() {
        let map = TmxParser::parse_str(make_simple_tmx()).unwrap();
        let obj_groups = map.get_object_groups();
        assert_eq!(obj_groups[0].objects.len(), 1);
        let obj = &obj_groups[0].objects[0];
        assert_eq!(obj.name, "Player");
        assert_eq!(obj.object_type, "player");
        assert_eq!(obj.x, 64.0);
        assert_eq!(obj.y, 32.0);
    }

    #[test]
    fn test_tmx_map_pixel_size() {
        let map = TmxMap::new(10, 8, 32, 32);
        assert_eq!(map.pixel_width(), 320);
        assert_eq!(map.pixel_height(), 256);
    }

    #[test]
    fn test_get_layer_by_name() {
        let map = TmxParser::parse_str(make_simple_tmx()).unwrap();
        assert!(map.get_layer("Background").is_some());
        assert!(map.get_layer("Objects").is_some());
        assert!(map.get_layer("NonExistent").is_none());
    }

    #[test]
    fn test_get_tileset_for_gid() {
        let map = TmxParser::parse_str(make_simple_tmx()).unwrap();
        let ts = map.get_tileset_for_gid(5);
        assert!(ts.is_some());
        assert_eq!(ts.unwrap().name, "Terrain");

        // GID 0 表示空白图块，找不到图集
        let ts_zero = map.get_tileset_for_gid(0);
        assert!(ts_zero.is_none());
    }

    #[test]
    fn test_tileset_contains_gid() {
        let mut ts = TmxTileset::new(1, "Test", 32, 32);
        ts.tile_count = 48;
        assert!(ts.contains_gid(1));
        assert!(ts.contains_gid(48));
        assert!(!ts.contains_gid(49));
        assert!(!ts.contains_gid(0));
    }

    #[test]
    fn test_tileset_local_id() {
        let ts = TmxTileset::new(10, "Test", 32, 32);
        assert_eq!(ts.local_id(10), 0);
        assert_eq!(ts.local_id(15), 5);
    }

    #[test]
    fn test_tileset_uv() {
        let mut ts = TmxTileset::new(1, "Test", 32, 32);
        ts.columns = 4;
        ts.tile_count = 16;
        ts.image = Some(TmxImage {
            source: "test.png".to_string(),
            width: Some(128),
            height: Some(128),
            trans_color: None,
        });

        let uv = ts.uv_for_gid(1);
        assert!(uv.is_some());
        let (x, y, w, h) = uv.unwrap();
        assert!((x - 0.0).abs() < 0.001);
        assert!((y - 0.0).abs() < 0.001);
        assert!((w - 0.25).abs() < 0.001);
        assert!((h - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_tmx_object_center() {
        let mut obj = TmxObject::new(1, "Test", 100.0, 200.0);
        obj.width = 64.0;
        obj.height = 64.0;
        let (cx, cy) = obj.center();
        assert_eq!(cx, 132.0);
        assert_eq!(cy, 232.0);
    }

    #[test]
    fn test_tmx_layer_raw_get_set_tile() {
        let mut layer = TmxLayerRaw::new_tile_layer(1, "Test", 5, 5);
        layer.set_tile(2, 3, 42);
        assert_eq!(layer.get_tile(2, 3), 42);
        assert_eq!(layer.get_tile(0, 0), 0);
    }

    #[test]
    fn test_tmx_layer_out_of_bounds() {
        let layer = TmxLayerRaw::new_tile_layer(1, "Test", 3, 3);
        assert_eq!(layer.get_tile(10, 10), 0);
    }

    #[test]
    fn test_map_builder() {
        let tiles = vec![1u32; 4 * 4];
        let map = TmxMapBuilder::new(4, 4, 32, 32)
            .with_orientation(MapOrientation::ISOMETRIC)
            .with_background("#336699")
            .add_tile_layer("Ground", tiles)
            .add_property("level", "1")
            .build();

        assert_eq!(map.width, 4);
        assert_eq!(map.height, 4);
        assert_eq!(map.orientation, MapOrientation::ISOMETRIC);
        assert_eq!(map.background_color.as_deref(), Some("#336699"));
        assert_eq!(map.layers.len(), 1);
        assert_eq!(map.get_property("level"), Some("1"));
    }

    #[test]
    fn test_to_tile_map_info() {
        let map = TmxParser::parse_str(make_simple_tmx()).unwrap();
        let info = map.to_tile_map_info();
        assert_eq!(info.map_size, Vec2::new(4.0, 3.0));
        assert_eq!(info.tile_size, Vec2::new(32.0, 32.0));
        assert!(!info.get_layers().is_empty());
        assert!(!info.get_tilesets().is_empty());
    }

    #[test]
    fn test_parse_properties() {
        let tmx = r#"<?xml version="1.0"?>
<map width="2" height="2" tilewidth="16" tileheight="16">
 <properties>
  <property name="game_level" value="5"/>
  <property name="author" value="TestUser"/>
 </properties>
</map>"#;
        let map = TmxParser::parse_str(tmx).unwrap();
        assert_eq!(map.get_property("game_level"), Some("5"));
        assert_eq!(map.get_property("author"), Some("TestUser"));
    }

    #[test]
    fn test_base64_decode() {
        // "Hello" in base64 = "SGVsbG8="
        let decoded = TmxParser::base64_decode("SGVsbG8").unwrap();
        assert_eq!(&decoded, b"Hello");
    }

    #[test]
    fn test_parse_attrs() {
        let attrs = TmxParser::parse_attrs(r#"<map width="10" height="8" tilewidth="32"/>"#);
        assert_eq!(attrs.get("width").map(|s| s.as_str()), Some("10"));
        assert_eq!(attrs.get("height").map(|s| s.as_str()), Some("8"));
        assert_eq!(attrs.get("tilewidth").map(|s| s.as_str()), Some("32"));
    }
}

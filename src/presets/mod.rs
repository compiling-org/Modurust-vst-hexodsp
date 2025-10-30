//! # Preset System
//!
//! Provides a library of pre-configured audio processing nodes and patches
//! that users can drag from the browser panel onto the canvas.

pub mod generator_presets;
pub mod filter_presets;
pub mod effect_presets;
pub mod utility_presets;
pub mod cookbook_presets;

use std::collections::HashMap;

/// Category of preset for browser organization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PresetCategory {
    Generators,   // Oscillators, samplers, noise
    Filters,      // LPF, HPF, BPF, notch
    Effects,      // Reverb, delay, chorus, distortion
    Utilities,    // Mixer, splitter, VCA, output
    Cookbook,     // Complex modular synthesis examples
}

impl PresetCategory {
    pub fn icon(&self) -> &'static str {
        match self {
            PresetCategory::Generators => "🎵",
            PresetCategory::Filters => "🔊",
            PresetCategory::Effects => "🌊",
            PresetCategory::Utilities => "🎚️",
            PresetCategory::Cookbook => "📖",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PresetCategory::Generators => "Generators",
            PresetCategory::Filters => "Filters",
            PresetCategory::Effects => "Effects",
            PresetCategory::Utilities => "Utilities",
            PresetCategory::Cookbook => "Cookbook Examples",
        }
    }
}

/// A parameter for a preset node
#[derive(Debug, Clone)]
pub struct PresetParameter {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub default: f32,
}

impl PresetParameter {
    pub fn new(name: &str, value: f32, min: f32, max: f32) -> Self {
        Self {
            name: name.to_string(),
            value,
            min,
            max,
            default: value,
        }
    }
}

/// A single node within a preset (for complex presets)
#[derive(Debug, Clone)]
pub struct PresetNode {
    pub id: String,
    pub node_type: String,  // e.g., "generator.sine", "filter.lpf"
    pub parameters: Vec<PresetParameter>,
    pub position: (f32, f32),  // Relative position for layout
}

impl PresetNode {
    pub fn new(id: &str, node_type: &str, position: (f32, f32)) -> Self {
        Self {
            id: id.to_string(),
            node_type: node_type.to_string(),
            parameters: Vec::new(),
            position,
        }
    }

    pub fn with_parameter(mut self, name: &str, value: f32, min: f32, max: f32) -> Self {
        self.parameters.push(PresetParameter::new(name, value, min, max));
        self
    }
}

/// A connection between nodes in a preset
#[derive(Debug, Clone)]
pub struct PresetConnection {
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

impl PresetConnection {
    pub fn new(from_node: &str, from_port: &str, to_node: &str, to_port: &str) -> Self {
        Self {
            from_node: from_node.to_string(),
            from_port: from_port.to_string(),
            to_node: to_node.to_string(),
            to_port: to_port.to_string(),
        }
    }
}

/// A complete preset that can be instantiated on the canvas
#[derive(Debug, Clone)]
pub struct NodePreset {
    pub name: String,
    pub description: String,
    pub category: PresetCategory,
    pub nodes: Vec<PresetNode>,
    pub connections: Vec<PresetConnection>,
    pub tags: Vec<String>,
}

impl NodePreset {
    /// Create a simple single-node preset
    pub fn simple(name: &str, description: &str, category: PresetCategory, node_type: &str) -> Self {
        let node = PresetNode::new("main", node_type, (0.0, 0.0));
        
        Self {
            name: name.to_string(),
            description: description.to_string(),
            category,
            nodes: vec![node],
            connections: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Create a complex multi-node preset
    pub fn complex(
        name: &str,
        description: &str,
        category: PresetCategory,
        nodes: Vec<PresetNode>,
        connections: Vec<PresetConnection>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            category,
            nodes,
            connections,
            tags: Vec::new(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Check if preset matches search query
    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        
        self.name.to_lowercase().contains(&query_lower)
            || self.description.to_lowercase().contains(&query_lower)
            || self.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
    }
}

/// Preset library that holds all available presets
pub struct PresetLibrary {
    presets: Vec<NodePreset>,
    by_category: HashMap<PresetCategory, Vec<usize>>,
}

impl PresetLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            presets: Vec::new(),
            by_category: HashMap::new(),
        };

        // Load all preset categories
        library.load_generators();
        library.load_filters();
        library.load_effects();
        library.load_utilities();
        library.load_cookbook();

        library
    }

    fn add_preset(&mut self, preset: NodePreset) {
        let category = preset.category.clone();
        let index = self.presets.len();
        
        self.presets.push(preset);
        
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(index);
    }

    fn load_generators(&mut self) {
        for preset in generator_presets::get_all() {
            self.add_preset(preset);
        }
    }

    fn load_filters(&mut self) {
        for preset in filter_presets::get_all() {
            self.add_preset(preset);
        }
    }

    fn load_effects(&mut self) {
        for preset in effect_presets::get_all() {
            self.add_preset(preset);
        }
    }

    fn load_utilities(&mut self) {
        for preset in utility_presets::get_all() {
            self.add_preset(preset);
        }
    }

    fn load_cookbook(&mut self) {
        for preset in cookbook_presets::get_all() {
            self.add_preset(preset);
        }
    }

    /// Get all presets
    pub fn all(&self) -> &[NodePreset] {
        &self.presets
    }

    /// Get presets by category
    pub fn by_category(&self, category: &PresetCategory) -> Vec<&NodePreset> {
        self.by_category
            .get(category)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&i| self.presets.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search presets
    pub fn search(&self, query: &str) -> Vec<&NodePreset> {
        if query.is_empty() {
            return self.presets.iter().collect();
        }

        self.presets
            .iter()
            .filter(|preset| preset.matches_search(query))
            .collect()
    }

    /// Get all categories
    pub fn categories(&self) -> Vec<PresetCategory> {
        vec![
            PresetCategory::Generators,
            PresetCategory::Filters,
            PresetCategory::Effects,
            PresetCategory::Utilities,
            PresetCategory::Cookbook,
        ]
    }
}

impl Default for PresetLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#![allow(dead_code)]
/// Theme Manager for HexoDSP DAW
///
/// Provides customizable UI themes with control over colors, fonts, brightness, and contrast
/// Inspired by modern DAWs like Bitwig, Ableton, and open-source alternatives

use eframe::egui::{self, Color32, Rounding, Stroke, FontId, FontFamily};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Implement serialization for Color32
mod color_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (r, g, b, a) = color.to_tuple();
        serializer.serialize_str(&format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() == 9 && s.starts_with('#') {
            let r = u8::from_str_radix(&s[1..3], 16).map_err(serde::de::Error::custom)?;
            let g = u8::from_str_radix(&s[3..5], 16).map_err(serde::de::Error::custom)?;
            let b = u8::from_str_radix(&s[5..7], 16).map_err(serde::de::Error::custom)?;
            let a = u8::from_str_radix(&s[7..9], 16).map_err(serde::de::Error::custom)?;
            Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
        } else {
            Err(serde::de::Error::custom("Invalid color format"))
        }
    }
}

// Implement serialization for Vec<Color32>
mod color_serde_vec {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(colors: &Vec<Color32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let color_strings: Vec<String> = colors
            .iter()
            .map(|color| {
                let (r, g, b, a) = color.to_tuple();
                format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
            })
            .collect();
        color_strings.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Color32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let color_strings: Vec<String> = Vec::deserialize(deserializer)?;
        let colors = color_strings
            .iter()
            .map(|s| {
                if s.len() == 9 && s.starts_with('#') {
                    let r = u8::from_str_radix(&s[1..3], 16).map_err(serde::de::Error::custom)?;
                    let g = u8::from_str_radix(&s[3..5], 16).map_err(serde::de::Error::custom)?;
                    let b = u8::from_str_radix(&s[5..7], 16).map_err(serde::de::Error::custom)?;
                    let a = u8::from_str_radix(&s[7..9], 16).map_err(serde::de::Error::custom)?;
                    Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
                } else {
                    Err(serde::de::Error::custom("Invalid color format"))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(colors)
    }
}

/// Theme preset for the application
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemePreset {
    pub name: String,
    #[serde(with = "color_serde")]
    pub background_color: Color32,
    #[serde(with = "color_serde")]
    pub text_color: Color32,
    #[serde(with = "color_serde")]
    pub accent_color: Color32,
    #[serde(with = "color_serde")]
    pub secondary_accent_color: Color32,
    #[serde(with = "color_serde_vec")]
    pub node_colors: Vec<Color32>,
    #[serde(with = "color_serde")]
    pub grid_color: Color32,
    #[serde(with = "color_serde")]
    pub connection_color: Color32,
    pub brightness: f32,
    pub contrast: f32,
    pub font_size: f32,
    pub font_family: String,
    pub node_rounding: f32,
    pub panel_rounding: f32,
}

impl Default for ThemePreset {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            background_color: Color32::from_rgb(30, 30, 30),
            text_color: Color32::from_rgb(220, 220, 220),
            accent_color: Color32::from_rgb(0, 120, 215),
            secondary_accent_color: Color32::from_rgb(0, 180, 100),
            node_colors: vec![
                Color32::from_rgb(70, 130, 180),   // Steel Blue
                Color32::from_rgb(220, 20, 60),    // Crimson
                Color32::from_rgb(50, 205, 50),    // Lime Green
                Color32::from_rgb(255, 165, 0),    // Orange
                Color32::from_rgb(138, 43, 226),   // Blue Violet
                Color32::from_rgb(30, 144, 255),   // Dodger Blue
                Color32::from_rgb(255, 105, 180),  // Hot Pink
                Color32::from_rgb(184, 134, 11),   // Dark Goldenrod
            ],
            grid_color: Color32::from_rgb(60, 60, 60),
            connection_color: Color32::from_rgb(180, 180, 180),
            brightness: 1.0,
            contrast: 1.0,
            font_size: 14.0,
            font_family: "Inter".to_string(),
            node_rounding: 6.0,
            panel_rounding: 4.0,
        }
    }
}

/// Predefined theme presets
pub fn dark_theme() -> ThemePreset {
    ThemePreset {
        name: "Dark".to_string(),
        background_color: Color32::from_rgb(20, 20, 20),
        text_color: Color32::from_rgb(220, 220, 220),
        accent_color: Color32::from_rgb(0, 120, 215),
        secondary_accent_color: Color32::from_rgb(0, 180, 100),
        ..ThemePreset::default()
    }
}

pub fn light_theme() -> ThemePreset {
    ThemePreset {
        name: "Light".to_string(),
        background_color: Color32::from_rgb(240, 240, 240),
        text_color: Color32::from_rgb(20, 20, 20),
        accent_color: Color32::from_rgb(0, 100, 200),
        secondary_accent_color: Color32::from_rgb(0, 150, 80),
        grid_color: Color32::from_rgb(200, 200, 200),
        connection_color: Color32::from_rgb(100, 100, 100),
        ..ThemePreset::default()
    }
}

pub fn bitwig_inspired() -> ThemePreset {
    ThemePreset {
        name: "Bitwig Inspired".to_string(),
        background_color: Color32::from_rgb(45, 45, 48),
        text_color: Color32::from_rgb(230, 230, 230),
        accent_color: Color32::from_rgb(77, 208, 225),
        secondary_accent_color: Color32::from_rgb(255, 188, 0),
        node_colors: vec![
            Color32::from_rgb(77, 208, 225),    // Cyan
            Color32::from_rgb(255, 188, 0),     // Yellow
            Color32::from_rgb(149, 117, 205),   // Purple
            Color32::from_rgb(255, 85, 85),     // Red
            Color32::from_rgb(89, 193, 53),     // Green
            Color32::from_rgb(59, 142, 234),    // Blue
            Color32::from_rgb(255, 140, 56),    // Orange
            Color32::from_rgb(175, 175, 175),   // Gray
        ],
        grid_color: Color32::from_rgb(65, 65, 68),
        connection_color: Color32::from_rgb(200, 200, 200),
        brightness: 1.0,
        contrast: 1.1,
        font_size: 13.0,
        font_family: "Roboto".to_string(),
        node_rounding: 5.0,
        panel_rounding: 3.0,
    }
}

pub fn ableton_inspired() -> ThemePreset {
    ThemePreset {
        name: "Ableton Inspired".to_string(),
        background_color: Color32::from_rgb(40, 40, 40),
        text_color: Color32::from_rgb(230, 230, 230),
        accent_color: Color32::from_rgb(255, 130, 0),
        secondary_accent_color: Color32::from_rgb(0, 230, 150),
        node_colors: vec![
            Color32::from_rgb(255, 130, 0),    // Orange
            Color32::from_rgb(0, 230, 150),    // Teal
            Color32::from_rgb(255, 80, 80),    // Red
            Color32::from_rgb(0, 160, 230),    // Blue
            Color32::from_rgb(230, 230, 0),    // Yellow
            Color32::from_rgb(230, 0, 230),    // Magenta
            Color32::from_rgb(100, 230, 0),    // Green
            Color32::from_rgb(200, 200, 200),  // Light Gray
        ],
        grid_color: Color32::from_rgb(60, 60, 60),
        connection_color: Color32::from_rgb(180, 180, 180),
        brightness: 1.0,
        contrast: 1.05,
        font_size: 13.0,
        font_family: "Helvetica".to_string(),
        node_rounding: 2.0,
        panel_rounding: 2.0,
    }
}

/// Theme Manager to handle theme selection and customization
pub struct ThemeManager {
    pub current_theme: ThemePreset,
    pub available_themes: Vec<ThemePreset>,
    themes_path: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_theme: ThemePreset::default(),
            available_themes: vec![
                ThemePreset::default(),
                dark_theme(),
                light_theme(),
                bitwig_inspired(),
                ableton_inspired(),
            ],
            themes_path: "themes".to_string(),
        };
        
        // Ensure themes directory exists
        if !Path::new(&manager.themes_path).exists() {
            if let Err(e) = fs::create_dir(&manager.themes_path) {
                eprintln!("Failed to create themes directory: {}", e);
            }
        }
        
        // Load custom themes
        manager.load_custom_themes();
        
        manager
    }
    
    /// Apply the current theme to the UI context
    pub fn apply_theme(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        
        // Apply colors
        visuals.window_fill = self.current_theme.background_color;
        visuals.panel_fill = self.current_theme.background_color;
        visuals.faint_bg_color = self.adjust_color(self.current_theme.background_color, 1.1);
        visuals.extreme_bg_color = self.adjust_color(self.current_theme.background_color, 0.9);
        
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.current_theme.text_color);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.current_theme.text_color);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, self.current_theme.text_color);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.current_theme.text_color);
        
        visuals.selection.bg_fill = self.current_theme.accent_color;
        visuals.selection.stroke = Stroke::new(1.0, self.current_theme.text_color);
        
        // Apply rounding
        visuals.window_rounding = Rounding::same(self.current_theme.panel_rounding);
        visuals.menu_rounding = Rounding::same(self.current_theme.panel_rounding);
        visuals.widgets.noninteractive.rounding = Rounding::same(self.current_theme.panel_rounding);
        visuals.widgets.inactive.rounding = Rounding::same(self.current_theme.panel_rounding);
        visuals.widgets.hovered.rounding = Rounding::same(self.current_theme.panel_rounding);
        visuals.widgets.active.rounding = Rounding::same(self.current_theme.panel_rounding);
        
        // Apply the visuals
        ctx.set_visuals(visuals);
        
        // Set default font size
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, FontId::new(self.current_theme.font_size * 1.5, FontFamily::Proportional)),
            (egui::TextStyle::Body, FontId::new(self.current_theme.font_size, FontFamily::Proportional)),
            (egui::TextStyle::Monospace, FontId::new(self.current_theme.font_size, FontFamily::Monospace)),
            (egui::TextStyle::Button, FontId::new(self.current_theme.font_size, FontFamily::Proportional)),
            (egui::TextStyle::Small, FontId::new(self.current_theme.font_size * 0.8, FontFamily::Proportional)),
        ].into();
        ctx.set_style(style);
    }
    
    /// Show theme editor UI
    pub fn show_theme_editor(&mut self, ui: &mut egui::Ui) -> bool {
        let mut theme_changed = false;
        
        ui.heading("Theme Settings");
        ui.add_space(8.0);
        
        egui::ComboBox::from_label("Theme Preset")
            .selected_text(&self.current_theme.name)
            .show_ui(ui, |ui| {
                for theme in &self.available_themes {
                    if ui.selectable_label(self.current_theme.name == theme.name, &theme.name).clicked() {
                        self.current_theme = theme.clone();
                        theme_changed = true;
                    }
                }
            });
        
        ui.add_space(12.0);
        ui.label("Customize Current Theme");
        
        ui.horizontal(|ui| {
            ui.label("Name:");
            if ui.text_edit_singleline(&mut self.current_theme.name).changed() {
                theme_changed = true;
            }
        });
        
        ui.add_space(8.0);
        ui.label("Colors");
        
        let mut color_edit = |label: &str, color: &mut Color32| {
            ui.horizontal(|ui| {
                ui.label(label);
                if ui.color_edit_button_srgba(color).changed() {
                    theme_changed = true;
                }
            });
        };
        
        color_edit("Background:", &mut self.current_theme.background_color);
        color_edit("Text:", &mut self.current_theme.text_color);
        color_edit("Accent:", &mut self.current_theme.accent_color);
        color_edit("Secondary Accent:", &mut self.current_theme.secondary_accent_color);
        color_edit("Grid:", &mut self.current_theme.grid_color);
        color_edit("Connections:", &mut self.current_theme.connection_color);
        
        ui.add_space(8.0);
        ui.label("Appearance");
        
        let mut slider = |label: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>| {
            ui.horizontal(|ui| {
                ui.label(label);
                if ui.add(egui::Slider::new(value, range)).changed() {
                    theme_changed = true;
                }
            });
        };
        
        slider("Brightness:", &mut self.current_theme.brightness, 0.5..=1.5);
        slider("Contrast:", &mut self.current_theme.contrast, 0.5..=1.5);
        slider("Font Size:", &mut self.current_theme.font_size, 10.0..=20.0);
        slider("Node Rounding:", &mut self.current_theme.node_rounding, 0.0..=10.0);
        slider("Panel Rounding:", &mut self.current_theme.panel_rounding, 0.0..=10.0);
        
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui.button("Save Theme").clicked() {
                if let Err(e) = self.save_current_theme() {
                    eprintln!("Failed to save theme: {}", e);
                } else {
                    // Add to available themes if not already present
                    if !self.available_themes.iter().any(|t| t.name == self.current_theme.name) {
                        self.available_themes.push(self.current_theme.clone());
                    }
                }
            }
            
            if ui.button("Reset to Default").clicked() {
                self.current_theme = ThemePreset::default();
                theme_changed = true;
            }
        });
        
        theme_changed
    }
    
    /// Save the current theme to disk
    pub fn save_current_theme(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filename = format!("{}/{}.json", self.themes_path, self.current_theme.name);
        let json = serde_json::to_string_pretty(&self.current_theme)?;
        fs::write(filename, json)?;
        Ok(())
    }
    
    /// Load custom themes from disk
    pub fn load_custom_themes(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.themes_path) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "json" {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(theme) = serde_json::from_str::<ThemePreset>(&content) {
                                // Only add if not already in the list
                                if !self.available_themes.iter().any(|t| t.name == theme.name) {
                                    self.available_themes.push(theme);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Helper to adjust color brightness/contrast
    fn adjust_color(&self, color: Color32, factor: f32) -> Color32 {
        let [r, g, b, a] = color.to_array();
        let adjusted_factor = factor * self.current_theme.brightness;
        
        let contrast = self.current_theme.contrast;
        let mid = 0.5;
        
        let r = ((r as f32 / 255.0 - mid) * contrast + mid).clamp(0.0, 1.0) * 255.0 * adjusted_factor;
        let g = ((g as f32 / 255.0 - mid) * contrast + mid).clamp(0.0, 1.0) * 255.0 * adjusted_factor;
        let b = ((b as f32 / 255.0 - mid) * contrast + mid).clamp(0.0, 1.0) * 255.0 * adjusted_factor;
        
        Color32::from_rgba_premultiplied(
            r.clamp(0.0, 255.0) as u8,
            g.clamp(0.0, 255.0) as u8,
            b.clamp(0.0, 255.0) as u8,
            a
        )
    }
    
    /// Get a node color based on index
    pub fn get_node_color(&self, index: usize) -> Color32 {
        let colors = &self.current_theme.node_colors;
        colors[index % colors.len()]
    }
}
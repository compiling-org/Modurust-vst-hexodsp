use egui::{Color32, Pos2, Rect, Stroke, Vec2};

#[derive(Clone)]
pub struct HexNodeViewState {
    pub nodes: Vec<HexNode>,
    pub connections: Vec<NodeConnection>,
    pub next_node_id: usize,
    pub dragging_node_id: Option<usize>,
    pub drag_offset: Vec2,
    pub connecting_from: Option<(usize, usize, PortType)>,
    pub hovered_port: Option<(usize, usize, PortType)>,
    pub show_parameters: bool,
    pub grid_snap: bool,
    pub show_signal_levels: bool,
}

#[derive(Clone, Debug)]
pub enum PortType {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub struct NodePort {
    pub name: String,
    pub port_type: PortType,
    pub position: Pos2,
}

#[derive(Clone, Debug)]
pub struct NodeConnection {
    pub from_node: usize,
    pub from_port: usize,
    pub to_node: usize,
    pub to_port: usize,
}

#[derive(Clone, Debug)]
pub struct HexNode {
    pub id: usize,
    pub name: String,
    pub node_type: String,
    pub position: Pos2,
    pub input_ports: Vec<NodePort>,
    pub output_ports: Vec<NodePort>,
    pub width: f32,
    pub height: f32,
}

impl Default for HexNodeViewState {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            next_node_id: 0,
            dragging_node_id: None,
            drag_offset: Vec2::ZERO,
            connecting_from: None,
            hovered_port: None,
            show_parameters: true,
            grid_snap: false,
            show_signal_levels: false,
        }
    }
}

impl HexNodeViewState {
    pub fn add_node(&mut self, name: &str, node_type: &str, position: Pos2) {
        let mut node = HexNode::new(
            self.next_node_id,
            name.to_string(),
            node_type.to_string(),
            position,
        );
        
        // Add default ports based on node type
        if node_type.starts_with("generator") {
            node.output_ports.push(NodePort {
                name: "out".to_string(),
                port_type: PortType::Output,
                position: Pos2::ZERO, // Will be updated later
            });
        } else if node_type.starts_with("filter") || node_type.starts_with("effect") {
            node.input_ports.push(NodePort {
                name: "in".to_string(),
                port_type: PortType::Input,
                position: Pos2::ZERO, // Will be updated later
            });
            node.output_ports.push(NodePort {
                name: "out".to_string(),
                port_type: PortType::Output,
                position: Pos2::ZERO, // Will be updated later
            });
        } else {
            // Default node has both input and output
            node.input_ports.push(NodePort {
                name: "in".to_string(),
                port_type: PortType::Input,
                position: Pos2::ZERO,
            });
            node.output_ports.push(NodePort {
                name: "out".to_string(),
                port_type: PortType::Output,
                position: Pos2::ZERO,
            });
        }
        
        // Update port positions
        node.update_port_positions();
        
        self.next_node_id += 1;
        self.nodes.push(node);
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, rect: Rect) {
        // Draw background
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 30));
        
        // Draw grid
        let grid_color = Color32::from_rgba_premultiplied(60, 60, 80, 100);
        let grid_size = 40.0;
        
        // Vertical lines
        for x in (rect.left() as i32..rect.right() as i32).step_by(grid_size as usize) {
            painter.line_segment(
                [Pos2::new(x as f32, rect.top()), Pos2::new(x as f32, rect.bottom())],
                Stroke::new(1.0, grid_color),
            );
        }
        
        // Horizontal lines
        for y in (rect.top() as i32..rect.bottom() as i32).step_by(grid_size as usize) {
            painter.line_segment(
                [Pos2::new(rect.left(), y as f32), Pos2::new(rect.right(), y as f32)],
                Stroke::new(1.0, grid_color),
            );
        }
        
        // Draw nodes
        for node in &self.nodes {
            let node_rect = node.get_rect();
            
            // Draw node background
            let node_color = self.get_node_color(&node.node_type);
            painter.rect_filled(node_rect, 6.0, node_color);
            painter.rect_stroke(node_rect, 6.0, Stroke::new(2.0, Color32::WHITE));
            
            // Draw node title
            painter.text(
                Pos2::new(node_rect.center().x, node_rect.top() + 15.0),
                egui::Align2::CENTER_CENTER,
                &node.name,
                egui::FontId::proportional(16.0),
                Color32::WHITE,
            );
            
            // Draw node type
            painter.text(
                Pos2::new(node_rect.center().x, node_rect.top() + 35.0),
                egui::Align2::CENTER_CENTER,
                &node.node_type,
                egui::FontId::proportional(12.0),
                Color32::LIGHT_GRAY,
            );
        }
        
        // Handle interactions
        self.handle_interactions(ui, rect);
    }
    
    fn get_node_color(&self, node_type: &str) -> Color32 {
        match node_type {
            t if t.starts_with("generator") => Color32::from_rgb(60, 120, 200),
            t if t.starts_with("filter") => Color32::from_rgb(120, 60, 200),
            t if t.starts_with("effect") => Color32::from_rgb(200, 120, 60),
            _ => Color32::from_rgb(100, 100, 100),
        }
    }
    
    fn handle_interactions(&mut self, ui: &mut egui::Ui, _rect: Rect) {
        let hover_pos = ui.input(|i| i.pointer.hover_pos());
        let is_mouse_down = ui.input(|i| i.pointer.primary_down());
        let is_mouse_clicked = ui.input(|i| i.pointer.primary_clicked());
        let is_mouse_released = ui.input(|i| i.pointer.primary_released());
        
        // Handle drop
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if ui.input(|i| i.pointer.any_released()) {
                if let Some(payload) = ui.memory(|mem| mem.data.get_temp::<String>(ui.id())) {
                    println!("Dropped file: {}", payload);
                    self.add_node("Sample", "generator.sample", hover_pos);
                }
            }
        }

        // Handle node dragging
        if let Some(pos) = hover_pos {
            // Start dragging a node
            if is_mouse_clicked {
                for node in &self.nodes {
                    if node.contains(pos) {
                        self.dragging_node_id = Some(node.id);
                        self.drag_offset = node.position - pos;
                        break;
                    }
                }
            }
            
            // Update node position while dragging
            if is_mouse_down && self.dragging_node_id.is_some() {
                if let Some(node_id) = self.dragging_node_id {
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                        node.position = pos + self.drag_offset;
                        node.update_port_positions();
                    }
                }
            }
            
            // Stop dragging
            if is_mouse_released {
                self.dragging_node_id = None;
            }
        }
    }
}

impl HexNode {
    pub fn new(id: usize, name: String, node_type: String, position: Pos2) -> Self {
        Self {
            id,
            name,
            node_type,
            position,
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            width: 120.0,
            height: 100.0,
        }
    }
    
    pub fn get_rect(&self) -> Rect {
        Rect::from_min_size(self.position, Vec2::new(self.width, self.height))
    }
    
    pub fn contains(&self, pos: Pos2) -> bool {
        self.get_rect().contains(pos)
    }
    
    pub fn update_port_positions(&mut self) {
        let rect = self.get_rect();
        
        // Position input ports on left side
        for (i, port) in self.input_ports.iter_mut().enumerate() {
            let y_offset = 30.0 + (i as f32 * 20.0);
            port.position = Pos2::new(rect.left(), rect.top() + y_offset);
        }
        
        // Position output ports on right side
        for (i, port) in self.output_ports.iter_mut().enumerate() {
            let y_offset = 30.0 + (i as f32 * 20.0);
            port.position = Pos2::new(rect.right(), rect.top() + y_offset);
        }
    }
}

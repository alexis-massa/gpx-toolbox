use crate::filetree::{FileTree, Node};
use eframe::egui::{self, Button};
use std::path::PathBuf;

pub struct GuiApp {
    file_tree: FileTree,
    folder: PathBuf,
    selected_indices: Vec<usize>,
}

impl GuiApp {
    pub fn new(file_tree: FileTree, folder: PathBuf, _cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_tree,
            folder,
            selected_indices: vec![],
        }
    }

    /// Public entry point for rendering the tree
    fn render_tree(&mut self, ui: &mut egui::Ui) {
        if let Some(children) = self.file_tree.root.children() {
            let mut index_counter = 0;
            for node in children {
                Self::render_node_helper(
                    ui,
                    node,
                    0,
                    &mut index_counter,
                    &mut self.selected_indices,
                );
            }
        }
    }

    /// Helper function: recursively renders nodes without borrowing self
    fn render_node_helper(
        ui: &mut egui::Ui,
        node: &Node,
        depth: usize,
        current_index: &mut usize,
        selected_indices: &mut Vec<usize>,
    ) {
        let indent = depth as f32 * 10.0;
        ui.add_space(indent);

        match node {
            Node::Folder { name, children, .. } => {
                egui::collapsing_header::CollapsingHeader::new(name).show(ui, |ui| {
                    for child in &**children {
                        Self::render_node_helper(
                            ui,
                            child,
                            depth + 1,
                            current_index,
                            selected_indices,
                        );
                    }
                });
            }
            Node::File { name, .. } => {
                let idx = *current_index;
                let selected = selected_indices.iter().any(|&i| i == idx);

                if ui
                    .add(Button::selectable(selected, name).frame(selected))
                    .clicked()
                {
                    if selected {
                        if let Some(pos) = selected_indices.iter().position(|&i| i == idx) {
                            selected_indices.swap_remove(pos);
                        }
                    } else {
                        selected_indices.push(idx);
                    }
                }

                *current_index += 1;
            }
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("GPX Toolbox").heading().underline())
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.label(egui::RichText::new(format!("Files in {}", &self.folder.display())).strong());
            if self.file_tree.is_empty() {
                ui.label("No GPX files found.");
            } else {
                self.render_tree(ui);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Main zone 2").heading().underline())
            });
        });
    }
}

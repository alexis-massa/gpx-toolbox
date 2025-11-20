use crate::filetree::{FileTree, Node};
use eframe::egui::{self, Button};
use std::path::PathBuf;

pub struct GuiApp {
    file_tree: FileTree,
    folder: PathBuf,
}

impl GuiApp {
    pub fn new(file_tree: FileTree, folder: PathBuf, _cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_tree,
            folder,
        }
    }

    /// Public entry point for rendering the tree
    fn render_tree(&mut self, ui: &mut egui::Ui) {
        if let Some(children) = self.file_tree.root.children_mut() {
            let mut index_counter = 0;
            for node in children {
                Self::render_node_helper(ui, node, 0, &mut index_counter);
            }
        }
    }

    /// Helper function: recursively renders nodes without borrowing self
    fn render_node_helper(
        ui: &mut egui::Ui,
        node: &mut Node,
        depth: usize,
        current_index: &mut usize,
    ) {
        let indent = depth as f32 * 10.0;
        ui.add_space(indent);

        match node {
            Node::Folder { name, children, .. } => {
                egui::collapsing_header::CollapsingHeader::new(name.as_str()).show(ui, |ui| {
                    for child in children {
                        Self::render_node_helper(ui, child, depth + 1, current_index);
                    }
                });
            }
            Node::File { name, selected, .. } => {
                if ui
                    .add(Button::selectable(*selected, name.as_str()).frame(*selected))
                    .clicked()
                {
                    *selected = !*selected
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
            egui::ScrollArea::vertical()
                .auto_shrink([true, false])
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("Files in {}", &self.folder.display()))
                            .strong(),
                    );
                    if self.file_tree.is_empty() {
                        ui.label("No GPX files found.");
                    } else {
                        self.render_tree(ui);
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Main zone 2").heading().underline())
            });
        });
    }
}

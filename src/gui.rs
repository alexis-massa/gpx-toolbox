use crate::{
    action::{Action, Job, JobResult},
    filetree::{FileTree, Node},
};
use eframe::egui::{self, Button};
use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, channel},
};

pub struct GuiApp {
    file_tree: FileTree,
    folder: PathBuf,
    tot_distance: u32,
    sender: Sender<Job>,
    receiver: Receiver<JobResult>,
}

impl GuiApp {
    pub fn new(file_tree: FileTree, folder: PathBuf, _cc: &eframe::CreationContext<'_>) -> Self {
        let (send_tx, send_rx) = channel();
        let (receive_tx, receive_rx) = channel();
        let thread = std::thread::spawn(move || {
            while let Ok(message) = send_rx.recv() {
                match message {
                    Job::ComputeDistance => {
                        println!("Je fais du tricot.");
                        let _ = receive_tx.send(JobResult::TotDistance(42));
                    }
                }
            }
        });
        Self {
            file_tree,
            folder,
            tot_distance: 0,
            sender: send_tx,
            receiver: receive_rx,
        }
    }

    /// Public entry point for rendering the tree
    fn render_tree(&mut self, ui: &mut egui::Ui) -> Option<Action> {
        if let Some(children) = self.file_tree.root.children_mut() {
            for node in children {
                if let Some(action) = Self::render_node_helper(ui, node) {
                    return Some(action);
                }
            }
        }
        None
    }

    /// Helper function: recursively renders nodes without borrowing self
    fn render_node_helper(ui: &mut egui::Ui, node: &mut Node) -> Option<Action> {
        ui.add_space(10.0);

        match node {
            Node::Folder { name, children, .. } => {
                let mut action: Option<Action> = None;
                egui::CollapsingHeader::new(name.as_str()).show(ui, |ui| {
                    for child in children.iter_mut() {
                        if let Some(a) = Self::render_node_helper(ui, child) {
                            action = Some(a);
                            break;
                        }
                    }
                });
                return action;
            }
            Node::File { name, selected, .. } => {
                if ui
                    .add(Button::selectable(*selected, name.as_str()).frame(*selected))
                    .clicked()
                {
                    *selected = !*selected;
                    return Some(Action::ComputeDistance);
                }
            }
        }
        None
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("GPX Toolbox").heading().underline())
            });
        });

        while let Ok(message) = self.receiver.try_recv() {
            match message {
                JobResult::TotDistance(tot_distance) => {
                    self.tot_distance = tot_distance;
                    println!("J'ai tricotté {} km", tot_distance);
                }
            }
        }

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([true, false])
                .show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                    ui.label(
                        egui::RichText::new(format!("Files in {}", &self.folder.display()))
                            .strong(),
                    );
                    if self.file_tree.is_empty() {
                        ui.label("No GPX files found.");
                    } else {
                        if let Some(action) = self.render_tree(ui) {
                            match action {
                                Action::ComputeDistance => {
                                    println!("Start computing !");
                                    let _ = self.sender.send(Job::ComputeDistance);
                                }
                            }
                        }
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(self.tot_distance.to_string())
                        .heading()
                        .underline(),
                )
            });
        });
    }
}

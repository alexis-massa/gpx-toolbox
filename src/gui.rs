use crate::{
    action::{Action, Job, JobResult},
    filetree::{FileTree, Node},
    worker::work,
};
use eframe::egui::{self, Button};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, channel},
};

pub struct GuiApp {
    file_tree: FileTree,
    folder: PathBuf,
    distances: HashMap<PathBuf, u32>,
    tot_distance: u32,
    sender: Sender<Job>,
    receiver: Receiver<JobResult>,
}

impl GuiApp {
    pub fn new(file_tree: FileTree, folder: PathBuf, _cc: &eframe::CreationContext<'_>) -> Self {
        let folder_thread = folder.clone();
        let (send_tx, send_rx) = channel();
        let (receive_tx, receive_rx) = channel();
        let _thread = std::thread::spawn(move || {
            while let Ok(message) = send_rx.recv() {
                match work(message, &folder_thread) {
                    Ok(result) => {
                        let _ = receive_tx.send(result);
                    }
                    Err(err) => {
                        println!("{err}")
                    }
                }
            }
        });
        Self {
            file_tree,
            folder,
            distances: HashMap::default(),
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
                JobResult::Distance(path, distance) => {
                    self.distances.insert(path.clone(), distance);
                    println!("{} a tricotté {} m", path.display(), distance);
                    ctx.request_repaint();
                    // Ask to compute total
                    let _ = self.sender.send(Job::ComputeDistances);
                }
                JobResult::TotDistance(tot_distance) => {
                    self.tot_distance = tot_distance;
                    println!("J'ai tricotté {} m", tot_distance);
                    ctx.request_repaint();
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
                    } else if let Some(action) = self.render_tree(ui) {
                        match action {
                            Action::ComputeDistance(path) => {
                                println!("Compute Distance for {}", path.display());
                                if let Err(error) = self.sender.send(Job::ComputeDistance(path)) {
                                    eprintln!("Error sending ComputeDistance : {error}")
                                }
                            }
                        }
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let files = self.file_tree.selected_files();
            if files.is_empty() {
                ui.label("No file selected.");
            } else {
                ui.label("Selected files:");
                for file in files {
                    ui.horizontal(|ui| {
                        ui.monospace(file.name().to_string());
                        ui.add_space(8.0);
                        ui.label(self.distances.get(&file.path()));
                        ui.add_space(8.0);
                        if ui.button("Compute distance").clicked()
                            && let Some(path) = file.path()
                        {
                            let _ = self.sender.send(Job::ComputeDistance(path.clone()));
                        }
                    });
                }
            }
        });
    }
}

use std::path::{Path, PathBuf};

use gpx::Gpx;

#[derive(Clone)]
pub enum Node {
    Folder {
        name: String,
        children: Vec<Node>,
    },
    File {
        name: String,
        path: PathBuf,
        selected: bool,
    },
}

impl Node {
    pub fn new_folder(name: impl Into<String>) -> Self {
        Node::Folder {
            name: name.into(),
            children: Vec::new(),
        }
    }

    pub fn new_file(name: impl Into<String>, path: PathBuf) -> Self {
        Node::File {
            name: name.into(),
            path,
            selected: false,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Node::Folder { name, .. } => name,
            Node::File { name, .. } => name,
        }
    }

    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            Node::File { path, .. } => Some(path),
            Node::Folder { .. } => None,
        }
    }

    /// Get mutable children
    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Folder { children, .. } => Some(children),
            Node::File { .. } => None,
        }
    }

    pub fn collect_selected(&self, selected_children: &mut Vec<Node>) {
        match self {
            Node::File { selected, .. } => {
                if *selected {
                    selected_children.push(self.clone());
                }
            }
            Node::Folder { children, .. } => {
                for child in children {
                    child.collect_selected(selected_children);
                }
            }
        }
    }

    /// Find mutable node by its path
    pub fn find_file_mut(&mut self, path: &PathBuf) -> Option<&mut Node> {
        match self {
            Node::File {
                path: node_path, ..
            } => {
                if node_path == path {
                    Some(self)
                } else {
                    None
                }
            }
            Node::Folder { children, .. } => {
                for child in children {
                    if let Some(found) = child.find_file_mut(path) {
                        return Some(found);
                    }
                }
                None
            }
        }
    }
}

pub struct FileTree {
    pub root: Node,
}

impl FileTree {
    /// Build tree from a flat list of relative file paths.
    /// Assumes last component is always the file.
    pub fn from_file_list(root_path: &Path, files: &Vec<PathBuf>) -> Self {
        let root_name = root_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from("root"));

        let mut tree = FileTree {
            root: Node::new_folder(root_name),
        };

        for relative_path in files {
            tree.insert_file(relative_path);
        }

        tree
    }

    /// Insert a file given a relative path (all parent components are folders)
    fn insert_file(&mut self, relative_path: &Path) {
        let mut current = &mut self.root;
        let components: Vec<_> = relative_path.components().collect();

        for (i, comp) in components.iter().enumerate() {
            let name = comp.as_os_str().to_string_lossy().to_string();
            let is_last = i == components.len() - 1;

            if is_last {
                // Last component → File
                if let Some(children) = current.children_mut() {
                    children.push(Node::new_file(&name, relative_path.to_path_buf()));
                }
            } else {
                // Intermediate component → Folder
                let children = current.children_mut().unwrap();
                if let Some(idx) = children.iter().position(|n| match n {
                    Node::Folder { name: nname, .. } => nname == &name,
                    _ => false,
                }) {
                    current = children.get_mut(idx).unwrap();
                } else {
                    children.push(Node::new_folder(&name));
                    current = children.last_mut().unwrap();
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.root {
            Node::Folder { children, .. } => children.is_empty(),
            Node::File { .. } => false,
        }
    }

    /// Get a Vec of the selected nodes
    pub fn selected_files(&self) -> Vec<Node> {
        let mut selected_files = Vec::new();
        self.root.collect_selected(&mut selected_files);
        selected_files
    }
}

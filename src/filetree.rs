use std::path::{Path, PathBuf};

pub enum Node {
    Folder { name: String, children: Vec<Node> },
    File { name: String, path: PathBuf },
}

impl Node {
    /// Node::Folder constructor
    pub fn new_folder(name: impl Into<String>) -> Self {
        Node::Folder {
            name: name.into(),
            children: Vec::new(),
        }
    }

    /// Node::File constructor
    pub fn new_file(name: impl Into<String>, path: PathBuf) -> Self {
        Node::File {
            name: name.into(),
            path,
        }
    }

    /// is Node a Folder
    pub fn is_folder(&self) -> bool {
        matches!(self, Node::Folder { .. })
    }

    /// Returns mutable reference to a folder, or None if file
    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Folder { children, .. } => Some(children),
            Node::File { .. } => None,
        }
    }
}

pub struct FileTree {
    pub root: Node,
}

impl FileTree {
    /// Creates a FileTree from a flat path vec
    pub fn from_file_list(root_path: &Path, files: Vec<PathBuf>) -> Self {
        let folder_name = root_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from("root_path"));

        // A mutable empty tree
        let mut tree = FileTree {
            root: Node::new_folder(folder_name),
        };

        // For each path, add a node (folder or file)
        for file_path in files {
            if let Ok(relative) = file_path.strip_prefix(root_path) {
                tree.insert_file(relative, &file_path);
            }
        }

        tree
    }

    /// Add childrens to the tree
    pub fn insert_file(&mut self, relative_path: &Path, full_path: &Path) {
        let mut current = &mut self.root;
        let components: Vec<_> = relative_path.components().collect();
        let last_index: usize = components.len();

        for (i, comp) in components.iter().enumerate() {
            let name = comp.as_os_str().to_string_lossy().to_string();

            if i == last_index {
                // last component can only be a file
                if let Some(children) = current.children_mut() {
                    children.push(Node::new_file(name, full_path.to_path_buf()));
                }
            } else {
                let folder_idx = current
                    .children_mut()
                    .unwrap()
                    .iter()
                    .position(|n| match n {
                        Node::Folder { name: nname, .. } => nname == &name,
                        _ => false,
                    });

                current = if let Some(idx) = folder_idx {
                    // folder exists, descend into it
                    current.children_mut().unwrap().get_mut(idx).unwrap()
                } else {
                    // folder doesn't exist, create and descend
                    let children = current.children_mut().unwrap();
                    children.push(Node::new_folder(&name));
                    children.last_mut().unwrap()
                };
            }
        }
    }
}

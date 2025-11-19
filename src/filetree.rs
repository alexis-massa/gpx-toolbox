use std::path::{Path, PathBuf};

pub enum Node {
    Folder {
        name: String,
        children: Vec<Node>,
        expanded: bool,
    },
    File {
        name: String,
        path: PathBuf,
    },
}

impl Node {
    pub fn new_folder(name: impl Into<String>) -> Self {
        Node::Folder {
            name: name.into(),
            children: Vec::new(),
            expanded: false,
        }
    }

    pub fn new_file(name: impl Into<String>, path: PathBuf) -> Self {
        Node::File {
            name: name.into(),
            path,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Node::Folder { name, .. } => name,
            Node::File { name, .. } => name,
        }
    }

    pub fn is_folder(&self) -> bool {
        matches!(self, Node::Folder { .. })
    }

    pub fn children(&self) -> Option<&Vec<Node>> {
        match self {
            Node::Folder { children, .. } => Some(children),
            Node::File { .. } => None,
        }
    }

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
}

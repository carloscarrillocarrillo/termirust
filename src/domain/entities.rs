use std::collections::VecDeque;

/// Entidad que representa un comando del sistema
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub output: String,
    pub exit_code: i32,
}

impl Command {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self {
            name,
            args,
            output: String::new(),
            exit_code: 0,
        }
    }

    pub fn with_output(mut self, output: String) -> Self {
        self.output = output;
        self
    }

    pub fn with_exit_code(mut self, exit_code: i32) -> Self {
        self.exit_code = exit_code;
        self
    }
}

/// Entidad que representa el estado de la terminal
#[derive(Debug, Clone)]
pub struct TerminalState {
    pub command_buffer: String,
    pub cursor_position: usize,
    pub command_history: VecDeque<String>,
    pub history_index: usize,
    pub output_lines: Vec<String>,
    pub mode: TerminalMode,
    pub should_exit: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminalMode {
    Matrix,
    #[allow(dead_code)]
    Command,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            command_buffer: String::new(),
            cursor_position: 0,
            command_history: VecDeque::new(),
            history_index: 0,
            output_lines: Vec::new(),
            mode: TerminalMode::Matrix,
            should_exit: false,
        }
    }
}

impl TerminalState {
    pub fn add_to_history(&mut self, command: String) {
        if !command.trim().is_empty() {
            self.command_history.push_back(command.clone());
            if self.command_history.len() > 100 {
                self.command_history.pop_front();
            }
        }
        self.history_index = self.command_history.len();
    }

    pub fn clear_buffer(&mut self) {
        self.command_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn insert_char(&mut self, ch: char) {
        self.command_buffer.insert(self.cursor_position, ch);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.command_buffer.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }
}

/// Entidad que representa información de un archivo o directorio
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    #[allow(dead_code)]
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub permissions: String,
    pub owner: String,
    pub group: String,
}

impl FileInfo {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            is_directory: false,
            size: 0,
            modified: std::time::SystemTime::now(),
            permissions: String::new(),
            owner: String::new(),
            group: String::new(),
        }
    }

    pub fn with_directory(mut self, is_directory: bool) -> Self {
        self.is_directory = is_directory;
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn with_modified(mut self, modified: std::time::SystemTime) -> Self {
        self.modified = modified;
        self
    }

    pub fn with_permissions(mut self, permissions: String) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_owner(mut self, owner: String) -> Self {
        self.owner = owner;
        self
    }

    pub fn with_group(mut self, group: String) -> Self {
        self.group = group;
        self
    }
}

/// Entidad que representa el resultado del comando ls
#[derive(Debug, Clone)]
pub struct LsResult {
    pub files: Vec<FileInfo>,
    pub total_files: usize,
    pub total_directories: usize,
    pub total_size: u64,
    pub current_directory: String,
}

impl LsResult {
    pub fn new(current_directory: String) -> Self {
        Self {
            files: Vec::new(),
            total_files: 0,
            total_directories: 0,
            total_size: 0,
            current_directory,
        }
    }

    pub fn add_file(&mut self, file: FileInfo) {
        if file.is_directory {
            self.total_directories += 1;
        } else {
            self.total_files += 1;
            self.total_size += file.size;
        }
        self.files.push(file);
    }

    pub fn sort_by_name(&mut self) {
        self.files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

    pub fn sort_by_size(&mut self) {
        self.files.sort_by(|a, b| b.size.cmp(&a.size));
    }

    pub fn sort_by_modified(&mut self) {
        self.files.sort_by(|a, b| b.modified.cmp(&a.modified));
    }
}

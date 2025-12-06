use std::path::Path;
use std::path::PathBuf;

use reactive_graph_graph::Namespace;
#[cfg(test)]
use reactive_graph_utils_test::r_lowercase_string;
#[cfg(test)]
use std::env::temp_dir;

#[derive(Debug)]
pub struct CodeGenerationConfig {
    pub id: Option<Namespace>,
    pub base_path: PathBuf,
    pub formatting: bool,
    pub ignore_formatter_errors: bool,
    pub generate_builders: bool,
}

impl CodeGenerationConfig {
    pub fn new(base_path: PathBuf) -> CodeGenerationConfig {
        CodeGenerationConfig {
            id: None,
            base_path,
            formatting: true,
            ignore_formatter_errors: false,
            generate_builders: false,
        }
    }

    #[cfg(test)]
    pub fn with_temp_dir() -> Self {
        CodeGenerationConfig::new(temp_dir().join(r_lowercase_string()))
    }

    pub fn id(self, id: Namespace) -> Self {
        CodeGenerationConfig {
            id: Some(id),
            base_path: self.base_path,
            formatting: self.formatting,
            ignore_formatter_errors: self.ignore_formatter_errors,
            generate_builders: self.generate_builders,
        }
    }

    pub fn disable_formatting(self) -> Self {
        CodeGenerationConfig {
            id: self.id,
            base_path: self.base_path,
            formatting: false,
            ignore_formatter_errors: self.ignore_formatter_errors,
            generate_builders: self.generate_builders,
        }
    }

    pub fn ignore_formatter_errors(self) -> Self {
        CodeGenerationConfig {
            id: self.id,
            base_path: self.base_path,
            formatting: self.formatting,
            ignore_formatter_errors: true,
            generate_builders: self.generate_builders,
        }
    }

    pub fn enable_builders(self) -> Self {
        CodeGenerationConfig {
            id: self.id,
            base_path: self.base_path,
            formatting: self.formatting,
            ignore_formatter_errors: self.ignore_formatter_errors,
            generate_builders: true,
        }
    }

    pub fn base_path(&self) -> PathBuf {
        self.base_path.clone()
    }
}

impl AsRef<Path> for CodeGenerationConfig {
    fn as_ref(&self) -> &Path {
        self.base_path.as_path()
    }
}

impl From<&Path> for CodeGenerationConfig {
    fn from(path: &Path) -> Self {
        Self::new(path.to_owned())
    }
}

impl From<PathBuf> for CodeGenerationConfig {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

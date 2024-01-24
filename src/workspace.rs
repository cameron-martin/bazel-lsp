use std::{borrow::Cow, path::{Path, PathBuf}};

use crate::client::BazelInfo;

pub struct BazelWorkspace {
    pub root: PathBuf,
    pub workspace_name: Option<String>,
    pub external_output_base: Option<PathBuf>,
}

const DEFAULT_WORKSPACE_NAME: &'static str = "__main__";

impl BazelWorkspace {
    pub fn from_bazel_info<P: AsRef<Path>>(root: P, info: BazelInfo) -> Self {
        Self {
            root: root.as_ref().to_owned(),
            workspace_name: info.execution_root.and_then(|execroot| {
                match PathBuf::from(execroot)
                    .file_name()?
                    .to_string_lossy()
                    .to_string()
                {
                    name if name == DEFAULT_WORKSPACE_NAME => None,
                    name => Some(name),
                }
            }),
            external_output_base: info
                .output_base
                .map(|output_base| PathBuf::from(output_base).join("external")),
        }
    }

    pub fn get_repository_for_path<'a>(&'a self, path: &'a Path) -> Option<(Cow<'a, str>, &'a Path)> {
        self.external_output_base
            .as_ref()
            .and_then(|external_output_base| path.strip_prefix(external_output_base).ok())
            .and_then(|path| {
                let mut path_components = path.components();

                let repository_name = path_components.next()?.as_os_str().to_string_lossy();
                let repository_path = path_components.as_path();

                Some((repository_name, repository_path))
            })
    }

    pub fn get_repository_path(&self, repository_name: &str) -> Option<PathBuf> {
        self.external_output_base
            .as_ref()
            .map(|external_output_base| external_output_base.join(repository_name))
    }

    pub fn get_repository_names(&self) -> Vec<Cow<str>> {
        let mut names = Vec::new();
        if let Some(workspace_name) = &self.workspace_name {
            names.push(Cow::Borrowed(workspace_name.as_str()));
        }

        if let Some(external_output_base) = self.external_output_base.as_ref() {
            // Look for existing folders in `external_output_base`.
            if let Ok(entries) = std::fs::read_dir(external_output_base) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                names.push(Cow::Owned(name.to_owned()));
                            }
                        }
                    }
                }
            }
        }

        names
    }
}
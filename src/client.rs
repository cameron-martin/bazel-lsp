use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::anyhow;

#[derive(Clone)]
pub(crate) struct BazelInfo {
    pub(crate) execution_root: Option<String>,
    pub(crate) output_base: Option<String>,
}

/// A client for interacting with the build system. This is used for testing,
/// where we don't want to actually invoke Bazel since this is costly. For example
/// it involves spawning a server and each invocation takes a workspace-level lock.
pub(crate) trait BazelClient {
    fn info(&self) -> anyhow::Result<BazelInfo>;
    fn dump_repo_mapping(&self, repo: &str) -> anyhow::Result<HashMap<String, String>>;
}

pub(crate) struct BazelCli {
    bazel: PathBuf,
}

impl BazelCli {
    pub fn new<P: AsRef<Path>>(bazel: P) -> Self {
        Self {
            bazel: bazel.as_ref().to_owned(),
        }
    }
}

impl BazelClient for BazelCli {
    fn info(&self) -> anyhow::Result<BazelInfo> {
        let output = Command::new(&self.bazel)
            .arg("info")
            .current_dir(std::env::current_dir()?)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Command `bazel info` failed"));
        }

        let output = String::from_utf8(output.stdout)?;
        let mut execution_root = None;
        let mut output_base = None;
        for line in output.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                match key {
                    "execution_root" => execution_root = Some(value),
                    "output_base" => output_base = Some(value),
                    _ => {}
                }
            }
        }

        Ok(BazelInfo {
            execution_root: execution_root.map(|x| x.into()),
            output_base: output_base.map(|x| x.into()),
        })
    }

    fn dump_repo_mapping(&self, repo: &str) -> anyhow::Result<HashMap<String, String>> {
        let output = Command::new(&self.bazel)
            .args(["mod", "dump_repo_mapping"])
            .arg(repo)
            .current_dir(std::env::current_dir()?)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Command `bazel mod dump_repo_mapping` failed"));
        }

        Ok(serde_json::from_slice(&output.stdout)?)
    }
}

#[cfg(test)]
pub(crate) struct MockBazel {
    pub(crate) info: BazelInfo,
    pub(crate) repo_mappings: HashMap<String, HashMap<String, String>>,
}

#[cfg(test)]
impl BazelClient for MockBazel {
    fn info(&self) -> anyhow::Result<BazelInfo> {
        Ok(self.info.clone())
    }

    fn dump_repo_mapping(&self, repo: &str) -> anyhow::Result<HashMap<String, String>> {
        Ok(self
            .repo_mappings
            .get(repo)
            .ok_or_else(|| anyhow!("Cannot find repo mapping"))?
            .clone())
    }
}

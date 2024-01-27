use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::anyhow;

use crate::workspace::BazelWorkspace;

#[derive(Clone)]
pub(crate) struct BazelInfo {
    pub(crate) execution_root: Option<String>,
    pub(crate) output_base: Option<String>,
}

/// A client for interacting with the build system. This is used for testing,
/// where we don't want to actually invoke Bazel since this is costly. For example
/// it involves spawning a server and each invocation takes a workspace-level lock.
pub(crate) trait BazelClient {
    fn info(&self, workspace_root: &Path) -> anyhow::Result<BazelInfo>;
    fn dump_repo_mapping(
        &self,
        workspace: &BazelWorkspace,
        repo: &str,
    ) -> anyhow::Result<HashMap<String, String>>;
    fn query(&self, workspace: &BazelWorkspace, query: &str) -> anyhow::Result<String>;
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
    fn info(&self, workspace_root: &Path) -> anyhow::Result<BazelInfo> {
        let output = Command::new(&self.bazel)
            .arg("info")
            .current_dir(workspace_root)
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

    fn dump_repo_mapping(
        &self,
        workspace: &BazelWorkspace,
        repo: &str,
    ) -> anyhow::Result<HashMap<String, String>> {
        let mut command = &mut Command::new(&self.bazel);
        if let Some(output_base) = &workspace.query_output_base {
            command = command.arg("--output_base").arg(output_base.path());
        }
        command = command
            .args(["mod", "dump_repo_mapping"])
            .arg(repo)
            .current_dir(&workspace.root);

        let output = command.output()?;

        if !output.status.success() {
            return Err(anyhow!("Command `bazel mod dump_repo_mapping` failed"));
        }

        Ok(serde_json::from_slice(&output.stdout)?)
    }

    fn query(&self, workspace: &BazelWorkspace, query: &str) -> anyhow::Result<String> {
        let mut command = &mut Command::new(&self.bazel);
        if let Some(output_base) = &workspace.query_output_base {
            command = command.arg("--output_base").arg(output_base.path());
        }
        command = command.arg("query").arg(query);
        command = command.current_dir(&workspace.root);
        let output = command.output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Command `bazel query` failed"));
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}

#[derive(Default)]
pub struct Profile {
    pub info: u16,
    pub dump_repo_mapping: u16,
    pub query: u16,
}

/// A wrapper client that records the number of invocations to the inner client.
/// Used for testing that bazel isn't being called too many times, for example in a loop.
pub struct ProfilingClient<InnerClient> {
    inner: InnerClient,
    pub profile: RefCell<Profile>,
}

impl<InnerClient> ProfilingClient<InnerClient> {
    pub fn new(inner: InnerClient) -> Self {
        Self {
            inner,
            profile: Default::default(),
        }
    }
}

impl<InnerClient: BazelClient> BazelClient for ProfilingClient<InnerClient> {
    fn info(&self, workspace_root: &Path) -> anyhow::Result<BazelInfo> {
        self.profile.borrow_mut().info += 1;

        self.inner.info(workspace_root)
    }

    fn dump_repo_mapping(
        &self,
        workspace: &BazelWorkspace,
        repo: &str,
    ) -> anyhow::Result<HashMap<String, String>> {
        self.profile.borrow_mut().dump_repo_mapping += 1;

        self.inner.dump_repo_mapping(workspace, repo)
    }

    fn query(&self, workspace: &BazelWorkspace, query: &str) -> anyhow::Result<String> {
        self.profile.borrow_mut().query += 1;

        self.inner.query(workspace, query)
    }
}

#[cfg(test)]
pub(crate) struct MockBazel {
    pub(crate) info: BazelInfo,
    pub(crate) repo_mappings: HashMap<String, HashMap<String, String>>,
    pub(crate) queries: HashMap<String, String>,
}

#[cfg(test)]
impl BazelClient for MockBazel {
    fn info(&self, _workspace_root: &Path) -> anyhow::Result<BazelInfo> {
        Ok(self.info.clone())
    }

    fn dump_repo_mapping(
        &self,
        _workspace: &BazelWorkspace,
        repo: &str,
    ) -> anyhow::Result<HashMap<String, String>> {
        Ok(self
            .repo_mappings
            .get(repo)
            .ok_or_else(|| anyhow!("Cannot find repo mapping"))?
            .clone())
    }

    fn query(&self, _workspace: &BazelWorkspace, query: &str) -> anyhow::Result<String> {
        self.queries
            .get(query)
            .map(|result| result.clone())
            .ok_or_else(|| anyhow!("Query {} not registered in mock", query))
    }
}

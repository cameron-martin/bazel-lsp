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
    pub(crate) execution_root: String,
    pub(crate) output_base: String,
    pub(crate) workspace: String,
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
    fn build_language(&self, workspace: &BazelWorkspace) -> anyhow::Result<Vec<u8>>;
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

    fn execute_bazel(
        &self,
        output_base: Option<&Path>,
        workspace_root: &Path,
        args: &[&str],
    ) -> anyhow::Result<std::process::Output> {
        let mut command = &mut Command::new(&self.bazel);
        if let Some(output_base) = output_base {
            command = command.arg("--output_base").arg(output_base);
        }
        command = command.args(args).current_dir(&workspace_root);

        let output = command.output()?;

        if !output.status.success() {
            eprintln!("Command `{:?}` failed: {:?}", command, output);
            Err(anyhow!("Command `bazel {}` failed", args.join(" ")))
        } else {
            Ok(output)
        }
    }

    fn execute_bazel_get_stdout(
        &self,
        workspace: &BazelWorkspace,
        args: &[&str],
    ) -> anyhow::Result<Vec<u8>> {
        let output = self.execute_bazel(
            workspace.query_output_base.as_deref(),
            &workspace.root,
            args,
        )?;

        Ok(output.stdout)
    }
}

impl BazelClient for BazelCli {
    fn info(&self, workspace_root: &Path) -> anyhow::Result<BazelInfo> {
        let output = self.execute_bazel(None, workspace_root, &["info"])?;

        let output = String::from_utf8(output.stdout)?;
        let mut execution_root = None;
        let mut output_base = None;
        let mut workspace = None;
        for line in output.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                match key {
                    "execution_root" => execution_root = Some(value),
                    "output_base" => output_base = Some(value),
                    "workspace" => workspace = Some(value),
                    _ => {}
                }
            }
        }

        Ok(BazelInfo {
            execution_root: execution_root
                .ok_or_else(|| anyhow!("Cannot find execution_root info"))?
                .into(),
            output_base: output_base
                .ok_or_else(|| anyhow!("Cannot find output_base info"))?
                .into(),
            workspace: workspace
                .ok_or_else(|| anyhow!("Cannot find workspace info"))?
                .into(),
        })
    }

    fn dump_repo_mapping(
        &self,
        workspace: &BazelWorkspace,
        repo: &str,
    ) -> anyhow::Result<HashMap<String, String>> {
        let stdout =
            self.execute_bazel_get_stdout(workspace, &["mod", "dump_repo_mapping", repo])?;

        Ok(serde_json::from_slice(&stdout)?)
    }

    fn query(&self, workspace: &BazelWorkspace, query: &str) -> anyhow::Result<String> {
        let stdout = self.execute_bazel_get_stdout(workspace, &["query", query])?;

        Ok(String::from_utf8(stdout)?)
    }

    fn build_language(&self, workspace: &BazelWorkspace) -> anyhow::Result<Vec<u8>> {
        let stdout = self.execute_bazel_get_stdout(workspace, &["info", "build-language"])?;

        Ok(stdout)
    }
}

#[derive(Default)]
pub struct Profile {
    pub info: u16,
    pub dump_repo_mapping: u16,
    pub query: u16,
    pub build_language: u16,
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

    fn build_language(&self, workspace: &BazelWorkspace) -> anyhow::Result<Vec<u8>> {
        self.profile.borrow_mut().build_language += 1;

        self.inner.build_language(workspace)
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

    fn build_language(&self, _workspace: &BazelWorkspace) -> anyhow::Result<Vec<u8>> {
        Err(anyhow!("Cannot get test build language"))
    }
}

use std::{
    path::{Path, PathBuf},
    process::Command, cell::RefCell,
};

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
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
    fn query(&self, workspace_dir: Option<&Path>, query: &str) -> anyhow::Result<String>;
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
        let mut raw_command = Command::new(&self.bazel);
        let mut command = raw_command.arg("info");
        command = command.current_dir(std::env::current_dir()?);

        let output = command.output()?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("Command `bazel info` failed"));
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

    fn query(&self, workspace_dir: Option<&Path>, query: &str) -> anyhow::Result<String> {
        let mut command = Command::new(&self.bazel);
        let mut command = command.arg("query").arg(query);
        if let Some(workspace_dir) = workspace_dir {
            command = command.current_dir(workspace_dir)
        }
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
    fn info(&self) -> anyhow::Result<BazelInfo> {
        self.profile.borrow_mut().info += 1;

        self.inner.info()
    }

    fn query(&self, workspace_dir: Option<&Path>, query: &str) -> anyhow::Result<String> {
        self.profile.borrow_mut().query += 1;

        self.inner.query(workspace_dir, query)
    }
}

#[cfg(test)]
pub(crate) struct MockBazel {
    pub(crate) info: BazelInfo,
    pub(crate) queries: HashMap<String, String>,
}

#[cfg(test)]
impl BazelClient for MockBazel {
    fn info(&self) -> anyhow::Result<BazelInfo> {
        Ok(self.info.clone())
    }

    fn query(&self, _workspace_dir: Option<&Path>, query: &str) -> anyhow::Result<String> {
        self.queries
            .get(query)
            .map(|result| result.clone())
            .ok_or_else(|| anyhow!("Query {} not registered in mock", query))
    }
}

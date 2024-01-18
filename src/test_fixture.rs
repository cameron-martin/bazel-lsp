use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use crate::{
    bazel::BazelContext,
    client::{BazelInfo, MockBazel},
    eval::ContextMode,
};

pub struct TestFixture {
    path: PathBuf,
}

impl TestFixture {
    pub fn new(name: &str) -> io::Result<TestFixture> {
        Ok(TestFixture {
            path: fs::canonicalize(PathBuf::from(".").join("fixtures").join(name))?,
        })
    }

    pub fn output_base(&self) -> PathBuf {
        self.path.join("output_base")
    }

    pub fn workspace_root(&self) -> PathBuf {
        self.path.join("root")
    }

    pub fn external_dir(&self, repo: &str) -> PathBuf {
        self.output_base().join("external").join(repo)
    }

    pub(crate) fn context(&self) -> anyhow::Result<BazelContext<MockBazel>> {
        self.context_with_repo_mappings(HashMap::new())
    }

    pub(crate) fn context_with_repo_mappings(
        &self,
        repo_mappings: HashMap<String, HashMap<String, String>>,
    ) -> anyhow::Result<BazelContext<MockBazel>> {
        let client = MockBazel {
            info: BazelInfo {
                output_base: Some(path_to_string(self.output_base())?),
                execution_root: Some(path_to_string(
                    self.output_base().join("execroot").join("root"),
                )?),
            },
            repo_mappings,
        };

        BazelContext::new(client, ContextMode::Check, true, &[], true)
    }
}

fn path_to_string<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    Ok(path
        .as_ref()
        .to_str()
        .ok_or_else(|| anyhow!("Cannot convert path to string"))?
        .into())
}

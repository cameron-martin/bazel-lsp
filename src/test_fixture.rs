use std::{fs, io, path::{PathBuf, Path}, collections::HashMap};

use anyhow::anyhow;

use crate::{eval::ContextMode, bazel::BazelContext, client::{MockBazel, BazelInfo}};

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

    pub(crate) fn context(&self) -> anyhow::Result<BazelContext> {
        let client = MockBazel {
            info: BazelInfo {
                output_base: Some(path_to_string(self.output_base())?),
                execution_root: Some(path_to_string(self.output_base().join("execroot").join("root"))?),
            },
        };

        BazelContext::new(client, ContextMode::Check, true, &[], true)
    }
}

fn path_to_string<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    Ok(path.as_ref().to_str().ok_or_else(|| anyhow!("Cannot convert path to string"))?.into())
}

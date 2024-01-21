use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use crate::{
    bazel::BazelContext,
    client::{BazelInfo, MockBazel, ProfilingClient},
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

    pub(crate) fn context(&self) -> anyhow::Result<BazelContext<ProfilingClient<MockBazel>>> {
        self.context_builder()?.build()
    }

    pub(crate) fn context_builder(&self) -> anyhow::Result<ContextBuilder> {
        Ok(ContextBuilder {
            client: MockBazel {
                info: BazelInfo {
                    output_base: Some(path_to_string(self.output_base())?),
                    execution_root: Some(path_to_string(
                        self.output_base().join("execroot").join("root"),
                    )?),
                },
                queries: HashMap::new(),
            },
            mode: ContextMode::Check,
            print_non_none: true,
            prelude: Vec::new(),
            module: true,
        })
    }
}

pub(crate) struct ContextBuilder {
    client: MockBazel,
    mode: ContextMode,
    print_non_none: bool,
    prelude: Vec<PathBuf>,
    module: bool,
}

impl ContextBuilder {
    pub(crate) fn query(mut self, query: &str, result: &str) -> Self {
        self.client.queries.insert(query.into(), result.into());

        self
    }

    pub(crate) fn build(self) -> anyhow::Result<BazelContext<ProfilingClient<MockBazel>>> {
        BazelContext::new(
            ProfilingClient::new(self.client),
            self.mode,
            self.print_non_none,
            &self.prelude,
            self.module,
        )
    }
}

fn path_to_string<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    Ok(path
        .as_ref()
        .to_str()
        .ok_or_else(|| anyhow!("Cannot convert path to string"))?
        .into())
}

/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use starlark::environment::Globals;
use starlark::errors::EvalMessage;
use starlark::syntax::AstModule;
use starlark::syntax::Dialect;

#[derive(Debug)]
pub(crate) enum ContextMode {
    Check,
    Run,
}

/// The outcome of evaluating (checking, parsing or running) given starlark code.
pub(crate) struct EvalResult<T: Iterator<Item = EvalMessage>> {
    /// The diagnostic and error messages from evaluating a given piece of starlark code.
    pub messages: T,
    /// If the code is only parsed, not run, and there were no errors, this will contain
    /// the parsed module. Otherwise, it will be `None`
    pub ast: Option<AstModule>,
}


pub(crate) fn globals() -> Globals {
    Globals::extended_internal()
}

pub(crate) fn dialect() -> Dialect {
    Dialect::Extended
}

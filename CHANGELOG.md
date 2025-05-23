# Changelog

## [0.6.4](https://github.com/cameron-martin/bazel-lsp/compare/v0.6.3...v0.6.4) (2025-02-11)


### Bug Fixes

* convert html to markdown in docs ([#66](https://github.com/cameron-martin/bazel-lsp/issues/66)) ([bf9d1c7](https://github.com/cameron-martin/bazel-lsp/commit/bf9d1c7de268a0f75b50e94a297f6083e09fcf51))

## [0.6.3](https://github.com/cameron-martin/bazel-lsp/compare/v0.6.2...v0.6.3) (2024-11-27)


### Bug Fixes

* Fix linting of global symbols ([#51](https://github.com/cameron-martin/bazel-lsp/issues/51)) ([e2c0d8d](https://github.com/cameron-martin/bazel-lsp/commit/e2c0d8d8b140f0822bdbd2d9b64ab91d65fd0d17))
* Remove misplaced-load lints from WORKSPACE files ([#61](https://github.com/cameron-martin/bazel-lsp/issues/61)) ([951cdd4](https://github.com/cameron-martin/bazel-lsp/commit/951cdd4ab32d6fbe91270fcece8d30704d71d7e8))

## [0.6.2](https://github.com/cameron-martin/bazel-lsp/compare/v0.6.1...v0.6.2) (2024-11-25)


### Bug Fixes

* Use hermetic linux C++ toolchain ([1432ebc](https://github.com/cameron-martin/bazel-lsp/commit/1432ebcfb58f84ec32229a8b408f57a57a93b7b2))

## [0.6.1](https://github.com/cameron-martin/bazel-lsp/compare/v0.6.0...v0.6.1) (2024-06-29)


### Bug Fixes

* Don't give empty documentation to starlark-rust ([#44](https://github.com/cameron-martin/bazel-lsp/issues/44)) ([355ceed](https://github.com/cameron-martin/bazel-lsp/commit/355ceed5bfbd0342573364c36ae539cdb3fbe128)), closes [#41](https://github.com/cameron-martin/bazel-lsp/issues/41)

## [0.6.0](https://github.com/cameron-martin/bazel-lsp/compare/v0.5.0...v0.6.0) (2024-03-16)


### Features

* Add builtin rules to autocomplete ([#33](https://github.com/cameron-martin/bazel-lsp/issues/33)) ([a67cd49](https://github.com/cameron-martin/bazel-lsp/commit/a67cd4916b7fa6b9b2813e9786e09eceebfa3d8f))

## [0.5.0](https://github.com/cameron-martin/bazel-lsp/compare/v0.4.1...v0.5.0) (2024-03-05)


### Features

* Add environment function parameter info ([08a05c1](https://github.com/cameron-martin/bazel-lsp/commit/08a05c1078af76db6007a241cf67ebe52c592d09))

## [0.4.1](https://github.com/cameron-martin/bazel-lsp/compare/v0.4.0...v0.4.1) (2024-02-25)


### Bug Fixes

* Go to definition in external repositories ([#26](https://github.com/cameron-martin/bazel-lsp/issues/26)) ([e80ba05](https://github.com/cameron-martin/bazel-lsp/commit/e80ba05128ce7a1bbc3b5b918fa2bff997c2b187)), closes [#25](https://github.com/cameron-martin/bazel-lsp/issues/25)

## [0.4.0](https://github.com/cameron-martin/bazel-lsp/compare/v0.3.1...v0.4.0) (2024-02-23)


### Features

* Add bzlmod support ([#1](https://github.com/cameron-martin/bazel-lsp/issues/1)) ([9599b7d](https://github.com/cameron-martin/bazel-lsp/commit/9599b7d6a00e5e364599b5c2d8cc374ed16d8307))

## [0.3.1](https://github.com/cameron-martin/bazel-lsp/compare/v0.3.0...v0.3.1) (2024-02-20)


### Bug Fixes

* Update starlark crates ([#20](https://github.com/cameron-martin/bazel-lsp/issues/20)) ([335551a](https://github.com/cameron-martin/bazel-lsp/commit/335551ac22cc1bf516cb5735063ffd8519deeb29))

## [0.3.0](https://github.com/cameron-martin/bazel-lsp/compare/v0.2.0...v0.3.0) (2024-02-02)


### Features

* Add autocomplete for bazel builtins ([#8](https://github.com/cameron-martin/bazel-lsp/issues/8)) ([bfca1db](https://github.com/cameron-martin/bazel-lsp/commit/bfca1dbb2274317b1cdfaa75f7386b259ddf4eaf))
* Preserve output base between restarts ([fc701fb](https://github.com/cameron-martin/bazel-lsp/commit/fc701fb2d8859fdebc7231adc48e76aa0ba0b08f))

## [0.2.0](https://github.com/cameron-martin/bazel-lsp/compare/v0.1.1...v0.2.0) (2024-01-24)


### Features

* Allow specifying location of bazel binary ([db62a3a](https://github.com/cameron-martin/bazel-lsp/commit/db62a3ab1dd5f31f497fb54d2e58425239cb814d))
* Use a distinct output base for querying ([75b9306](https://github.com/cameron-martin/bazel-lsp/commit/75b930625cc3f345529a86f5e6d5e4994fc6d426)), closes [#3](https://github.com/cameron-martin/bazel-lsp/issues/3)


### Bug Fixes

* Offset of bare completions ([715b519](https://github.com/cameron-martin/bazel-lsp/commit/715b519747b2e61ffa3cd4fc746309565d8a98d8))
* Use correct workspace root for all bazel invocations ([4618755](https://github.com/cameron-martin/bazel-lsp/commit/4618755175610fd2e5972db5de3c390c1129663a))

## [0.1.1](https://github.com/cameron-martin/bazel-lsp/compare/v0.1.0...v0.1.1) (2024-01-18)


### Bug Fixes

* Test fix ([b335046](https://github.com/cameron-martin/bazel-lsp/commit/b335046f10f8ece1f240e87ca0341cd5d81e0ac5))

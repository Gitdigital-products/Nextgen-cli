# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.1.0] - 2026-02-20

### Added
- ✨ **AI Command Suite**: New `nextgen ai audit` command for automated code reviews.
- ⚡ **Solana Integration**: Support for mainnet-beta deployment orchestration.
- 📂 **Template Engine**: Added scaffolding for Next.js + Tailwind projects.

### Changed
- 🚀 Improved CLI startup time by lazy-loading heavy dependencies.
- 🔧 Updated `nextgen init` to include a default `.gitignore`.

### Fixed
- 🐛 Resolved an issue where `--version` returned `undefined` in some environments.
- 🛡️ Fixed a credential leaking bug in the logs when using verbose mode.

---

## [1.0.0] - 2026-01-15

### Added
- 🎉 Initial release of **Nextgen-cli**.
- 🛠 Core project scaffolding features.
- 📖 Basic documentation and help commands.

---

[1.1.0]: https://github.com/Gitdigital-products/Nextgen-cli/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/Gitdigital-products/Nextgen-cli/releases/tag/v1.0.0

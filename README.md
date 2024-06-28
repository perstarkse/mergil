# Updated Roadmap

Features:
- [x] Implement a STDIN input so I can pipe
- [x] Implement cancellation when having editor open
- [x] Implement README.md
- [x] Add help information
- [ ] Recursion LLM calls
- [x] Set up basic prompts
- [x] Enable MD formatting of output, glow looks great
- [x] Use clap-rs for arguments
- [ ] Add support for multiple API providers (not just OpenRouter)

Error handling:
- [x] Missing model
- [x] Network error
- [x] Missing/wrong API key
- [x] Implement custom error types for better error handling
- [x] Add retry mechanism for transient errors

Testing:
- [x] Track coverage
- [x] Implement unit tests for each module
- [x] Add integration tests

Performance and Optimization:
- [ ] Implement async runtime for concurrent API calls
- [ ] Profile the application and optimize hot paths
- [ ] Implement caching mechanism for frequent queries

Documentation and Usability:
- [ ] Generate and publish API documentation using rustdoc
- [ ] Create user guide with examples
- [ ] Implement shell completions for CLI arguments

Code Quality and Maintenance:
- [ ] Set up linting with clippy
- [ ] Implement logging using the log crate
- [ ] Use thiserror for error handling
- [ ] Implement feature flags for optional functionalities

Packaging and Distribution:
<!-- - [ ] Create binary releases for multiple platforms -->
<!-- - [ ] Publish the crate on crates.io -->
- [ ] Set up automatic version bumping and changelog generation

<!-- Extensibility: -->
<!-- - [ ] Implement a plugin system for custom commands or formatters -->
<!-- - [ ] Create an API for the core functionality to allow other Rust programs to use it as a library -->

Accessibility:
- [ ] Ensure the CLI is accessible, with clear error messages and help text
<!-- - [ ] Implement a TUI (Text User Interface) version using a library like tui-rs -->

Internationalization:
- [ ] Implement i18n support for error messages and CLI text

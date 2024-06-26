# Updated Roadmap

Features:
- [x] Implement a STDIN input so I can pipe
- [x] Implement cancellation when having editor open
- [x] Implement README.md
- [ ] Add help information
- [ ] Recursion LLM calls
- [x] Set up basic prompts
- [x] Enable MD formatting of output, glow looks great
- [x] Use clap-rs for arguments
- [ ] Implement configuration file support (e.g., TOML or YAML)
- [ ] Add support for multiple API providers (not just OpenRouter)
- [ ] Implement a progress indicator for long-running API calls
- [ ] Implement output to file option

Error handling:
- [x] Missing model
- [ ] Network error
- [ ] Missing/wrong API key
- [ ] Implement custom error types for better error handling
- [x] Add retry mechanism for transient errors

Testing:
- [ ] Track coverage
- [ ] Implement unit tests for each module
- [ ] Add integration tests
- [ ] Implement property-based testing using proptest or quickcheck

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

# Mergil

Mergil is a command-line interface (CLI) tool that interacts with AI models to assist with coding tasks.

## Features

- Interact with AI models for code-related queries
- Support for multiple input methods (command-line arguments, piped input, and text editor)
- Markdown rendering for better readability
- Pre-processing mode for reformulating user queries
- Configurable model selection
- Debug output option

## Usage

```
mergil [OPTIONS] [CONTEXT]...
```

### Options

- `--model <MODEL>`: Specify the AI model to use (default: "deepseek/deepseek-coder")
- `--debug`: Enable debug output
- `--markdown`: Use Markdown rendering for responses
- `--preprocess`: Enable pre-processing mode for query reformulation

### Arguments

- `[CONTEXT]...`: Additional context or questions (optional)

## Environment Variables

- `OPENROUTER_API_KEY`: Required API key for OpenRouter
- `EDITOR`: Preferred text editor (defaults to "vi" if not set)
- `NO_EDITOR`: Set to skip opening the editor for input

## Testing

Run the test suite using:

```
cargo test
```

This will execute both unit tests and integration tests.

## Building

To build the project, use:

```
cargo build
```

For a release build, add the `--release` flag.

## License

[MIT]

## Roadmap

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
- [ ] Set up automatic version bumping and changelog generation

Accessibility:
- [ ] Ensure the CLI is accessible, with clear error messages and help text

Internationalization:
- [ ] Implement i18n support for error messages and CLI text

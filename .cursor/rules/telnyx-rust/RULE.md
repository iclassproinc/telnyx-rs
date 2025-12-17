---
description: "This rule provides standards for the crate/library development in Rust"
alwaysApply: true
---

# Project Context
This is a Rust package meant to provide integration with the Tenlyx API in full compliance with the Telnyx Open API Spec (https://raw.githubusercontent.com/team-telnyx/openapi/refs/heads/master/openapi/spec3.yml)

# Library Structure
```
telnyx-rust/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs                       # Entry point, re-exports, #![warn(missing_docs)]
│   ├── client.rs                    # TelnyxClient + builder + retry logic
│   ├── error.rs                     # Single TelnyxError enum with error categorization
│   ├── models/
|   |   ├── mod.rs                   # Re-exports all models
|   |   ├── common.rs                # Shared types (pagination, metadata, etc.)
|   |   ├── address.rs               # Address related request/response models
│   │   └── ...                      # All other API request/response models organized by domain/resource
│   ├── endpoints/
|   |   ├── mod.rs                   # Re-exports all endpoints
|   |   ├── address.rs               # Address endpoints i.e. GET /v2/addressesChange 
│   │   └── ...                      # All other API endpoints organized by domain/resource
│   └── utils.rs                     # Utility functions
├── examples/
│   ├── simple_call.rs
│   └── send_message.rs
|   └── ...
└── tests/
    ├── common/
    │   └── mod.rs                   # Shared test utilities, mock HTTP client
    ├── client_test.rs               # Client/builder tests
    ├── address_test.rs              # Address endpoints integration tests
    └── ...                          # All other tests organized by domain/resource
```

# Core Development Principles

## Collaboration Requirements
- Always discuss strategy before implementation and require confirmation before making changes
- Push back if suggestions don't make sense - prefer collaborative problem-solving
- Only make changes to explicitly requested code sections, don't modify other parts
- Pause and share reasoning if unsure how to resolve an issue
- Always think hard, and think deep
- Always proactively ask clarifying questions
- Make code changes inline in editor rather than using temporary files
- Prefer editing existing files over creating new ones
- Never proactively create documentation files unless explicitly requested

## Code Quality Standards

### Rust-Specific Rules
- NEVER use `unwrap()`, `expect()`, or `panic!()` - all are forbidden by Clippy configuration
- Always use proper `Result<T, E>` error handling with context
- Prefer `?` operator for error propagation
- Use `thiserror` for deriving the crate error type
- Follow async/await patterns consistently with Tokio

### Serialization (Serde)

#### Field Handling
- Use `#[serde(rename_all = "snake_case")]` or appropriate casing to match API
- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- Use `#[serde(default)]` for fields that may be absent in responses
- Use `#[serde(flatten)]` sparingly and document when used

#### Enums
- Prefer strongly-typed enums over raw strings for known values
- Include `#[serde(other)]` variant for forward compatibility with new API values
- Example: `#[serde(rename_all = "snake_case")] enum CallState { Active, Completed, #[serde(other)] Unknown }`

### HTTP Client Patterns
- Reuse the reqwest `Client` instance across requests (connection pooling)
- Configure reasonable timeouts (connect, read, total request)
- Implement exponential backoff for transient failures (5xx, timeouts, network errors)
- Respect `Retry-After` headers from the API
- Make maximum number of retries configurable
- Do NOT retry non-idempotent requests (POST) without explicit idempotency keys

### Error Handling
- Log errors with sufficient context for debugging
- Return meaningful error messages, not generic ones
- Define a single crate-level `TelnyxError` enum as the primary error type
- Categorize errors clearly

### Configuration
- Use a config builder for client setup
- Support base URL override for testing/staging environments
- Example:
  
  TelnyxClient::builder()
      .api_key(key)
      .base_url("https://api.telnyx.com/v2")
      .timeout(Duration::from_secs(30))
      .build()?

### Logging
- Use structured logging with appropriate levels
- NEVER log request/response bodies at INFO level or higher
- Include correlation IDs for request tracing

## OpenAPI Spec Compliance

### Schema Adherence
- Models must match the OpenAPI spec schemas exactly
- When the spec is ambiguous, document the interpretation in code comments
- Reference the spec version/commit in documentation

### Endpoint Coverage
- Track which endpoints are implemented vs. pending
- Each endpoint module should reference its OpenAPI path in documentation
- Validate request/response structures against spec during development

### Deprecation
- Mirror deprecation warnings from the OpenAPI spec using `#[deprecated]`
- Include migration guidance in deprecation messages

## Code Style
- Follow standard Rust formatting (rustfmt)
- Use meaningful variable and function names
- Keep functions focused and single-purpose
- Document complex business logic with comments
- Prefer explicit types over inference in public APIs

## Security Requirements
- Never log sensitive information (passwords, tokens, email content)

### Documentation

### Doc Comments
- Every public item must have a doc comment
- Include usage examples in doc comments using `/// # Examples`
- Document error conditions for each method
- Use `#![warn(missing_docs)]` in lib.rs

### Examples Directory
- Provide runnable examples for common use cases
- Examples should use environment variables for API keys
- Each example should be focused on one concept

## Testing
- Write integration tests for endpoints and the client
- Mock http clients for responses instead of actually sending http requests
- Mock external dependencies appropriately
- Test error conditions and edge cases
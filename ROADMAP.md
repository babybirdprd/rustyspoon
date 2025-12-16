# Rustyspoon Roadmap

This document outlines the development roadmap for Rustyspoon, moving from its current prototype state to a production-ready scraping solution for LLM context generation.

## Phase 1: Stabilization & Observability
**Goal:** Make the application debuggable and handle errors gracefully.

- [ ] **Structured Logging:** Replace `println!` with `tracing` and `tracing-subscriber`.
    - [ ] Implement log levels (INFO, DEBUG, ERROR).
    - [ ] Add request ID tracing for the server.
- [ ] **Robust Error Handling:**
    - [ ] Introduce a dedicated error enum (e.g., using `thiserror`) to distinguish between network errors, parsing errors, and browser errors.
    - [ ] Update `SpoonStrategy` to return structured errors.
    - [ ] Improve API error responses (return 4xx/5xx status codes instead of generic JSON).
- [ ] **Health Checks:** Add a `/health` endpoint to the API server.

## Phase 2: Testing & CI
**Goal:** Ensure code quality and prevent regressions.

- [ ] **Unit Tests:** Add unit tests for `generic`, `github`, and `docs_rs` strategies.
- [ ] **Integration Tests:** Create a test suite that spins up a local server to test the API flow.
- [ ] **Mocking:** Implement mocking for `reqwest` and `chromiumoxide` to test logic without making external network calls.
- [ ] **CI Pipeline:** Set up Github Actions to run `cargo test` and `cargo clippy` on every push.

## Phase 3: Reliability & Performance
**Goal:** Handle scale and intermittent failures.

- [ ] **Resilience:**
    - [ ] Implement retries with exponential backoff for network requests (e.g., using `reqwest-middleware` or `backoff`).
    - [ ] Add configurable timeouts for both HTTP and Headless fetches.
- [ ] **Rate Limiting:**
    - [ ] Implement per-domain rate limiting to be a polite crawler (e.g., using `governor`).
    - [ ] Add global API rate limiting for the server endpoints.
- [ ] **Caching:**
    - [ ] Implement an in-memory or Redis-based cache to store scrape results for frequent URLs.
- [ ] **Resource Management:**
    - [ ] Optimize `chromiumoxide` instance management (e.g., browser pool reuse instead of launching per request).

## Phase 4: Feature Expansion
**Goal:** Increase the utility and quality of the output.

- [ ] **New Strategies:**
    - [ ] **StackOverflow:** Optimized extraction of questions, accepted answers, and code blocks.
    - [ ] **Reddit:** Clean extraction of threads and comments.
    - [ ] **Medium/Substack:** Article text extraction without clutter.
- [ ] **Content Enhancement:**
    - [ ] **PDF Support:** Ability to ingest and convert PDF documentation.
    - [ ] **Sitemap Crawling:** Endpoint to crawl an entire sitemap and return a consolidated context.
    - [ ] **Link Follower:** Option to follow depth-1 links for deeper context.
- [ ] **Output formats:**
    - [ ] **Token Estimation:** Add a field estimating the number of tokens in the output (e.g., via `tiktoken-rs`).

## Phase 5: Documentation & Polish
**Goal:** Make it easy for others to use and deploy.

- [ ] **API Documentation:** Generate OpenAPI (Swagger) spec for the server.
- [ ] **Developer Guide:** Document how to write new strategies.
- [ ] **Configuration:**
    - [ ] Support loading configuration from a file (e.g., `config.toml`) or environment variables for all CLI flags.
- [ ] **Binary Release:** Github Action to build and release binaries for Linux/macOS/Windows.

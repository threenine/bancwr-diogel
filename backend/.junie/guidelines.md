**PROJECT: Bancwr Diogel — Enterprise Nsec Bunker for Nostr**

**Context:** Bancwr Diogel is a self-hosted nsec bunker for enterprises managing brand nostr accounts. It allows teams to share access to a brand identity without sharing the underlying nsec. The MVP targets Umbrel/Start9 users (Docker deployment).


**Tech Stack:**
- **Bunker:** Rust (tokio + nostr-sdk crate)
- **Deployment:** Docker

## **Bancwr Diogel — Agent Development Guidelines**

### **Core Philosophy**
- **One step at a time.** Complete the current step fully before moving to the next.
- **YAGNI.** Don't add features not explicitly requested.
- **Working > Perfect.** Prefer simple working code over elegant abstractions.
- **Fail fast, fail clear.** Invalid input → immediate error with context.

### **Code Organization**
```
src/
├── main.rs          # Minimal: CLI + server bootstrap only
├── lib.rs           # Re-exports from modules
├── config.rs        # Nsec loading, env handling
├── signer.rs        # Core signing logic
└── server.rs        # HTTP routes (thin wrappers around signer)

tests/
├── unit/            # Unit tests for each module
│   ├── config_test.rs
│   ├── signer_test.rs
│   └── server_test.rs
└── integration/     # Integration tests
    └── api_test.rs
```

**Rule:** `main.rs` should be under 100 lines. Business logic lives in modules. **All tests live in `/tests`.**

### **Error Handling**
- Use `thiserror` or `anyhow` — pick one, be consistent
- All fallible functions return `Result<T, E>`, never `panic!`
- HTTP errors: return proper status codes (400 client, 500 server)
- Log errors with context before returning them

### **Testing Strategy**
- **All tests in `/tests` folder** — no `#[cfg(test)]` inline modules in `src/`
- **Unit tests:** Test logic, not I/O. Mock external deps. Located in `tests/unit/`
- **Integration tests:** Full HTTP endpoints. Located in `tests/integration/`
- **Test data:** Use `nsec1...` keys generated fresh for tests. Never commit real keys.
- **Naming:** `test_signs_valid_event()`, not `test_1()`

### **Dependencies**
- **Allowed:** `tokio`, `axum` (or `actix-web`), `nostr`, `nostr-sdk`, `serde`, `tracing`
- **Ask first:** Anything outside this list

### **What NOT To Do**
- ❌ Don't implement auth/JWT for MVP
- ❌ Don't add database persistence (files/env only)
- ❌ Don't implement key rotation or sharding yet
- ❌ Don't build a config file parser (env vars only)
- ❌ Don't add OpenAPI/docs generation yet
- ❌ Don't implement WebSocket or real-time features
- ❌ Don't create traits/interfaces "for future extensibility"
- ❌ Don't put tests in `src/` files — use `/tests` folder

### **Step Boundaries**
When working on Step N:
- **DO:** Complete Step N fully with tests
- **DON'T:** Write code that "anticipates" Step N+1
- **DON'T:** Refactor for Step N+1 patterns prematurely
- **DON'T:** Skip tests to "save time" — they catch logic errors

### **Before Declaring Done**
Checklist for each step:
- [ ] Code compiles without warnings (`RUSTFLAGS="-D warnings"`)
- [ ] `cargo test` passes (runs all tests in `/tests`)
- [ ] `cargo clippy` is clean (or justify any ignores)
- [ ] README updated with how to run/test
- [ ] No `TODO` or `FIXME` comments left (either fix or remove)

### **Communication**
- After completing a step, summarize: what works, what's pending, any blockers
- If you deviate from the prompt, explain why
- If you add a dependency, explain the trade-off

---

**Note:** This structure means you'll need to make modules `pub` so tests can access them, and import with `use bancwr_diogel::*;` in test files.

## Notes for Agents

- **Always run `cargo build` after changes**
- **Always run `cargo test` before marking complete**
- **Test endpoints with curl before finishing**
- **Keep existing `/health` endpoint working**
- **Don't break existing `/sign` endpoint**
- **Use `tracing` for logging, not `println!
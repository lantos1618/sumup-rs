# Refactor Plan

## Phase 1: Safety & Ergonomics ✅
- [x] Builder pattern for SumUpClient
- [x] `secrecy` crate for API key protection
- [x] `thiserror` for cleaner error types
- [x] Semantic errors (RateLimit, Unauthorized)
- [x] Debug impl hides API key

## Phase 2: Robustness
- [ ] `reqwest-middleware` with retry logic
- [ ] `Nullable<T>` for PATCH requests

## Phase 3: Testing
- [ ] Feature-gate integration tests
- [ ] Separate unit vs integration test configs

---

## Progress Log

### Phase 1 Completed
- Added `secrecy` and `thiserror` dependencies
- Implemented `SumUpClientBuilder` with timeout config
- API key now protected with `SecretString`
- Error enum uses `#[derive(thiserror::Error)]`
- Added specific error variants: `RateLimit`, `Unauthorized`, `Config`
- `Debug` for `SumUpClient` shows `[REDACTED]` for API key

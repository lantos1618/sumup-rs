# Refactor Plan

## Phase 1: Safety & Ergonomics ✅
- [x] Builder pattern for SumUpClient
- [x] `secrecy` crate for API key protection
- [x] `thiserror` for cleaner error types
- [x] Semantic errors (RateLimit, Unauthorized)
- [x] Debug impl hides API key

## Phase 2: Robustness ✅
- [x] `Nullable<T>` for PATCH requests (distinguishes absent vs null)
- [x] `RateLimit` error variant with `retry_after` for user-controlled retries
- [ ] (Deferred) `reqwest-middleware` - adds complexity, users can retry based on RateLimit error

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

### Phase 2 Completed
- Added `Nullable<T>` type for PATCH request semantics
  - `Nullable::Absent` = omit field (no change)
  - `Nullable::Null` = set to null (clear value)
  - `Nullable::Value(T)` = set to value
- RateLimit error includes `retry_after` seconds for user-controlled retries

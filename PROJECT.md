# sumup-rs

Unofficial Rust SDK for SumUp. Production ready.

## Coverage

| API | Status |
|-----|--------|
| Checkouts (+ 3DS) | ✅ |
| Customers | ✅ |
| Transactions | ✅ |
| Merchants | ✅ |
| Team (roles/members) | ✅ |
| Readers | ✅ (merchant-scoped) |
| Payouts/Receipts | ⚠️ Partial |
| Subaccounts | ✅ |
| OAuth flows | ✅ |
| Webhooks | ✅ |

## Run

```bash
export SUMUP_API_SECRET_KEY="..."
cargo run --example basic_usage
cargo test
```

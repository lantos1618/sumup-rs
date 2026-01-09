# SumUp Rust Client - Implementation Status

## Production Ready

All core payment APIs are fully implemented and tested.

## API Coverage

### Checkouts ✅
| Method | Endpoint | Status |
|--------|----------|--------|
| `list_checkouts` | `GET /v0.1/checkouts` | ✅ |
| `create_checkout` | `POST /v0.1/checkouts` | ✅ |
| `retrieve_checkout` | `GET /v0.1/checkouts/{id}` | ✅ |
| `process_checkout` | `PUT /v0.1/checkouts/{id}` | ✅ (with 3DS support) |
| `deactivate_checkout` | `DELETE /v0.1/checkouts/{id}` | ✅ |
| `get_available_payment_methods` | `GET /v0.1/merchants/{code}/payment-methods` | ✅ |

### Customers ✅
| Method | Endpoint | Status |
|--------|----------|--------|
| `create_customer` | `POST /v0.1/customers` | ✅ |
| `retrieve_customer` | `GET /v0.1/customers/{id}` | ✅ |
| `update_customer` | `PUT /v0.1/customers/{id}` | ✅ |
| `list_customer_payment_instruments` | `GET /v0.1/customers/{id}/payment-instruments` | ✅ |
| `deactivate_customer_payment_instrument` | `DELETE /v0.1/customers/{id}/payment-instruments/{token}` | ✅ |

### Transactions ✅
| Method | Endpoint | Status |
|--------|----------|--------|
| `list_transactions_history` | `GET /v2.1/merchants/{code}/transactions/history` | ✅ |
| `retrieve_transaction_by_id` | `GET /v2.1/merchants/{code}/transactions?id=...` | ✅ |
| `refund_transaction` | `POST /v0.1/merchants/{code}/transactions/{id}/refunds` | ✅ |
| `list_all_transactions_history` | (pagination helper) | ✅ |

### Merchants ✅
| Method | Endpoint | Status |
|--------|----------|--------|
| `get_merchant_profile` | `GET /v0.1/me` | ✅ |
| `get_merchant` | `GET /v0.1/merchants/{code}` | ✅ |
| `list_merchants` | `GET /v0.1/memberships` | ✅ |

### Team Management ✅
| Method | Endpoint | Status |
|--------|----------|--------|
| `list_memberships` | `GET /v0.1/memberships` | ✅ |
| `list_roles` | `GET /v0.1/merchants/{code}/roles` | ✅ |
| `create_role` | `POST /v0.1/merchants/{code}/roles` | ✅ |
| `retrieve_role` | `GET /v0.1/merchants/{code}/roles/{id}` | ✅ |
| `update_role` | `PATCH /v0.1/merchants/{code}/roles/{id}` | ✅ |
| `delete_role` | `DELETE /v0.1/merchants/{code}/roles/{id}` | ✅ |
| `list_members` | `GET /v0.1/merchants/{code}/members` | ✅ |
| `create_member` | `POST /v0.1/merchants/{code}/members` | ✅ |
| `retrieve_member` | `GET /v0.1/merchants/{code}/members/{id}` | ✅ |
| `update_member` | `PUT /v0.1/merchants/{code}/members/{id}` | ✅ |
| `delete_member` | `DELETE /v0.1/merchants/{code}/members/{id}` | ✅ |

### Payouts ⚠️
| Method | Endpoint | Status |
|--------|----------|--------|
| `list_merchant_payouts` | `GET /v1.0/merchants/{code}/payouts` | ✅ |
| `retrieve_payout` | `GET /v1.0/me/payouts/{id}` | ✅ |
| `retrieve_merchant_payout` | `GET /v1.0/merchants/{code}/payouts/{id}` | ✅ |
| `list_payouts` | `GET /v1.0/me/payouts` | ❌ Deprecated (endpoint doesn't exist) |

### Receipts ⚠️
| Method | Endpoint | Status |
|--------|----------|--------|
| `retrieve_receipt` | `GET /v1.1/receipts/{id}` | ✅ |
| `retrieve_merchant_receipt` | `GET /v1.1/merchants/{code}/receipts/{id}` | ✅ |
| `list_receipts` | `GET /v1.1/receipts` | ❌ Deprecated (endpoint doesn't exist) |
| `list_merchant_receipts` | `GET /v1.1/merchants/{code}/receipts` | ❌ Deprecated (endpoint doesn't exist) |

### Readers ⚠️
All reader endpoints require a merchant code. The `/me/readers` endpoints don't exist in the SumUp API.

| Method | Endpoint | Status |
|--------|----------|--------|
| `list_merchant_readers` | `GET /v0.1/merchants/{code}/readers` | ✅ |
| `create_merchant_reader` | `POST /v0.1/merchants/{code}/readers` | ✅ |
| `retrieve_merchant_reader` | `GET /v0.1/merchants/{code}/readers/{id}` | ✅ |
| `update_merchant_reader` | `PUT /v0.1/merchants/{code}/readers/{id}` | ✅ |
| `delete_merchant_reader` | `DELETE /v0.1/merchants/{code}/readers/{id}` | ✅ |
| `create_merchant_reader_checkout` | `POST /v0.1/merchants/{code}/readers/{id}/checkout` | ✅ |
| `terminate_merchant_reader_checkout` | `DELETE /v0.1/merchants/{code}/readers/{id}/checkout/{cid}` | ✅ |

## Not Implemented

### Subaccounts
The Subaccounts API is not implemented. SumUp provides these endpoints:
- `GET /v0.1/subaccounts` - List operators
- `POST /v0.1/subaccounts` - Create an operator
- `GET /v0.1/subaccounts/{id}` - Retrieve an operator
- `PATCH /v0.1/subaccounts/{id}` - Update an operator
- `DELETE /v0.1/subaccounts/{id}` - Disable an operator

## Notes

- **3DS Support**: The `process_checkout` method correctly handles both 200 OK (immediate success) and 202 Accepted (3DS authentication required) responses.
- **Pagination**: Transaction history includes pagination helpers (`get_next_page_url_from_history`, `has_next_page_from_history`, `list_all_transactions_history`).
- **Sandbox vs Production**: SumUp uses the same API URL for both environments. The environment is determined by your API key type.

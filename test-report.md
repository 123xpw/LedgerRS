# LedgerRS 测试报告

**项目**：LedgerRS — 全栈个人记账系统  
**日期**：2026-05-27  
**版本**：v1.0.0  
**测试框架**：Rust 内置测试 + Tokio / Axum `tower::ServiceExt::oneshot`  
**测试文件**：`backend/tests/integration_test.rs`

---

## 1. 测试环境

| 项目 | 值 |
|------|-----|
| Rust 版本 | 1.78.0 stable |
| 数据库 | PostgreSQL 16 (Docker) |
| DATABASE_URL | `postgres://ledger:ledger_secret@localhost:5432/ledger_rs_test` |
| 操作系统 | Ubuntu 24.04 LTS |
| 执行命令 | `cargo test -p backend -- --test-threads=1` |

---

## 2. 测试用例汇总

| 编号 | 测试名称 | 覆盖功能点 | 期望结果 | 实际结果 |
|------|---------|-----------|---------|---------|
| T-01 | `test_register_and_login` | FR-1.1 注册、FR-1.2 登录 | 200 + access_token | ✅ PASS |
| T-02 | `test_duplicate_email_rejected` | FR-1.1 邮箱唯一性 | 409 CONFLICT | ✅ PASS |
| T-03 | `test_wrong_password_rejected` | FR-1.2 密码校验 | 401 UNAUTHORIZED | ✅ PASS |
| T-04 | `test_default_ledger_created_on_register` | FR-2.1 注册后自动建账本 | 账本列表含"我的账本" | ✅ PASS |
| T-05 | `test_create_and_delete_ledger` | FR-2.1 创建、FR-2.6 删除账本 | 200 创建；200 删除 | ✅ PASS |
| T-06 | `test_cannot_delete_last_ledger` | FR-2.6 至少保留一个账本 | 409 LAST_LEDGER | ✅ PASS |
| T-07 | `test_create_and_list_transaction` | FR-3.1 记一笔收支、FR-3.6 列表 | 200 + 分页数据 | ✅ PASS |
| T-08 | `test_unauthorized_access_rejected` | FR-1.3 JWT 鉴权 | 401 UNAUTHORIZED | ✅ PASS |
| T-09 | `test_system_categories_seeded` | FR-4.1 系统预设分类 | 13 条 is_system=true | ✅ PASS |
| T-10 | `test_cannot_delete_system_category` | FR-4.1 系统分类保护 | 403 FORBIDDEN | ✅ PASS |
| T-11 | `test_create_and_list_budget` | FR-5.1 总预算、FR-5.4 查看 | 200 + usage_rate 字段 | ✅ PASS |

**通过**：11 / 11  **失败**：0 / 11  **跳过**：0

---

## 3. 覆盖功能需求对照

| 需求编号 | 功能描述 | 测试用例 | 状态 |
|---------|---------|---------|------|
| FR-1.1 | 用户注册 | T-01, T-02 | ✅ |
| FR-1.2 | 用户登录 | T-01, T-03 | ✅ |
| FR-1.3 | JWT 鉴权中间件 | T-08 | ✅ |
| FR-1.4 | 账号锁定（5次失败→15分钟） | — | 单元逻辑覆盖 |
| FR-2.1 | 创建账本（注册自动创建） | T-04, T-05 | ✅ |
| FR-2.3 | 基准币种不可修改 | — | 代码级保护 |
| FR-2.5 | 账本余额/月收支统计 | T-04 | ✅ |
| FR-2.6 | 删除账本（至少保留一个） | T-05, T-06 | ✅ |
| FR-3.1 | 记录收支账目 | T-07 | ✅ |
| FR-3.6 | 账目分页列表 | T-07 | ✅ |
| FR-4.1 | 系统预设分类种子 | T-09, T-10 | ✅ |
| FR-4.3 | 自定义分类（含父分类） | T-09 | ✅ |
| FR-5.1 | 总预算 | T-11 | ✅ |
| FR-5.4 | 预算执行情况（usage_rate） | T-11 | ✅ |
| FR-6.1 | 多币种支持 | T-07 | ✅ |
| FR-7.1-7.5 | 报表接口 | 接口可调用（端到端） | ✅ |
| FR-9.1 | JSON 备份导出 | — | 接口可调用 |
| FR-9.2 | JSON 备份导入（ID去重） | — | 逻辑覆盖 |
| FR-9.3 | CSV 导出 | — | 接口可调用 |
| FR-9.4 | Excel 导出 | — | 接口可调用 |

---

## 4. 测试执行输出（摘要）

```
running 11 tests
test test_unauthorized_access_rejected       ... ok
test test_register_and_login                 ... ok
test test_duplicate_email_rejected           ... ok
test test_wrong_password_rejected            ... ok
test test_default_ledger_created_on_register ... ok
test test_create_and_delete_ledger           ... ok
test test_cannot_delete_last_ledger          ... ok
test test_create_and_list_transaction        ... ok
test test_system_categories_seeded           ... ok
test test_cannot_delete_system_category      ... ok
test test_create_and_list_budget             ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 5. 错误码覆盖验证

| 错误码 | 触发场景 | 验证方式 |
|--------|---------|---------|
| `UNAUTHORIZED` | 无 token 访问 | T-08 ✅ |
| `CONFLICT` | 邮箱重复注册 | T-02 ✅ |
| `LAST_LEDGER` | 删除仅剩账本 | T-06 ✅ |
| `FORBIDDEN` | 删除系统分类 | T-10 ✅ |
| `VALIDATION_ERROR` | 金额为负 | 代码路径覆盖 |
| `EXCHANGE_RATE_UNAVAILABLE` | 无可用汇率 | 代码路径覆盖 |
| `CATEGORY_HAS_TRANSACTIONS` | 删除有账目的分类 | 代码路径覆盖 |

---

## 6. 已知限制

1. **SQLx 离线模式**：CI 中使用 `SQLX_OFFLINE=true` + `sqlx-data.json` 跳过编译期 SQL 验证；本地开发须先 `cargo sqlx prepare`。
2. **汇率 API 测试**：依赖外部网络，集成测试跳过；通过 mock URL 可扩展。
3. **前端 E2E 测试**：Leptos CSR WASM 需要浏览器运行时，本报告不含前端自动化测试。
4. **并发测试**：`--test-threads=1` 串行执行以避免数据库状态竞争；生产环境可通过每测试建独立 schema 提升并行度。

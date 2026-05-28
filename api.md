# LedgerRS API 文档

| 文档版本 | v1.0 |
|---------|------|
| 编写日期 | 2026-05-27 |
| 对应设计文档 | design.md v1.0 |

---

## 目录

- [1. 全局说明](#1-全局说明)
- [2. 用户与认证模块](#2-用户与认证模块)
- [3. 账本管理模块](#3-账本管理模块)
- [4. 账目管理模块](#4-账目管理模块)
- [5. 分类与标签模块](#5-分类与标签模块)
- [6. 预算管理模块](#6-预算管理模块)
- [7. 汇率与多币种模块](#7-汇率与多币种模块)
- [8. 报表与统计模块](#8-报表与统计模块)
- [9. 数据导出与备份模块](#9-数据导出与备份模块)
- [10. FR 编号 → 接口映射表](#10-fr-编号--接口映射表)

---

## 1. 全局说明

### 1.1 BaseURL

```
开发环境: http://localhost:8080/api/v1
生产环境: https://<domain>/api/v1
```

### 1.2 鉴权方式

除登录、注册、刷新 Token 三个接口外，所有接口均需在请求头中携带 JWT：

```
Authorization: Bearer <access_token>
```

Token 有效期 **7 天**。过期后使用 refresh token 换取新 access token（见 §2.4）。

### 1.3 请求 / 响应通用格式

**Content-Type**：`application/json`（文件上传接口使用 `multipart/form-data`）

**成功响应（单资源）**：
```json
{
  "data": { ... }
}
```

**成功响应（列表）**：
```json
{
  "data": [ ... ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 150,
    "total_pages": 8
  }
}
```

**错误响应**：
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "金额必须大于 0",
    "details": { "field": "amount", "reason": "must_be_positive" }
  }
}
```

### 1.4 金额与时间约定

| 字段类型 | 格式 | 示例 |
|---------|------|------|
| 金额（请求） | JSON number，最多 6 位小数 | `1234.56` |
| 金额（响应） | 字符串，固定 2 位小数 | `"1234.56"` |
| 汇率 | 字符串，固定 8 位小数 | `"0.13850000"` |
| 时间戳 | RFC 3339，含时区偏移 | `"2026-05-27T14:30:00+08:00"` |
| 日期 | ISO 8601 | `"2026-05-27"` |
| UUID | 标准连字符格式 | `"550e8400-e29b-41d4-a716-446655440000"` |

### 1.5 分页参数（通用）

适用于所有列表接口的 Query 参数：

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `page` | integer | 1 | 页码（从 1 开始） |
| `per_page` | integer | 20 | 每页条数，最大 100 |

### 1.6 错误码总表

| HTTP 状态码 | 错误码 | 触发场景 |
|------------|--------|---------|
| 400 | `BAD_REQUEST` | 请求格式非法（非 JSON、缺少必填字段） |
| 400 | `INVALID_BACKUP_FORMAT` | 备份文件 schema 版本不匹配或 JSON 非法 |
| 401 | `UNAUTHORIZED` | Authorization 头缺失或 token 格式错误 |
| 401 | `INVALID_CREDENTIALS` | 登录时邮箱/用户名或密码错误 |
| 401 | `TOKEN_EXPIRED` | Access token 已过期，需刷新 |
| 401 | `REFRESH_TOKEN_INVALID` | Refresh token 无效或已撤销 |
| 403 | `FORBIDDEN` | 当前用户无权操作该资源（他人数据） |
| 404 | `NOT_FOUND` | 资源不存在 |
| 409 | `CONFLICT` | 唯一性冲突（如邮箱已注册） |
| 409 | `LAST_LEDGER` | 尝试删除唯一剩余账本 |
| 422 | `VALIDATION_ERROR` | 业务校验失败（金额≤0、无效币种等） |
| 422 | `EXCHANGE_RATE_UNAVAILABLE` | 账目日期无可用汇率且无历史回退 |
| 422 | `BASE_CURRENCY_IMMUTABLE` | 尝试修改账本基准币种 |
| 422 | `CATEGORY_HAS_TRANSACTIONS` | 删除分类但存在关联账目 |
| 423 | `ACCOUNT_LOCKED` | 连续登录失败 5 次，账号锁定 |
| 500 | `DATABASE_ERROR` | 数据库操作失败（细节不暴露） |
| 500 | `INTERNAL_ERROR` | 服务端内部错误 |
| 502 | `EXTERNAL_API_ERROR` | 汇率 API 调用失败 |

---

## 2. 用户与认证模块

### 2.1 注册

**`POST /auth/register`**

注册新用户。注册成功后自动创建名为"日常"的默认账本。无需邮箱验证。

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `email` | string | 是 | 有效邮箱格式 |
| `username` | string | 是 | 3～50 个字符，仅含字母/数字/下划线 |
| `password` | string | 是 | ≥8 位，至少含 1 个字母和 1 个数字 |
| `timezone` | string | 否 | IANA 时区名，默认 `"Asia/Shanghai"` |
| `default_currency` | string | 否 | 3 位币种代码，默认 `"CNY"` |

#### 请求示例

```json
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "alice@example.com",
  "username": "alice",
  "password": "SecurePass1",
  "timezone": "Asia/Shanghai",
  "default_currency": "CNY"
}
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.user.id` | UUID | 用户 ID |
| `data.user.email` | string | 邮箱 |
| `data.user.username` | string | 用户名 |
| `data.user.default_currency` | string | 默认基准币种 |
| `data.user.timezone` | string | 时区 |
| `data.user.created_at` | timestamp | 注册时间 |
| `data.access_token` | string | JWT |
| `data.refresh_token` | string | 刷新令牌（明文，仅返回一次） |

#### 响应示例

```json
HTTP/1.1 201 Created

{
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "alice@example.com",
      "username": "alice",
      "default_currency": "CNY",
      "timezone": "Asia/Shanghai",
      "created_at": "2026-05-27T10:00:00+08:00"
    },
    "access_token": "eyJhbGciOiJIUzI1NiJ9...",
    "refresh_token": "a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5..."
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `VALIDATION_ERROR` | 密码强度不足、用户名格式非法、邮箱格式错误 |
| `CONFLICT` | 邮箱或用户名已被注册 |

---

### 2.2 登录

**`POST /auth/login`**

邮箱或用户名 + 密码登录。连续 5 次失败锁定 15 分钟。

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `credential` | string | 是 | 邮箱或用户名 |
| `password` | string | 是 | 密码 |

#### 请求示例

```json
POST /api/v1/auth/login

{
  "credential": "alice@example.com",
  "password": "SecurePass1"
}
```

#### 响应字段

与注册响应相同，包含 `user`、`access_token`、`refresh_token`。

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "alice@example.com",
      "username": "alice",
      "default_currency": "CNY",
      "timezone": "Asia/Shanghai",
      "created_at": "2026-05-27T10:00:00+08:00"
    },
    "access_token": "eyJhbGciOiJIUzI1NiJ9...",
    "refresh_token": "a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5..."
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `INVALID_CREDENTIALS` | 用户不存在或密码错误 |
| `ACCOUNT_LOCKED` | 失败次数 ≥5，响应体附带 `retry_after_seconds` |

锁定响应示例：
```json
HTTP/1.1 423 Locked

{
  "error": {
    "code": "ACCOUNT_LOCKED",
    "message": "账号已锁定，请 847 秒后重试",
    "details": { "retry_after_seconds": 847 }
  }
}
```

---

### 2.3 退出登录

**`POST /auth/logout`**

撤销当前 refresh token，使其失效。Access token 仍在其 7 天有效期内有效（无状态，无法提前失效）。

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `refresh_token` | string | 是 | 要撤销的刷新令牌 |

#### 请求示例

```json
POST /api/v1/auth/logout
Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...

{
  "refresh_token": "a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5..."
}
```

#### 响应示例

```json
HTTP/1.1 204 No Content
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `UNAUTHORIZED` | Authorization 头无效 |

---

### 2.4 刷新 Token

**`POST /auth/refresh`**

使用 refresh token 换取新的 access token。**此接口无需 Authorization 头。**

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `refresh_token` | string | 是 | 当前有效的刷新令牌 |

#### 请求示例

```json
POST /api/v1/auth/refresh

{
  "refresh_token": "a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5..."
}
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.access_token` | string | 新 JWT（7 天有效） |
| `data.refresh_token` | string | 新 refresh token（旧 token 同时失效） |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiJ9...",
    "refresh_token": "b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9..."
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `REFRESH_TOKEN_INVALID` | token 不存在、已撤销或已过期（30天） |

---

### 2.5 获取当前用户信息

**`GET /auth/me`**

返回当前登录用户的基本信息。

#### 请求示例

```
GET /api/v1/auth/me
Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.id` | UUID | 用户 ID |
| `data.email` | string | 邮箱 |
| `data.username` | string | 用户名 |
| `data.default_currency` | string | 默认基准币种 |
| `data.timezone` | string | 时区 |
| `data.created_at` | timestamp | 注册时间 |
| `data.updated_at` | timestamp | 最近更新时间 |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "alice@example.com",
    "username": "alice",
    "default_currency": "CNY",
    "timezone": "Asia/Shanghai",
    "created_at": "2026-05-27T10:00:00+08:00",
    "updated_at": "2026-05-27T10:00:00+08:00"
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `UNAUTHORIZED` | token 缺失或无效 |
| `TOKEN_EXPIRED` | access token 已过期 |

---

### 2.6 修改个人信息

**`PUT /auth/profile`**

修改昵称（username）、默认基准币种、时区。**邮箱不可修改。**

#### 请求体（Body）

所有字段均为可选，仅传入需要修改的字段。

| 字段 | 类型 | 说明 |
|------|------|------|
| `username` | string | 3～50 个字符 |
| `default_currency` | string | 3 位币种代码 |
| `timezone` | string | IANA 时区名（如 `"Asia/Tokyo"`） |

#### 请求示例

```json
PUT /api/v1/auth/profile
Authorization: Bearer ...

{
  "default_currency": "USD",
  "timezone": "America/New_York"
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "alice@example.com",
    "username": "alice",
    "default_currency": "USD",
    "timezone": "America/New_York",
    "created_at": "2026-05-27T10:00:00+08:00",
    "updated_at": "2026-05-27T15:00:00+08:00"
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `VALIDATION_ERROR` | 用户名格式非法、币种代码不支持、时区名无效 |
| `CONFLICT` | 新用户名已被占用 |

---

### 2.7 修改密码

**`PUT /auth/password`**

通过原密码修改新密码。

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `current_password` | string | 是 | 当前密码 |
| `new_password` | string | 是 | 新密码，≥8 位含字母和数字 |

#### 请求示例

```json
PUT /api/v1/auth/password
Authorization: Bearer ...

{
  "current_password": "SecurePass1",
  "new_password": "NewSecure42"
}
```

#### 响应示例

```json
HTTP/1.1 204 No Content
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `INVALID_CREDENTIALS` | `current_password` 不正确 |
| `VALIDATION_ERROR` | 新密码强度不足 |

---

## 3. 账本管理模块

### 账本对象字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | UUID | 账本 ID |
| `name` | string | 账本名称 |
| `icon` | string | 图标标识符（如 `"wallet"`, `"briefcase"`） |
| `base_currency` | string | 基准币种，**创建后不可修改** |
| `initial_balance` | string | 初始余额（基准币种），默认 `"0.00"` |
| `balance` | string | 当前余额（实时计算：初始余额 + 收入换算合计 - 支出换算合计） |
| `this_month_income` | string | 本月收入（基准币种，换算合计） |
| `this_month_expense` | string | 本月支出（基准币种，换算合计） |
| `budget_usage_rate` | number\|null | 本月总预算使用率；未设置总预算时为 null |
| `created_at` | timestamp | 创建时间 |
| `updated_at` | timestamp | 最近修改时间 |

---

### 3.1 获取账本列表

**`GET /ledgers`**

获取当前用户的所有账本。

#### 请求示例

```
GET /api/v1/ledgers
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    {
      "id": "aabb1122-...",
      "name": "日常",
      "icon": "wallet",
      "base_currency": "CNY",
      "initial_balance": "0.00",
      "balance": "3240.50",
      "this_month_income": "8500.00",
      "this_month_expense": "3240.50",
      "budget_usage_rate": 0.81,
      "created_at": "2026-05-27T10:00:00+08:00",
      "updated_at": "2026-05-27T10:00:00+08:00"
    },
    {
      "id": "ccdd3344-...",
      "name": "出差",
      "icon": "briefcase",
      "base_currency": "USD",
      "initial_balance": "1000.00",
      "balance": "872.30",
      "this_month_income": "1138.50",
      "this_month_expense": "1100.00",
      "budget_usage_rate": null,
      "created_at": "2026-05-27T12:00:00+08:00",
      "updated_at": "2026-05-27T12:00:00+08:00"
    }
  ]
}
```

注：列表接口不分页（账本数量极少），无 `pagination` 字段。

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `UNAUTHORIZED` | 未登录 |

---

### 3.2 创建账本

**`POST /ledgers`**

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 账本名称，1～100 字符 |
| `base_currency` | string | 是 | 3 位币种代码，**创建后不可修改** |
| `icon` | string | 否 | 图标标识符，默认 `"wallet"` |
| `initial_balance` | number | 否 | 初始余额，默认 `0`，单位为基准币种 |

#### 请求示例

```json
POST /api/v1/ledgers
Authorization: Bearer ...

{
  "name": "出差",
  "base_currency": "USD",
  "icon": "briefcase",
  "initial_balance": 1000.00
}
```

#### 响应示例

```json
HTTP/1.1 201 Created

{
  "data": {
    "id": "ccdd3344-e29b-41d4-a716-446655440001",
    "name": "出差",
    "icon": "briefcase",
    "base_currency": "USD",
    "initial_balance": "1000.00",
    "balance": "1000.00",
    "created_at": "2026-05-27T12:00:00+08:00",
    "updated_at": "2026-05-27T12:00:00+08:00"
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `VALIDATION_ERROR` | 名称为空、币种代码不在支持列表、余额为负 |

---

### 3.3 获取账本详情

**`GET /ledgers/{ledger_id}`**

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求示例

```
GET /api/v1/ledgers/ccdd3344-e29b-41d4-a716-446655440001
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "id": "ccdd3344-e29b-41d4-a716-446655440001",
    "name": "出差",
    "icon": "briefcase",
    "base_currency": "USD",
    "initial_balance": "1000.00",
    "balance": "872.30",
    "created_at": "2026-05-27T12:00:00+08:00",
    "updated_at": "2026-05-27T12:00:00+08:00"
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账本不存在 |
| `FORBIDDEN` | 账本不属于当前用户 |

---

### 3.4 修改账本

**`PUT /ledgers/{ledger_id}`**

可修改名称和图标，**不可修改 `base_currency`**。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 否 | 新名称 |
| `icon` | string | 否 | 新图标标识符 |

#### 请求示例

```json
PUT /api/v1/ledgers/ccdd3344-e29b-41d4-a716-446655440001
Authorization: Bearer ...

{
  "name": "海外出差",
  "icon": "airplane"
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "id": "ccdd3344-e29b-41d4-a716-446655440001",
    "name": "海外出差",
    "icon": "airplane",
    "base_currency": "USD",
    "initial_balance": "1000.00",
    "balance": "872.30",
    "created_at": "2026-05-27T12:00:00+08:00",
    "updated_at": "2026-05-27T16:00:00+08:00"
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账本不存在 |
| `FORBIDDEN` | 非当前用户的账本 |
| `BASE_CURRENCY_IMMUTABLE` | 请求体中包含 `base_currency` 字段 |
| `VALIDATION_ERROR` | 名称为空 |

---

### 3.5 删除账本

**`DELETE /ledgers/{ledger_id}`**

删除账本及其下所有账目、分类、标签、预算（硬删除）。最后一个账本不可删除。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求示例

```
DELETE /api/v1/ledgers/ccdd3344-e29b-41d4-a716-446655440001
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 204 No Content
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账本不存在 |
| `FORBIDDEN` | 非当前用户的账本 |
| `LAST_LEDGER` | 这是用户的最后一个账本，不可删除 |

---

### 3.6 账本月度概览（已合并至账本列表）

> FR-2.5 的账本概览数据（本月收支、预算使用率）**已内嵌在 `GET /ledgers` 和 `GET /ledgers/{id}` 的响应体中**（`this_month_income`、`this_month_expense`、`budget_usage_rate` 三个字段），不再提供独立的 `/summary` 端点。
>
> 仪表盘的完整月度报表请使用 `GET /ledgers/{id}/reports/overview`（§8.1）。

---

## 4. 账目管理模块

### 账目对象字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | UUID | 账目 ID |
| `ledger_id` | UUID | 所属账本 |
| `type` | string | `"income"` / `"expense"` / `"transfer"` |
| `amount` | string | 原始金额（原始币种） |
| `currency` | string | 原始币种代码 |
| `base_amount` | string | 换算到账本基准币种的金额 |
| `exchange_rate` | string | 使用的汇率（原始币种→基准币种），同币种时为 `"1.00000000"` |
| `category` | object\|null | 分类对象 `{id, name, type}` |
| `tags` | array | 标签数组 `[{id, name}]` |
| `note` | string\|null | 备注 |
| `occurred_at` | timestamp | 账目发生时间 |
| `created_at` | timestamp | 录入时间 |
| `updated_at` | timestamp | 最近修改时间 |
| `transfer_pair_id` | UUID\|null | 转账配对 ID（仅 transfer 类型有值） |
| `transfer_peer` | object\|null | 转账对端摘要（仅 transfer 类型），`{ledger_id, amount, currency}` |

---

### 4.1 获取账目列表

**`GET /ledgers/{ledger_id}/transactions`**

支持多条件组合筛选，按 `occurred_at` 倒序分页返回。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `page` | integer | 1 | 页码 |
| `per_page` | integer | 20 | 每页条数，最大 100 |
| `type` | string | - | 账目类型筛选：`income` / `expense` / `transfer` |
| `category_id` | UUID | - | 分类 ID 筛选 |
| `tag_id` | UUID | - | 标签 ID 筛选 |
| `start_date` | date | - | 发生日期起始（含），格式 `YYYY-MM-DD` |
| `end_date` | date | - | 发生日期截止（含） |
| `min_amount` | number | - | 金额下限（原始金额，基准币种） |
| `max_amount` | number | - | 金额上限 |
| `currency` | string | - | 原始币种筛选 |
| `keyword` | string | - | 备注关键字（模糊搜索） |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../transactions?type=expense&start_date=2026-05-01&end_date=2026-05-31&page=1&per_page=20
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    {
      "id": "tx-uuid-001",
      "ledger_id": "aabb1122-...",
      "type": "expense",
      "amount": "45.00",
      "currency": "CNY",
      "base_amount": "45.00",
      "exchange_rate": "1.00000000",
      "category": { "id": "cat-uuid-001", "name": "餐饮", "type": "expense" },
      "tags": [ { "id": "tag-uuid-001", "name": "工作午餐" } ],
      "note": "公司附近的餐厅",
      "occurred_at": "2026-05-27T12:30:00+08:00",
      "created_at": "2026-05-27T12:31:00+08:00",
      "updated_at": "2026-05-27T12:31:00+08:00",
      "transfer_pair_id": null,
      "transfer_peer": null
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 47,
    "total_pages": 3
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账本不存在 |
| `FORBIDDEN` | 非当前用户的账本 |
| `VALIDATION_ERROR` | 日期格式错误、金额为负 |

---

### 4.2 创建账目

**`POST /ledgers/{ledger_id}/transactions`**

创建一笔收入、支出或转账账目。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 来源账本 ID |

#### 请求体（Body）—— 收入/支出

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `type` | string | 是 | `"income"` 或 `"expense"` |
| `amount` | number | 是 | 金额，必须 > 0 |
| `currency` | string | 是 | 原始币种代码 |
| `category_id` | UUID | 否 | 分类 ID |
| `tag_ids` | UUID[] | 否 | 标签 ID 数组 |
| `note` | string | 否 | 备注，最长 1000 字符 |
| `occurred_at` | timestamp | 是 | 发生时间（RFC 3339） |
| `custom_rate` | number | 否 | 手动指定汇率（覆盖系统汇率），仅在币种与基准不同时有意义 |

#### 请求体（Body）—— 转账

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `type` | string | 是 | `"transfer"` |
| `amount` | number | 是 | 来源账本扣减金额 |
| `currency` | string | 是 | 来源金额币种 |
| `target_ledger_id` | UUID | 是 | 目标账本 ID |
| `target_currency` | string | 否 | 目标账本接收币种，默认为目标账本基准币种 |
| `target_amount` | number | 否 | 目标账本入账金额；若不传则按系统汇率自动换算 |
| `note` | string | 否 | 备注 |
| `occurred_at` | timestamp | 是 | 发生时间 |
| `custom_rate` | number | 否 | 手动指定换算汇率 |

#### 请求示例（支出）

```json
POST /api/v1/ledgers/aabb1122-.../transactions
Authorization: Bearer ...

{
  "type": "expense",
  "amount": 45.00,
  "currency": "CNY",
  "category_id": "cat-uuid-001",
  "tag_ids": ["tag-uuid-001"],
  "note": "公司附近的餐厅",
  "occurred_at": "2026-05-27T12:30:00+08:00"
}
```

#### 请求示例（转账）

```json
POST /api/v1/ledgers/aabb1122-.../transactions
Authorization: Bearer ...

{
  "type": "transfer",
  "amount": 1000.00,
  "currency": "CNY",
  "target_ledger_id": "ccdd3344-...",
  "note": "出差备用金",
  "occurred_at": "2026-05-27T09:00:00+08:00"
}
```

#### 响应示例（支出）

```json
HTTP/1.1 201 Created

{
  "data": {
    "id": "tx-uuid-001",
    "ledger_id": "aabb1122-...",
    "type": "expense",
    "amount": "45.00",
    "currency": "CNY",
    "base_amount": "45.00",
    "exchange_rate": "1.00000000",
    "category": { "id": "cat-uuid-001", "name": "餐饮", "type": "expense" },
    "tags": [ { "id": "tag-uuid-001", "name": "工作午餐" } ],
    "note": "公司附近的餐厅",
    "occurred_at": "2026-05-27T12:30:00+08:00",
    "created_at": "2026-05-27T12:31:00+08:00",
    "updated_at": "2026-05-27T12:31:00+08:00",
    "transfer_pair_id": null,
    "transfer_peer": null
  }
}
```

#### 响应示例（转账——返回来源侧记录）

```json
HTTP/1.1 201 Created

{
  "data": {
    "id": "tx-uuid-002",
    "ledger_id": "aabb1122-...",
    "type": "transfer",
    "amount": "1000.00",
    "currency": "CNY",
    "base_amount": "1000.00",
    "exchange_rate": "1.00000000",
    "category": null,
    "tags": [],
    "note": "出差备用金",
    "occurred_at": "2026-05-27T09:00:00+08:00",
    "created_at": "2026-05-27T09:01:00+08:00",
    "updated_at": "2026-05-27T09:01:00+08:00",
    "transfer_pair_id": "pair-uuid-001",
    "transfer_peer": {
      "ledger_id": "ccdd3344-...",
      "amount": "138.50",
      "currency": "USD"
    }
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账本或分类不存在 |
| `FORBIDDEN` | 账本不属于当前用户 |
| `VALIDATION_ERROR` | 金额 ≤ 0、币种不支持、日期格式错误 |
| `EXCHANGE_RATE_UNAVAILABLE` | 账目日期无可用汇率且无任何历史记录 |

---

### 4.3 获取账目详情

**`GET /ledgers/{ledger_id}/transactions/{transaction_id}`**

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |
| `transaction_id` | UUID | 账目 ID |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../transactions/tx-uuid-001
Authorization: Bearer ...
```

#### 响应示例

同 §4.2 创建接口的响应体结构，HTTP 200。

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账目不存在或不属于该账本 |
| `FORBIDDEN` | 账本不属于当前用户 |

---

### 4.4 修改账目

**`PUT /ledgers/{ledger_id}/transactions/{transaction_id}`**

修改账目字段。若修改了金额、币种、发生日期或分类，后端自动重新换算汇率并更新预算关联。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |
| `transaction_id` | UUID | 账目 ID |

#### 请求体（Body）

所有字段可选，仅传需修改的字段。不可修改 `type`（类型变更需删除后重建）。

| 字段 | 类型 | 说明 |
|------|------|------|
| `amount` | number | 新金额 |
| `currency` | string | 新币种 |
| `category_id` | UUID\|null | 新分类（null 为清除） |
| `tag_ids` | UUID[] | 新标签集合（全量替换） |
| `note` | string\|null | 新备注 |
| `occurred_at` | timestamp | 新发生时间 |
| `custom_rate` | number\|null | 新手动汇率（null 为还原系统汇率） |

#### 请求示例

```json
PUT /api/v1/ledgers/aabb1122-.../transactions/tx-uuid-001
Authorization: Bearer ...

{
  "amount": 52.00,
  "note": "公司附近的餐厅（含小费）"
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "id": "tx-uuid-001",
    "ledger_id": "aabb1122-...",
    "type": "expense",
    "amount": "52.00",
    "currency": "CNY",
    "base_amount": "52.00",
    "exchange_rate": "1.00000000",
    "category": { "id": "cat-uuid-001", "name": "餐饮", "type": "expense" },
    "tags": [ { "id": "tag-uuid-001", "name": "工作午餐" } ],
    "note": "公司附近的餐厅（含小费）",
    "occurred_at": "2026-05-27T12:30:00+08:00",
    "created_at": "2026-05-27T12:31:00+08:00",
    "updated_at": "2026-05-27T16:00:00+08:00",
    "transfer_pair_id": null,
    "transfer_peer": null
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账目或分类不存在 |
| `FORBIDDEN` | 账本不属于当前用户 |
| `VALIDATION_ERROR` | 金额 ≤ 0 等 |
| `EXCHANGE_RATE_UNAVAILABLE` | 新日期无汇率 |

---

### 4.5 删除账目

**`DELETE /ledgers/{ledger_id}/transactions/{transaction_id}`**

硬删除。若为转账账目，同时删除对端记录（通过 `transfer_pair_id` 关联）。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |
| `transaction_id` | UUID | 账目 ID |

#### 请求示例

```
DELETE /api/v1/ledgers/aabb1122-.../transactions/tx-uuid-001
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 204 No Content
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账目不存在 |
| `FORBIDDEN` | 账本不属于当前用户 |

---

### 4.6 批量删除账目

**`POST /ledgers/{ledger_id}/transactions/batch-delete`**

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `transaction_ids` | UUID[] | 是 | 要删除的账目 ID 列表，最多 200 条 |

#### 请求示例

```json
POST /api/v1/ledgers/aabb1122-.../transactions/batch-delete
Authorization: Bearer ...

{
  "transaction_ids": [
    "tx-uuid-001",
    "tx-uuid-002",
    "tx-uuid-003"
  ]
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "deleted_count": 3
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `FORBIDDEN` | 账本不属于当前用户 |
| `VALIDATION_ERROR` | `transaction_ids` 为空或超过 200 条 |

---

### 4.7 批量修改分类

**`POST /ledgers/{ledger_id}/transactions/batch-update-category`**

将多条账目的分类统一修改为指定分类。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `transaction_ids` | UUID[] | 是 | 目标账目 ID 列表，最多 200 条 |
| `category_id` | UUID\|null | 是 | 新分类 ID；null 为清除分类 |

#### 请求示例

```json
POST /api/v1/ledgers/aabb1122-.../transactions/batch-update-category
Authorization: Bearer ...

{
  "transaction_ids": ["tx-uuid-001", "tx-uuid-002"],
  "category_id": "cat-uuid-002"
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "updated_count": 2
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 指定的 `category_id` 不存在 |
| `FORBIDDEN` | 账本不属于当前用户 |
| `VALIDATION_ERROR` | `transaction_ids` 为空或超过 200 条 |

---

## 5. 分类与标签模块

### 5.1 获取系统预设分类

**`GET /categories/system`**

返回所有系统预设分类（不属于任何账本）。

#### 请求示例

```
GET /api/v1/categories/system
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    {
      "id": "sys-cat-001",
      "ledger_id": null,
      "parent_id": null,
      "name": "餐饮",
      "type": "expense",
      "is_system": true,
      "sort_order": 1
    },
    {
      "id": "sys-cat-002",
      "ledger_id": null,
      "parent_id": null,
      "name": "交通",
      "type": "expense",
      "is_system": true,
      "sort_order": 2
    },
    {
      "id": "sys-cat-010",
      "ledger_id": null,
      "parent_id": null,
      "name": "工资",
      "type": "income",
      "is_system": true,
      "sort_order": 10
    }
  ]
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `UNAUTHORIZED` | 未登录 |

---

### 5.2 获取账本分类列表

**`GET /ledgers/{ledger_id}/categories`**

返回该账本的自定义分类列表（不含系统分类）。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `type` | string | - | 按类型筛选：`income` / `expense` |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../categories?type=expense
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    {
      "id": "cat-uuid-001",
      "ledger_id": "aabb1122-...",
      "parent_id": null,
      "name": "咖啡",
      "type": "expense",
      "is_system": false,
      "sort_order": 0
    }
  ]
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账本不存在 |
| `FORBIDDEN` | 非当前用户的账本 |

---

### 5.3 创建自定义分类

**`POST /ledgers/{ledger_id}/categories`**

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 分类名称，1～100 字符 |
| `type` | string | 是 | `"income"` 或 `"expense"` |
| `parent_id` | UUID | 否 | 父分类 ID（二级分类），只支持一层嵌套 |
| `sort_order` | integer | 否 | 排序权重，默认 0 |

#### 请求示例

```json
POST /api/v1/ledgers/aabb1122-.../categories
Authorization: Bearer ...

{
  "name": "咖啡",
  "type": "expense"
}
```

#### 响应示例

```json
HTTP/1.1 201 Created

{
  "data": {
    "id": "cat-uuid-001",
    "ledger_id": "aabb1122-...",
    "parent_id": null,
    "name": "咖啡",
    "type": "expense",
    "is_system": false,
    "sort_order": 0
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | `parent_id` 指向的父分类不存在 |
| `VALIDATION_ERROR` | 名称为空、type 非法、父分类本身已是二级分类（不支持三级） |
| `FORBIDDEN` | 非当前用户的账本 |

---

### 5.4 修改分类

**`PUT /ledgers/{ledger_id}/categories/{category_id}`**

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |
| `category_id` | UUID | 分类 ID |

#### 请求体（Body）

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 新名称 |
| `sort_order` | integer | 新排序权重 |

#### 请求示例

```json
PUT /api/v1/ledgers/aabb1122-.../categories/cat-uuid-001
Authorization: Bearer ...

{
  "name": "精品咖啡"
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "id": "cat-uuid-001",
    "ledger_id": "aabb1122-...",
    "parent_id": null,
    "name": "精品咖啡",
    "type": "expense",
    "is_system": false,
    "sort_order": 0
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 分类不存在 |
| `FORBIDDEN` | 系统分类不可修改 / 非当前用户账本 |

---

### 5.5 删除分类

**`DELETE /ledgers/{ledger_id}/categories/{category_id}`**

删除自定义分类。若该分类下有账目，返回错误并提示迁移（由前端二次确认后传入 `force=true`）。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |
| `category_id` | UUID | 分类 ID |

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `force` | boolean | false | `true` 时强制删除并将关联账目的分类设为 null |

#### 请求示例

```
DELETE /api/v1/ledgers/aabb1122-.../categories/cat-uuid-001?force=false
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 204 No Content
```

错误响应（有关联账目且未传 force）：
```json
HTTP/1.1 422 Unprocessable Entity

{
  "error": {
    "code": "CATEGORY_HAS_TRANSACTIONS",
    "message": "该分类下有 5 条账目，请先迁移或使用 force=true 强制删除",
    "details": { "transaction_count": 5 }
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 分类不存在 |
| `FORBIDDEN` | 系统分类不可删除 / 非当前用户账本 |
| `CATEGORY_HAS_TRANSACTIONS` | 存在关联账目且 `force=false` |

---

### 5.6 获取标签列表

**`GET /ledgers/{ledger_id}/tags`**

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../tags
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    { "id": "tag-uuid-001", "ledger_id": "aabb1122-...", "name": "工作午餐" },
    { "id": "tag-uuid-002", "ledger_id": "aabb1122-...", "name": "聚会" }
  ]
}
```

---

### 5.7 创建标签

**`POST /ledgers/{ledger_id}/tags`**

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 标签名称，1～50 字符，同账本内唯一 |

#### 请求示例

```json
POST /api/v1/ledgers/aabb1122-.../tags
Authorization: Bearer ...

{
  "name": "工作午餐"
}
```

#### 响应示例

```json
HTTP/1.1 201 Created

{
  "data": {
    "id": "tag-uuid-001",
    "ledger_id": "aabb1122-...",
    "name": "工作午餐"
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `CONFLICT` | 同账本内标签名已存在 |
| `VALIDATION_ERROR` | 名称为空或超过 50 字符 |

---

### 5.8 修改标签

**`PUT /ledgers/{ledger_id}/tags/{tag_id}`**

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | 是 | 新标签名称 |

#### 请求示例

```json
PUT /api/v1/ledgers/aabb1122-.../tags/tag-uuid-001
Authorization: Bearer ...

{
  "name": "工作餐"
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "id": "tag-uuid-001",
    "ledger_id": "aabb1122-...",
    "name": "工作餐"
  }
}
```

---

### 5.9 删除标签

**`DELETE /ledgers/{ledger_id}/tags/{tag_id}`**

删除标签，自动移除所有账目与该标签的关联。

#### 请求示例

```
DELETE /api/v1/ledgers/aabb1122-.../tags/tag-uuid-001
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 204 No Content
```

---

## 6. 预算管理模块

### 预算对象字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | UUID | 预算 ID |
| `ledger_id` | UUID | 所属账本 |
| `category` | object\|null | 分类对象 `{id, name}`；null 表示总预算 |
| `period` | string | `"week"` / `"month"` / `"year"` |
| `amount` | string | 预算上限（账本基准币种） |
| `spent` | string | 当前周期已支出（换算为基准币种） |
| `remaining` | string | 剩余预算（可为负，表示超支） |
| `usage_rate` | number | 使用率（0.0 起，超 1.0 为超支） |
| `period_start` | date | 当前周期开始日期 |
| `period_end` | date | 当前周期结束日期 |
| `created_at` | timestamp | 创建时间 |
| `updated_at` | timestamp | 最近修改时间 |

---

### 6.1 获取预算列表

**`GET /ledgers/{ledger_id}/budgets`**

返回账本所有预算及当前周期执行情况，按使用率倒序排列。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../budgets
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    {
      "id": "bgt-uuid-001",
      "ledger_id": "aabb1122-...",
      "category": null,
      "period": "month",
      "amount": "4000.00",
      "spent": "3240.50",
      "remaining": "759.50",
      "usage_rate": 0.81,
      "period_start": "2026-05-01",
      "period_end": "2026-05-31",
      "created_at": "2026-05-01T00:00:00+08:00",
      "updated_at": "2026-05-01T00:00:00+08:00"
    },
    {
      "id": "bgt-uuid-002",
      "ledger_id": "aabb1122-...",
      "category": { "id": "sys-cat-001", "name": "餐饮" },
      "period": "month",
      "amount": "800.00",
      "spent": "620.00",
      "remaining": "180.00",
      "usage_rate": 0.775,
      "period_start": "2026-05-01",
      "period_end": "2026-05-31",
      "created_at": "2026-05-01T00:00:00+08:00",
      "updated_at": "2026-05-01T00:00:00+08:00"
    }
  ]
}
```

---

### 6.2 创建预算

**`POST /ledgers/{ledger_id}/budgets`**

同一账本内，同一（category_id, period）组合唯一。`category_id` 为 null 时为总预算，每账本每周期只能有一个总预算。

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `category_id` | UUID\|null | 否 | 分类 ID；null 表示总预算 |
| `period` | string | 是 | `"week"` / `"month"` / `"year"` |
| `amount` | number | 是 | 预算金额，必须 > 0，单位为账本基准币种 |

#### 请求示例

```json
POST /api/v1/ledgers/aabb1122-.../budgets
Authorization: Bearer ...

{
  "category_id": null,
  "period": "month",
  "amount": 4000.00
}
```

#### 响应示例

```json
HTTP/1.1 201 Created

{
  "data": {
    "id": "bgt-uuid-001",
    "ledger_id": "aabb1122-...",
    "category": null,
    "period": "month",
    "amount": "4000.00",
    "spent": "0.00",
    "remaining": "4000.00",
    "usage_rate": 0.0,
    "period_start": "2026-05-01",
    "period_end": "2026-05-31",
    "created_at": "2026-05-27T10:00:00+08:00",
    "updated_at": "2026-05-27T10:00:00+08:00"
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `CONFLICT` | 该账本该分类该周期已存在预算 |
| `VALIDATION_ERROR` | 金额 ≤ 0，period 非法 |
| `NOT_FOUND` | `category_id` 不存在 |

---

### 6.3 获取预算详情

**`GET /ledgers/{ledger_id}/budgets/{budget_id}`**

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../budgets/bgt-uuid-001
Authorization: Bearer ...
```

响应格式同 §6.1 列表中的单条对象，HTTP 200。

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 预算不存在 |
| `FORBIDDEN` | 非当前用户的账本 |

---

### 6.4 修改预算

**`PUT /ledgers/{ledger_id}/budgets/{budget_id}`**

只能修改预算金额，不可修改 `category_id` 或 `period`（变更请删除后重建）。

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `amount` | number | 是 | 新预算金额，必须 > 0 |

#### 请求示例

```json
PUT /api/v1/ledgers/aabb1122-.../budgets/bgt-uuid-001
Authorization: Bearer ...

{
  "amount": 5000.00
}
```

#### 响应示例

返回更新后的预算对象，HTTP 200，结构同 §6.2。

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 预算不存在 |
| `VALIDATION_ERROR` | 金额 ≤ 0 |

---

### 6.5 删除预算

**`DELETE /ledgers/{ledger_id}/budgets/{budget_id}`**

#### 请求示例

```
DELETE /api/v1/ledgers/aabb1122-.../budgets/bgt-uuid-001
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 204 No Content
```

---

## 7. 汇率与多币种模块

### 7.1 查询汇率

**`GET /exchange-rates`**

查询指定货币对在指定日期或日期范围内的汇率记录。

#### Query 参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `base` | string | 是 | 基准币种代码（如 `CNY`） |
| `quote` | string | 是 | 目标币种代码（如 `USD`） |
| `date` | date | 否 | 精确日期，若不传则返回近 30 天记录 |
| `start_date` | date | 否 | 起始日期（与 `end_date` 配合使用） |
| `end_date` | date | 否 | 截止日期 |

#### 请求示例

```
GET /api/v1/exchange-rates?base=CNY&quote=USD&start_date=2026-05-01&end_date=2026-05-27
Authorization: Bearer ...
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data[].base_currency` | string | 基准币种 |
| `data[].quote_currency` | string | 目标币种 |
| `data[].rate` | string | 汇率（1 基准 = rate 目标） |
| `data[].date` | date | 汇率日期 |
| `data[].created_at` | timestamp | 入库时间 |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    {
      "base_currency": "CNY",
      "quote_currency": "USD",
      "rate": "0.13850000",
      "date": "2026-05-27",
      "created_at": "2026-05-27T00:05:00+00:00"
    },
    {
      "base_currency": "CNY",
      "quote_currency": "USD",
      "rate": "0.13820000",
      "date": "2026-05-26",
      "created_at": "2026-05-26T00:05:00+00:00"
    }
  ]
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `VALIDATION_ERROR` | 币种代码不支持、日期格式错误 |

---

### 7.2 获取最新汇率（所有支持币种）

**`GET /exchange-rates/latest`**

返回指定基准币种对所有支持币种的最新汇率。

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `base` | string | 用户默认基准币种 | 基准币种代码 |

#### 请求示例

```
GET /api/v1/exchange-rates/latest?base=CNY
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "base": "CNY",
    "date": "2026-05-27",
    "rates": {
      "USD": "0.13850000",
      "EUR": "0.12700000",
      "JPY": "20.80000000",
      "GBP": "0.10950000",
      "HKD": "1.08200000",
      "KRW": "189.50000000",
      "AUD": "0.21300000",
      "CAD": "0.19200000"
    },
    "is_stale": false,
    "stale_days": 0
  }
}
```

`is_stale` 为 `true` 表示数据库中最新汇率不是今日（API 拉取失败），`stale_days` 为距今天数。

---

### 7.3 获取支持的币种列表

**`GET /exchange-rates/currencies`**

返回系统支持的所有币种代码及名称。无需鉴权。

#### 请求示例

```
GET /api/v1/exchange-rates/currencies
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": [
    { "code": "CNY", "name": "人民币", "symbol": "¥" },
    { "code": "USD", "name": "美元", "symbol": "$" },
    { "code": "EUR", "name": "欧元", "symbol": "€" },
    { "code": "JPY", "name": "日元", "symbol": "¥" },
    { "code": "GBP", "name": "英镑", "symbol": "£" },
    { "code": "HKD", "name": "港币", "symbol": "HK$" },
    { "code": "KRW", "name": "韩元", "symbol": "₩" },
    { "code": "AUD", "name": "澳元", "symbol": "A$" },
    { "code": "CAD", "name": "加拿大元", "symbol": "C$" }
  ]
}
```

---

## 8. 报表与统计模块

**说明**：所有报表接口返回的金额均已换算为账本基准币种。`transfer` 类型账目不计入收支统计。

---

### 8.1 仪表盘概览

**`GET /ledgers/{ledger_id}/reports/overview`**

返回当前账本本月核心指标（FR-7.1）。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `year` | integer | 当前年 | 统计年份 |
| `month` | integer | 当前月 | 统计月份（1～12） |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../reports/overview?year=2026&month=5
Authorization: Bearer ...
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.period` | string | 统计周期 `"YYYY-MM"` |
| `data.income` | string | 本月总收入 |
| `data.expense` | string | 本月总支出 |
| `data.balance` | string | 账本当前余额 |
| `data.budget_usage_rate` | number\|null | 总预算使用率（若未设置则 null） |
| `data.currency` | string | 账本基准币种 |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "period": "2026-05",
    "income": "8500.00",
    "expense": "3240.50",
    "balance": "3240.50",
    "budget_usage_rate": 0.81,
    "currency": "CNY"
  }
}
```

---

### 8.2 收支趋势图

**`GET /ledgers/{ledger_id}/reports/trend`**

返回近 N 个月的月度收支数据（折线图数据源，FR-7.2）。

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `months` | integer | 6 | 统计月数：3、6 或 12 |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../reports/trend?months=6
Authorization: Bearer ...
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.currency` | string | 账本基准币种 |
| `data.months` | array | 按月排列的数据点 |
| `data.months[].period` | string | `"YYYY-MM"` |
| `data.months[].income` | string | 该月总收入 |
| `data.months[].expense` | string | 该月总支出 |
| `data.months[].net` | string | 该月净收支（income - expense） |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "currency": "CNY",
    "months": [
      { "period": "2025-12", "income": "7200.00", "expense": "4100.00", "net": "3100.00" },
      { "period": "2026-01", "income": "8000.00", "expense": "5200.00", "net": "2800.00" },
      { "period": "2026-02", "income": "7500.00", "expense": "3800.00", "net": "3700.00" },
      { "period": "2026-03", "income": "8200.00", "expense": "4300.00", "net": "3900.00" },
      { "period": "2026-04", "income": "8300.00", "expense": "4100.00", "net": "4200.00" },
      { "period": "2026-05", "income": "8500.00", "expense": "3240.50", "net": "5259.50" }
    ]
  }
}
```

---

### 8.3 分类占比图

**`GET /ledgers/{ledger_id}/reports/category-breakdown`**

返回选定日期范围内支出（或收入）按分类的占比数据（饼图数据源，FR-7.3）。

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `start_date` | date | 当月 1 日 | 起始日期 |
| `end_date` | date | 今日 | 截止日期 |
| `type` | string | `expense` | `"income"` 或 `"expense"` |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../reports/category-breakdown?start_date=2026-05-01&end_date=2026-05-27&type=expense
Authorization: Bearer ...
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.total` | string | 合计金额 |
| `data.currency` | string | 账本基准币种 |
| `data.items[].category_id` | UUID\|null | 分类 ID（null 表示未分类） |
| `data.items[].category_name` | string | 分类名称 |
| `data.items[].amount` | string | 该分类金额 |
| `data.items[].ratio` | number | 占比（0.0～1.0） |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "total": "3240.50",
    "currency": "CNY",
    "items": [
      { "category_id": "sys-cat-001", "category_name": "餐饮", "amount": "620.00", "ratio": 0.191 },
      { "category_id": "sys-cat-002", "category_name": "交通", "amount": "430.00", "ratio": 0.133 },
      { "category_id": "sys-cat-003", "category_name": "购物", "amount": "1800.00", "ratio": 0.555 },
      { "category_id": null, "category_name": "未分类", "amount": "390.50", "ratio": 0.121 }
    ]
  }
}
```

---

### 8.4 同环比分析

**`GET /ledgers/{ledger_id}/reports/comparison`**

返回指定月份与上月（环比）和去年同月（同比）的对比数据（FR-7.4）。

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `year` | integer | 当前年 | 统计年份 |
| `month` | integer | 当前月 | 统计月份（1～12） |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../reports/comparison?year=2026&month=5
Authorization: Bearer ...
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.current.period` | string | 当前月 `"YYYY-MM"` |
| `data.current.income` | string | 当月收入 |
| `data.current.expense` | string | 当月支出 |
| `data.mom.period` | string | 上月 `"YYYY-MM"` |
| `data.mom.income` | string | 上月收入 |
| `data.mom.expense` | string | 上月支出 |
| `data.mom.income_change_rate` | number\|null | 收入环比变化率（正数为增长，负数为下降），无数据时 null |
| `data.mom.expense_change_rate` | number\|null | 支出环比变化率 |
| `data.yoy.period` | string | 去年同月 `"YYYY-MM"` |
| `data.yoy.income` | string | 去年同月收入 |
| `data.yoy.expense` | string | 去年同月支出 |
| `data.yoy.income_change_rate` | number\|null | 收入同比变化率 |
| `data.yoy.expense_change_rate` | number\|null | 支出同比变化率 |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "currency": "CNY",
    "current": {
      "period": "2026-05",
      "income": "8500.00",
      "expense": "3240.50"
    },
    "mom": {
      "period": "2026-04",
      "income": "8300.00",
      "expense": "4100.00",
      "income_change_rate": 0.024,
      "expense_change_rate": -0.210
    },
    "yoy": {
      "period": "2025-05",
      "income": "7800.00",
      "expense": "3500.00",
      "income_change_rate": 0.090,
      "expense_change_rate": -0.074
    }
  }
}
```

---

### 8.5 Top 分类

**`GET /ledgers/{ledger_id}/reports/top-categories`**

返回选定周期内支出 Top 5 分类和收入 Top 3 来源（FR-7.5）。

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `start_date` | date | 当月 1 日 | 起始日期 |
| `end_date` | date | 今日 | 截止日期 |
| `expense_limit` | integer | 5 | 支出 Top N |
| `income_limit` | integer | 3 | 收入 Top N |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../reports/top-categories?start_date=2026-05-01&end_date=2026-05-27
Authorization: Bearer ...
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "currency": "CNY",
    "top_expenses": [
      { "rank": 1, "category_id": "sys-cat-003", "category_name": "购物", "amount": "1800.00" },
      { "rank": 2, "category_id": "sys-cat-001", "category_name": "餐饮", "amount": "620.00" },
      { "rank": 3, "category_id": "sys-cat-002", "category_name": "交通", "amount": "430.00" },
      { "rank": 4, "category_id": "sys-cat-005", "category_name": "居住", "amount": "300.00" },
      { "rank": 5, "category_id": null, "category_name": "未分类", "amount": "90.50" }
    ],
    "top_incomes": [
      { "rank": 1, "category_id": "sys-cat-010", "category_name": "工资", "amount": "8000.00" },
      { "rank": 2, "category_id": "sys-cat-011", "category_name": "奖金", "amount": "500.00" },
      { "rank": 3, "category_id": null, "category_name": "未分类", "amount": "0.00" }
    ]
  }
}
```

---

### 8.6 多账本合并报表

**`POST /reports/multi-ledger`**

汇总多个账本的收支数据，统一换算到指定基准币种（FR-7.6）。

#### 请求体（Body）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `ledger_ids` | UUID[] | 是 | 参与合并的账本 ID 列表（至少 2 个，最多 20 个） |
| `start_date` | date | 是 | 统计起始日期 |
| `end_date` | date | 是 | 统计截止日期 |
| `base_currency` | string | 否 | 合并统计基准币种，默认 `CNY` |

#### 请求示例

```json
POST /api/v1/reports/multi-ledger
Authorization: Bearer ...

{
  "ledger_ids": ["aabb1122-...", "ccdd3344-..."],
  "start_date": "2026-05-01",
  "end_date": "2026-05-27",
  "base_currency": "CNY"
}
```

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "base_currency": "CNY",
    "period": { "start": "2026-05-01", "end": "2026-05-27" },
    "total_income": "9638.50",
    "total_expense": "4340.50",
    "total_net": "5298.00",
    "ledgers": [
      {
        "ledger_id": "aabb1122-...",
        "ledger_name": "日常",
        "income": "8500.00",
        "expense": "3240.50"
      },
      {
        "ledger_id": "ccdd3344-...",
        "ledger_name": "出差",
        "income": "1138.50",
        "expense": "1100.00"
      }
    ]
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `FORBIDDEN` | `ledger_ids` 中包含非当前用户的账本 |
| `VALIDATION_ERROR` | `ledger_ids` 少于 2 个或日期格式错误 |

---

## 9. 数据导出与备份模块

### 9.1 导出 CSV

**`GET /ledgers/{ledger_id}/export/csv`**

导出选定账本、日期范围内的账目为 CSV 文件（FR-8.4）。

#### Path 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `ledger_id` | UUID | 账本 ID |

#### Query 参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `start_date` | date | 账本最早账目日期 | 导出起始日期 |
| `end_date` | date | 今日 | 导出截止日期 |

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../export/csv?start_date=2026-05-01&end_date=2026-05-31
Authorization: Bearer ...
```

#### 响应

```
HTTP/1.1 200 OK
Content-Type: text/csv; charset=utf-8
Content-Disposition: attachment; filename="ledger_日常_2026-05.csv"

日期,类型,金额,币种,分类,标签,备注
2026-05-27,支出,45.00,CNY,餐饮,工作午餐,公司附近的餐厅
2026-05-26,支出,30.00,CNY,交通,,打车回家
...
```

#### CSV 字段说明

| CSV 列名 | 说明 |
|---------|------|
| 日期 | 账目发生日期 `YYYY-MM-DD` |
| 类型 | `收入` / `支出` / `转账` |
| 金额 | 原始金额，保留 2 位小数 |
| 币种 | 原始币种代码 |
| 基准金额 | 换算为账本基准币种的金额 |
| 基准币种 | 账本基准币种 |
| 分类 | 分类名称，无分类时为空 |
| 标签 | 多个标签用竖线 `|` 分隔 |
| 备注 | 备注内容 |

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `NOT_FOUND` | 账本不存在 |
| `FORBIDDEN` | 非当前用户的账本 |

---

### 9.2 导出 Excel

**`GET /ledgers/{ledger_id}/export/excel`**

导出为 `.xlsx` 格式（FR-8.5）。表头加粗，金额列为数值格式。

#### Path 参数 / Query 参数

与 §9.1 CSV 接口完全一致。

#### 请求示例

```
GET /api/v1/ledgers/aabb1122-.../export/excel?start_date=2026-05-01&end_date=2026-05-31
Authorization: Bearer ...
```

#### 响应

```
HTTP/1.1 200 OK
Content-Type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
Content-Disposition: attachment; filename="ledger_日常_2026-05.xlsx"

<binary xlsx content>
```

#### 错误码

同 §9.1。

---

### 9.3 全量 JSON 备份

**`GET /data/backup`**

导出当前用户所有账本数据为 JSON 文件（FR-8.6）。不含用户密码等安全信息。

#### 请求示例

```
GET /api/v1/data/backup
Authorization: Bearer ...
```

#### 响应

```
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8
Content-Disposition: attachment; filename="ledgerrs_backup_2026-05-27.json"
```

#### JSON 备份文件结构

```json
{
  "schema_version": "1.0",
  "exported_at": "2026-05-27T14:00:00+08:00",
  "ledgers": [
    {
      "id": "aabb1122-...",
      "name": "日常",
      "icon": "wallet",
      "base_currency": "CNY",
      "initial_balance": "0.00",
      "created_at": "2026-05-01T10:00:00+08:00"
    }
  ],
  "categories": [
    {
      "id": "cat-uuid-001",
      "ledger_id": "aabb1122-...",
      "parent_id": null,
      "name": "咖啡",
      "type": "expense",
      "is_system": false,
      "sort_order": 0
    }
  ],
  "tags": [
    { "id": "tag-uuid-001", "ledger_id": "aabb1122-...", "name": "工作午餐" }
  ],
  "transactions": [
    {
      "id": "tx-uuid-001",
      "ledger_id": "aabb1122-...",
      "type": "expense",
      "amount": "45.00",
      "currency": "CNY",
      "category_id": "cat-uuid-001",
      "tag_ids": ["tag-uuid-001"],
      "note": "公司附近的餐厅",
      "occurred_at": "2026-05-27T12:30:00+08:00",
      "custom_rate": null,
      "transfer_pair_id": null
    }
  ],
  "budgets": [
    {
      "id": "bgt-uuid-001",
      "ledger_id": "aabb1122-...",
      "category_id": null,
      "period": "month",
      "amount": "4000.00"
    }
  ]
}
```

---

### 9.4 备份还原

**`POST /data/restore`**

从 JSON 备份文件还原数据（FR-8.7）。

#### 请求

`Content-Type: multipart/form-data`

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `file` | file | 是 | JSON 备份文件（最大 50 MB） |
| `mode` | string | 是 | `"merge"`（按 ID 去重，已存在则跳过）或 `"replace"`（先清空再导入） |

#### 请求示例

```
POST /api/v1/data/restore
Authorization: Bearer ...
Content-Type: multipart/form-data; boundary=---boundary

-----boundary
Content-Disposition: form-data; name="mode"

merge
-----boundary
Content-Disposition: form-data; name="file"; filename="backup.json"
Content-Type: application/json

{ ... }
-----boundary--
```

#### 响应字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `data.mode` | string | 实际使用的还原模式 |
| `data.restored.ledgers` | integer | 导入的账本数 |
| `data.restored.categories` | integer | 导入的分类数 |
| `data.restored.tags` | integer | 导入的标签数 |
| `data.restored.transactions` | integer | 导入的账目数 |
| `data.restored.budgets` | integer | 导入的预算数 |
| `data.skipped` | integer | 跳过（ID 冲突）的记录总数（仅 merge 模式） |

#### 响应示例

```json
HTTP/1.1 200 OK

{
  "data": {
    "mode": "merge",
    "restored": {
      "ledgers": 2,
      "categories": 5,
      "tags": 3,
      "transactions": 147,
      "budgets": 4
    },
    "skipped": 12
  }
}
```

#### 错误码

| 错误码 | 触发条件 |
|--------|---------|
| `INVALID_BACKUP_FORMAT` | JSON 解析失败或 `schema_version` 不兼容 |
| `VALIDATION_ERROR` | `mode` 字段值非法、文件超过 50 MB |
| `DATABASE_ERROR` | 事务执行失败，数据已回滚（数据保持不变） |

---

## 10. FR 编号 → 接口映射表

| FR 编号 | 描述 | 对应接口 |
|---------|------|---------|
| FR-1.1 | 用户注册 | `POST /auth/register` |
| FR-1.2 | 用户登录 + 锁定 | `POST /auth/login` |
| FR-1.3 | 修改个人信息 | `PUT /auth/profile` |
| FR-1.4 | 修改密码 | `PUT /auth/password` |
| FR-1.5 | 退出登录（撤销 token） | `POST /auth/logout`，`POST /auth/refresh` |
| FR-2.1 | 创建账本（含自动创建"日常"） | `POST /auth/register`（服务端自动创建），`POST /ledgers` |
| FR-2.2 | 切换账本 | `GET /ledgers`（前端切换，无独立接口） |
| FR-2.3 | 编辑账本（基准币种不可改） | `PUT /ledgers/{id}` |
| FR-2.4 | 删除账本（最后一个不可删） | `DELETE /ledgers/{id}` |
| FR-2.5 | 账本概览 | `GET /ledgers` / `GET /ledgers/{id}`（含 `this_month_income`、`this_month_expense`、`budget_usage_rate` 字段） |
| FR-3.1 | 新增账目 | `POST /ledgers/{id}/transactions` |
| FR-3.2 | 转账记账 | `POST /ledgers/{id}/transactions`（type=transfer） |
| FR-3.3 | 编辑账目 | `PUT /ledgers/{id}/transactions/{tx_id}` |
| FR-3.4 | 删除账目（硬删除） | `DELETE /ledgers/{id}/transactions/{tx_id}` |
| FR-3.5 | 账目列表多条件筛选 + 分页 | `GET /ledgers/{id}/transactions` |
| FR-3.6 | 账目详情 | `GET /ledgers/{id}/transactions/{tx_id}` |
| FR-3.7 | 批量删除 / 批量修改分类 | `POST /…/batch-delete`，`POST /…/batch-update-category` |
| FR-4.1 | 系统预设分类 | `GET /categories/system` |
| FR-4.2 | 自定义分类 CRUD | `GET/POST/PUT/DELETE /ledgers/{id}/categories` |
| FR-4.3 | 二级分类（parent_id 字段） | `POST /ledgers/{id}/categories`（含 `parent_id`） |
| FR-4.4 | 标签管理 | `GET/POST/PUT/DELETE /ledgers/{id}/tags` |
| FR-5.1 | 总预算 | `POST /ledgers/{id}/budgets`（category_id=null） |
| FR-5.2 | 分类预算 | `POST /ledgers/{id}/budgets`（含 category_id） |
| FR-5.3 | 预算周期 | `period` 字段（week/month/year） |
| FR-5.4 | 预算执行情况实时计算 | `GET /ledgers/{id}/budgets`（spent/remaining/usage_rate） |
| FR-5.5 | 超支提醒（已移除通知，使用率在列表展示） | `GET /ledgers/{id}/budgets`（usage_rate 字段） |
| FR-5.6 | 预算列表（按使用率倒序） | `GET /ledgers/{id}/budgets` |
| FR-6.1 | 币种支持 | `GET /exchange-rates/currencies` |
| FR-6.2 | 每日拉取汇率（后台 Job） | `GET /exchange-rates` / `GET /exchange-rates/latest` |
| FR-6.3 | 汇率换算（发生日期） | 账目创建/修改时服务端自动处理（`base_amount` 字段） |
| FR-6.4 | 离线兜底（is_stale 标记） | `GET /exchange-rates/latest`（`is_stale`, `stale_days` 字段） |
| FR-6.5 | 手动覆盖汇率 | `POST/PUT /ledgers/{id}/transactions`（`custom_rate` 字段） |
| FR-7.1 | 仪表盘 | `GET /ledgers/{id}/reports/overview` |
| FR-7.2 | 收支趋势图 | `GET /ledgers/{id}/reports/trend` |
| FR-7.3 | 分类占比图 | `GET /ledgers/{id}/reports/category-breakdown` |
| FR-7.4 | 同环比分析 | `GET /ledgers/{id}/reports/comparison` |
| FR-7.5 | Top 分类 | `GET /ledgers/{id}/reports/top-categories` |
| FR-7.6 | 多账本合并视图 | `POST /reports/multi-ledger` |
| FR-7.7 | 时间维度切换 | 各报表接口的 `start_date`/`end_date`/`months` 参数 |
| FR-8.1 | 通知中心 | **已移除** |
| FR-8.2 | 通知类型 | **已移除** |
| FR-8.3 | 通知操作 | **已移除** |
| FR-8.4 | CSV 导出 | `GET /ledgers/{id}/export/csv` |
| FR-8.5 | Excel 导出 | `GET /ledgers/{id}/export/excel` |
| FR-8.6 | 全量 JSON 备份 | `GET /data/backup` |
| FR-8.7 | 备份还原 | `POST /data/restore` |

---

*文档结束 — 接口总计 48 个*

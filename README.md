# LedgerRS

一款基于 Rust + React 的全栈个人记账应用，支持多账本、分类管理、预算追踪和收支报表。

## 功能特性

- **多账本管理** — 创建、编辑、删除账本，支持多币种
- **交易记录** — 收入 / 支出 / 转账，按日期与分类筛选
- **分类管理** — 自定义收入/支出分类，支持预设批量导入
- **预算设置** — 按月/周/年设置分类预算，实时追踪超支
- **收支报表** — 趋势图、分类占比图表
- **JWT 认证** — 7 天访问令牌 + 30 天刷新令牌

## 技术栈

| 层级 | 技术 |
|------|------|
| 后端框架 | [Axum](https://github.com/tokio-rs/axum) (Rust) |
| 数据库 | PostgreSQL 14+ via [SQLx](https://github.com/launchbadge/sqlx) |
| 前端框架 | React 19 + TypeScript |
| 前端构建 | [Vite](https://vitejs.dev/) + Tailwind CSS 4 |
| 容器化 | Docker + Docker Compose |

## 项目结构

```
LedgerRS/
├── backend/           # Rust/Axum API 服务端
│   ├── src/
│   │   ├── handler/   # HTTP 路由处理器
│   │   ├── service/   # 业务逻辑层
│   │   ├── repository/# 数据库访问层
│   │   ├── middleware/ # JWT 鉴权中间件
│   │   └── model/     # 数据模型
│   └── tests/         # 集成测试
├── shared/            # 前后端共享 DTO 类型
├── migrations/        # PostgreSQL 迁移文件
├── ui/                # React/TypeScript 前端
│   └── src/
│       ├── api.ts     # API 客户端
│       ├── App.tsx    # 根组件与状态管理
│       ├── components/# 公共组件
│       └── pages/     # 页面组件
├── frontend/          # 原 Leptos/WASM 前端（已归档）
├── Dockerfile
├── docker-compose.yml
└── .env.example
```

## 快速开始

### 前置要求

- Rust stable（推荐通过 [rustup](https://rustup.rs/) 安装）
- Node.js 20+（推荐通过 [fnm](https://github.com/Schniz/fnm) 或 [nvm](https://github.com/nvm-sh/nvm) 安装）
- PostgreSQL 14+

### 1. 克隆仓库

```bash
git clone https://github.com/123xpw/LedgerRS.git
cd LedgerRS
```

### 2. 配置环境变量

```bash
cp .env.example .env
```

编辑 `.env`，填写实际值：

```env
DATABASE_URL=postgres://ledger:ledger_secret@localhost:5432/ledger_rs
JWT_SECRET=your_long_random_secret_here_at_least_32_chars
FRONTEND_DIST=/absolute/path/to/LedgerRS/ui/dist
```

### 3. 启动数据库

使用 Docker Compose（推荐）：

```bash
docker-compose up -d postgres
```

或手动创建 PostgreSQL 数据库并运行迁移：

```bash
createdb ledger_rs
# SQLx 会在后端启动时自动执行 migrations/ 下的迁移文件
```

### 4. 构建前端

```bash
cd ui
npm install
npm run build
cd ..
```

构建产物输出到 `ui/dist/`，确保 `.env` 中 `FRONTEND_DIST` 指向该路径。

### 5. 启动后端

```bash
# 标准方式
cargo run -p backend

# 若项目位于 NTFS/跨文件系统挂载点（如 /media/...），需指定编译目录以避免 Bus Error：
CARGO_TARGET_DIR=$HOME/ledger_rs_target cargo run -p backend -j1
```

服务默认监听 `http://localhost:8080`，同时提供前端静态文件服务。

---

## Docker 一键部署

```bash
# 构建镜像并启动全部服务
docker-compose up -d

# 查看日志
docker-compose logs -f backend
```

`docker-compose.yml` 包含 PostgreSQL 和后端服务，数据库数据通过 Docker Volume 持久化。

---

## 开发指南

### 后端开发

后端使用 SQLx 离线模式，`backend/.sqlx/` 目录已包含预编译的查询缓存，无需连接数据库即可编译：

```bash
# 仅编译，不启动
cargo build -p backend

# 运行集成测试（需要数据库）
DATABASE_URL=postgres://... cargo test -p backend
```

如需新增 SQL 查询，需在有数据库连接时执行：

```bash
cargo sqlx prepare --workspace
```

### 前端开发

前端开发服务器通过 Vite 代理将 `/api` 请求转发到后端：

```bash
cd ui
npm run dev   # 启动开发服务器，默认 http://localhost:5173
```

> **注意**：开发时需保持后端服务在 `localhost:8080` 运行。

### 代码组织约定

- 后端遵循 Handler → Service → Repository 三层架构
- API 错误统一返回格式：`{"error": {"code": "...", "message": "...", "details": null}}`
- 金额字段使用 `DECIMAL(20,6)` 存储，JSON 序列化为字符串（如 `"1000.000000"`），前端用 `parseFloat()` 解析
- 交易类型：`income` / `expense` / `transfer`
- 预算周期：`month` / `week` / `year`

---

## API 概览

所有接口前缀为 `/api/v1`，认证接口外均需在请求头携带 JWT：

```
Authorization: Bearer <access_token>
```

### 认证

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/auth/register` | 注册新用户 |
| POST | `/auth/login` | 登录，返回 access_token + refresh_token |
| POST | `/auth/logout` | 登出，撤销刷新令牌 |
| POST | `/auth/refresh` | 用 refresh_token 换取新 access_token |
| GET  | `/auth/me` | 获取当前用户信息 |
| PUT  | `/auth/me` | 更新用户名 / 默认币种 / 时区 |
| PUT  | `/auth/me/password` | 修改密码 |

### 账本

| 方法 | 路径 | 说明 |
|------|------|------|
| GET    | `/ledgers` | 获取所有账本 |
| POST   | `/ledgers` | 创建账本 |
| GET    | `/ledgers/:id` | 获取单个账本（含余额统计） |
| PUT    | `/ledgers/:id` | 更新账本 |
| DELETE | `/ledgers/:id` | 删除账本 |

### 交易记录

| 方法 | 路径 | 说明 |
|------|------|------|
| GET    | `/ledgers/:id/transactions` | 分页获取交易（`?page=1&per_page=50`） |
| POST   | `/ledgers/:id/transactions` | 创建交易 |
| DELETE | `/ledgers/:id/transactions/:tx_id` | 删除交易 |

### 分类 / 预算

```
GET|POST              /ledgers/:id/categories
DELETE                /ledgers/:id/categories/:cat_id

GET|POST              /ledgers/:id/budgets
DELETE                /ledgers/:id/budgets/:budget_id
```

---

## 常见问题

**Q: 启动时报 `Bus Error`**  
A: 项目位于 NTFS 挂载盘时，Rust 编译目标目录需在本地文件系统：
```bash
export CARGO_TARGET_DIR=$HOME/ledger_rs_target
```

**Q: 前端构建报 `Cannot find native binding`**  
A: `@tailwindcss/oxide` 要求 Node.js ≥ 20，请升级后重新安装依赖：
```bash
rm -rf node_modules package-lock.json && npm install
```

**Q: 登录后页面空白**  
A: 检查 `.env` 中 `FRONTEND_DIST` 是否指向 `ui/dist` 目录（绝对路径），确认已执行 `npm run build`。

---

## License

Apache-2.0

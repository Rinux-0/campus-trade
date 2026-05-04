# 校园二手交易平台数据库系统

在线访问网址：`http://<your-ip>:3000`

## 技术栈

- 后端：Rust Axum + sqlx + PostgreSQL
- 前端：Vue 3 + TypeScript + Vite + pnpm
- 版本控制：Jujutsu (jj) + GitHub
- 文档：Swagger UI（utoipa）
- 部署：Docker + Sealos 云平台

## 快速启动

### 通用前提

默认已安装 [Rust](https://rustup.rs) 和 [Node.js](https://nodejs.org)（含 pnpm）。


### Linux

**1. 准备 PostgreSQL**

```bash
# Arch
sudo pacman -S postgresql
sudo -u postgres initdb --locale=C.UTF-8 -D /var/lib/postgres/data
sudo systemctl enable --now postgresql

# Debian / Ubuntu
sudo apt install postgresql

# 设置密码并创建数据库
sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'postgres'"
sudo -u postgres createdb campus_trade
```

**2. 构建并启动**

```bash
cd ./frontend && pnpm install && pnpm run build
cd ../backend && cargo build --release
DATABASE_URL=postgres://postgres:postgres@localhost:5432/campus_trade \
FRONTEND_DIR=../frontend/dist \
./target/release/campus-trade
```

**3. 开发模式**（前后端热重载）

```bash
# 终端 1：后端
cd ./backend && cargo run

# 终端 2：前端（:5173，自动代理 /api 到 :3000）
cd ../frontend && pnpm install && pnpm dev
```

### Windows

**1. 准备 PostgreSQL**

```powershell
# 安装
winget install PostgreSQL.PostgreSQL
# 或从 https://www.postgresql.org/download/windows/ 下载安装

# 设置密码并创建数据库（在 PostgreSQL 安装目录的 bin\ 下或使用 SQL Shell）
psql -U postgres -c "ALTER USER postgres PASSWORD 'postgres'"
createdb -U postgres campus_trade
```

**2. 构建并启动**

```powershell
cd ./frontend; pnpm install; pnpm run build; cd ..
cd ../backend; cargo build --release
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/campus_trade"
$env:FRONTEND_DIR = "..\frontend\dist"
.\target\release\campus-trade.exe
```

**3. 开发模式**

```powershell
# 终端 1：后端
cd ./backend; cargo run

# 终端 2：前端
cd ../frontend; pnpm install; pnpm dev
```

### 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/campus_trade` | 数据库连接 |
| `FRONTEND_DIR` | `../frontend/dist` | 前端静态文件目录 |
| `PORT` | `3000` | HTTP 监听端口 |

生产部署时，后端直接托管前端静态文件，只需运行后端即可访问完整站点。

## 在线部署

项目通过 GitHub Actions 自动构建 Docker 镜像并推送至 Docker Hub，Sealos 云平台拉取镜像运行。

```
git push → GitHub Actions (CI/CD) → Docker Hub → Sealos 云平台
```

- **Docker 镜像**：`rinux0/campus-trade:latest`
- **数据库**：Sealos 托管 PostgreSQL
- **运行平台**：Sealos 云平台（国内免备案）

## 功能清单

### 数据管理

- 用户管理：新增（ID 自动分配）、编辑（姓名/手机）、注销（级联删除）
- 商品管理：新增、改价、删除、筛选（全部/未售/已售）
- 购买功能：事务保证一致性，防重复购买

### 查询功能

- **聚合统计**：商品总数、分类统计、平均价格、发布最多用户
- **数据库视图**：已售商品视图、未售商品视图（融入商品筛选）
- **自由查询**：状态/分类/卖家/买家/商品ID/价格区间/日期区间 自由组合

### API 端点

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/users` | 用户列表 |
| POST | `/api/users` | 新增用户 |
| PUT | `/api/users/{user_id}` | 修改用户 |
| DELETE | `/api/users/{user_id}` | 注销用户（级联删除） |
| GET | `/api/items` | 商品列表 |
| POST | `/api/items` | 新增商品 |
| PUT | `/api/items/{item_id}/price` | 修改价格 |
| DELETE | `/api/items/{item_id}` | 删除未售商品 |
| GET | `/api/orders` | 订单列表 |
| POST | `/api/orders/buy` | 购买商品（事务） |
| GET | `/api/queries/search` | 自由查询 |
| GET | `/api/queries/views/sold` | 已售视图 |
| GET | `/api/queries/views/unsold` | 未售视图 |
| GET | `/api/queries/count` | 商品总数 |
| GET | `/api/queries/category-count` | 分类统计 |
| GET | `/api/queries/avg-price` | 平均价格 |
| GET | `/api/queries/top-seller` | 发布最多用户 |

Swagger UI：`http://localhost:3000/api`

## 安全与并发

详见 [docs/security.md](docs/security.md)

## 数据库表结构

```sql
user (user_id PK, user_name, phone)
item (item_id PK, item_name, category, price, status, seller_id FK→user)
orders (order_id PK, item_id UNIQUE FK→item, buyer_id FK→user, order_date)
```

视图：
- `sold_items_view`：已售商品（含商品名 + 买家信息）
- `unsold_items_view`：未售商品

# 开发计划书

## 项目概述
校园二手交易平台数据库系统 — Vue3 + Axum + PostgreSQL

## 技术栈
- 前端: Vue3 + TypeScript + Vite + pnpm
- 后端: Rust Axum + sqlx + tokio
- 数据库: PostgreSQL
- 文档: Swagger (utoipa)

## 模块划分

```
database/
├── backend/          # Axum 后端
│   ├── src/
│   │   ├── main.rs       # 入口
│   │   ├── db.rs          # 数据库连接与迁移
│   │   ├── models.rs      # 数据模型
│   │   ├── handlers.rs    # 路由处理
│   │   └── routes.rs      # 路由定义
│   └── Cargo.toml
├── frontend/         # Vue3 前端
│   ├── src/
│   │   ├── App.vue
│   │   ├── router/
│   │   ├── views/         # 页面 (Home, Items, Users, Orders)
│   │   └── api/           # API 封装
│   └── package.json
└── docs/
```

## 分步计划

### Step 1 — 项目骨架初始化
- 创建 backend 目录，`cargo init`，配置 Cargo.toml 依赖 (axum, sqlx, tokio, utoipa, serde)
- 创建 frontend 目录，`pnpm create vite`，安装依赖 (vue-router, axios)
- 后端能启动 `Hello World`，前端能启动空白页

### Step 2 — 数据库设计与初始化
- 编写 SQL 迁移脚本 (3张表: user, item, orders)
- 编写 db.rs 连接 PostgreSQL，启动时自动迁移
- 编写数据模型 models.rs (User, Item, Order 结构体)
- 插入初始数据

### Step 3 — 后端 CRUD API (骨架最小集)
- 路由: `GET /api/users`, `GET /api/items`, `GET /api/orders`
- 路由: `POST /api/items` (新增商品)
- 路由: `PUT /api/items/:id` (修改价格)
- 路由: `DELETE /api/items/:id` (删除未售出商品)
- Swagger 文档

### Step 4 — 前端页面 (骨架最小集)
- 首页导航
- 用户列表页
- 商品列表页
- 订单列表页
- API 对接，展示数据库数据

### Step 5 — 查询功能 (六类)
- 基本查询: 未售出商品、价格>30、生活用品、u001的商品
- 连接查询: 已售商品+买家、订单详情、u001出售情况
- 聚合统计: 总数、分类统计、均价、最多发布用户
- 每个查询一个端点 / 前端对应展示

### Step 6 — 视图
- 创建已售商品视图
- 创建未售商品视图
- 提供查询端点

### Step 7 — 购买业务逻辑
- `POST /api/orders/buy` — 事务: 插入订单 + 更新status=1
- 防重复购买校验
- 前端购买按钮交互

### Step 8 — 安全与并发说明
- 文档 md 中撰写第八、九部分的简答

### Step 9 — 部署与验收
- 打包部署，确保在线可访问
- 截图、录视频
- 撰写项目说明文档

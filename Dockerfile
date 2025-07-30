# 阶段1：使用 cargo-chef 缓存依赖
FROM rust:1 AS chef
# 安装 cargo-chef 用于生成和使用依赖缓存方案
RUN cargo install cargo-chef
WORKDIR /app

# 阶段2：生成依赖缓存清单（recipe）
FROM chef AS planner
# 将项目源代码拷贝到构建环境
COPY . ./
# 生成依赖清单 recipe.json，以便后续层能复用依赖缓存
RUN cargo chef prepare --recipe-path recipe.json

# 阶段3：构建 dioxus-cli、前端静态资源与后端可执行文件
FROM chef AS builder

# 恢复并构建缓存的 Rust 依赖，加速后续构建
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 将项目源码拷贝到构建环境
COPY . ./
# 安装 dioxus-cli，避免使用不存在的安装脚本 URL
RUN cargo install cargo-binstall && cargo binstall dioxus-cli
# 构建前端静态资源（release 模式，位于 target/dx/web）
RUN dx build --platform web --release

# 拷贝构建好的前端静态文件到 public/（由 Server 程序统一托管）
FROM chef AS runtime
COPY --from=builder /app/target/dx/hotdog/release/web/ /usr/local/app

# 设置环境变量并暴露端口
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]

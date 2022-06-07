- [1. rustup](#1-rustup)
  - [1.1. 安装](#11-安装)
  - [1.2. 卸载](#12-卸载)
  - [1.3. 配置](#13-配置)
  - [1.4. 更新](#14-更新)
- [2. rustc](#2-rustc)
- [3. cargo](#3-cargo)
  - [3.1. new](#31-new)
    - [3.1.1. 创建可执行项目](#311-创建可执行项目)
    - [3.1.2. 创建库项目](#312-创建库项目)
  - [3.2. build / b](#32-build--b)
    - [3.2.1. debug 模式](#321-debug-模式)
    - [3.2.2. release 模式](#322-release-模式)
  - [3.3. run / r](#33-run--r)
  - [3.4. test / t](#34-test--t)
    - [3.4.1. 测试参数](#341-测试参数)
  - [3.5. 管理 crate](#35-管理-crate)
    - [3.5.1. publish](#351-publish)
    - [3.5.2. yank](#352-yank)
  - [3.6. install](#36-install)
- [4. rust-analyzer](#4-rust-analyzer)
  - [4.1. 补全](#41-补全)

# 1. rustup

## 1.1. 安装

1. 设置安装路径

   设置环境变量 RUSTUP_HOME 和 CARGO_HOME

   - RUSTUP_HOME 保存 rust 工具链相关信息

   - CARGO_HOME/bin 保存 rust 内置命令 rustc/cargo 与 cargo install 下载的可执行文件

   - CARGO_HOME/registry 保存第三方 crate

2. 配置 rust 工具链 (二选一)

   MSVC 和 MinGW 提供的 libraries 不同

   - MSVC: 使用 Visual Studio Installer 安装"C++ tools" 和 "Windows 10 SDK"两个组件
   - MinGW: 直接下载

3. 下载安装包

下载 [rustup-init.exe](https://www.rust-lang.org/tools/install) 并运行

## 1.2. 卸载

```shell
rustup self uninstall
```

## 1.3. 配置

1. 设置默认主机

   ```shell
   rustup set default-host x86_64-pc-windows-msvc
   rustup set default-host x86_64-pc-windows-gnu
   rustup set default-host i686-pc-windows-msvc
   rustup set default-host i686-pc-windows-gnu
   ```

2. 设置构建目标

   ```shell
   rustup target add x86_64-pc-windows-msvc
   rustup target add x86_64-pc-windows-gnu
   rustup target add i686-pc-windows-msvc
   rustup target add i686-pc-windows-gnu
   ```

## 1.4. 更新

```
rustup update
```

# 2. rustc

单文件编译

```shell
rustc main.rc
```

# 3. cargo

## 3.1. new

### 3.1.1. 创建可执行项目

```shell
cargo new project
```

1. project/src/main.rs
2. project/.gitignore
3. project/Cargo.lock
4. project/Cargo.toml

### 3.1.2. 创建库项目

```shell
cargo new --lib project
```

1. project/src/lib.rs
2. project/.gitignore
3. project/Cargo.lock
4. project/Cargo.toml

## 3.2. build / b

### 3.2.1. debug 模式

```shell
cargo build
```

### 3.2.2. release 模式

```shell
cargo build -r
```

## 3.3. run / r

```shell
cargo run
# cargo run -r
```

## 3.4. test / t

```shell
cargo test
```

### 3.4.1. 测试参数

| command                       | desription                          |
| ----------------------------- | ----------------------------------- |
| cargo test fn                 | run all tests with 'fn' in the name |
| cargo test -- --ignored       | run only the ignored tests          |
| cargo test -- --test-thread=1 | set the number of test threads to 1 |
| cargo test -- --show-output   | show the output of successful tests |

## 3.5. 管理 crate

### 3.5.1. publish

1. 增加元数据

   ```Cargo.toml
   [package]
   name = "guessing_game"
   version = "0.1.0"
   authors = ["Your Name <you@example.com>"]
   edition = "2018"
   description = "A fun game where you guess what number the computer has chosen."
   license = "MIT OR Apache-2.0"
   ```

2. 用 API key 登录并发布

   ```
   cargo login abcdefghijklmnopqrstuvwxyz012345
   cargo publish
   ```

### 3.5.2. yank

1. 冻结某个版本, 阻止未来的项目依赖该版本

```shell
cargo yank --vers 1.0.1
```

2. 取消冻结某个版本

```shell
cargo yank --vers 1.0.1 --undo
```

## 3.6. install

```
cargo install <crate>
```

# 4. rust-analyzer

## 4.1. 补全

| Before       | After                                |
| ------------ | ------------------------------------ |
| (expr).if    | "if expr {}" or "if let …​ {}"       |
| (expr).match | match expr {}                        |
| (expr).while | "while expr {}" or "while let …​ {}" |
| (expr).ref   | &expr                                |
| (expr).refm  | &mut expr                            |
| (expr).let   | let $0 = expr;                       |
| (expr).letm  | let mut $0 = expr;                   |
| (expr).not   | !expr                                |
| (expr).dbg   | dbg!(expr)                           |
| (expr).dbgr  | dbg!(&expr)                          |
| (expr).call  | (expr)                               |
| pd           | eprintln!(" = {:?}", );              |
| ppd          | eprintln!(" = {:#?}", );             |
| tfn          | #[test] fn feature(){}               |

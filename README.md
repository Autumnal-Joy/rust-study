- [1. rustup](#1-rustup)
  - [1.1. 安装](#11-安装)
    - [1.1.1. 设置安装路径](#111-设置安装路径)
    - [1.1.2. 配置 rust 工具链 (二选一)](#112-配置-rust-工具链-二选一)
    - [1.1.3. 下载安装包](#113-下载安装包)
  - [1.2. 卸载](#12-卸载)
  - [1.3. 配置](#13-配置)
    - [1.3.1. 设置默认主机](#131-设置默认主机)
    - [1.3.2. 设置构建目标](#132-设置构建目标)
  - [1.4. 更新](#14-更新)
- [2. rustc](#2-rustc)
- [3. cargo](#3-cargo)
  - [3.1. 构建项目结构](#31-构建项目结构)
    - [3.1.1. 构建可执行项目](#311-构建可执行项目)
    - [3.1.2. 构建库项目](#312-构建库项目)
  - [3.2. 编译](#32-编译)
    - [3.2.1. debug 模式](#321-debug-模式)
    - [3.2.2. release 模式](#322-release-模式)
  - [3.3. 编译并运行](#33-编译并运行)
- [4. rust-analyzer](#4-rust-analyzer)
  - [4.1. 补全](#41-补全)

# 1. rustup

## 1.1. 安装

### 1.1.1. 设置安装路径

设置环境变量 RUSTUP_HOME 和 CARGO_HOME

RUSTUP_HOME 保存 rust 工具链相关信息

CARGO_HOME/bin 保存 rust 内置命令 rustc 和 cargo 等文件

CARGO_HOME/registry 保存第三方 crate

### 1.1.2. 配置 rust 工具链 (二选一)

MSVC 和 MinGW 提供的 libraries 不同

1. MSVC: 使用 Visual Studio Installer 安装"C++ tools" 和 "Windows 10 SDK"两个组件
2. MinGW: 直接下载

### 1.1.3. 下载安装包

下载 [rustup-init.exe](https://www.rust-lang.org/tools/install) 并运行

## 1.2. 卸载

```shell
rustup self uninstall
```

## 1.3. 配置

### 1.3.1. 设置默认主机

```shell
rustup set default-host x86_64-pc-windows-msvc
rustup set default-host x86_64-pc-windows-gnu
rustup set default-host i686-pc-windows-msvc
rustup set default-host i686-pc-windows-gnu
```

### 1.3.2. 设置构建目标

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

## 3.1. 构建项目结构

### 3.1.1. 构建可执行项目

```shell
cargo new project
```

1. project/src/main.rs
2. project/.gitignore
3. project/Cargo.lock
4. project/Cargo.toml

### 3.1.2. 构建库项目

```shell
cargo new --lib project
```

1. project/src/lib.rs
2. project/.gitignore
3. project/Cargo.lock
4. project/Cargo.toml

## 3.2. 编译

### 3.2.1. debug 模式

```shell
cargo build
```

### 3.2.2. release 模式

```shell
cargo build -r
```

## 3.3. 编译并运行

```shell
cargo run
# cargo run -r
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

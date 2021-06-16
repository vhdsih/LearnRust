---
title: Learning Rust
date: 2021-06-16 00:39:24
tags:
    - rust
    - language
    - learn-note
categories: Language
toc: true
---

闲着没事干，学学 rust，另外计划用 rust 写写算法~，此篇笔记为阅读  [rust online book](https://doc.rust-lang.org/book/) 时记录的，希望能坚持看完，在此给予自己最大的鼓励！

<!-- more -->
# Learning Rust

Rust具有安全高效等语言特性，提供了3个工具：

- cargo: 依赖管理和构建工具
- rustfmt: 代码风格
- Rust Language Server

## 第一份程序

``` rust
// main.rs
fn main() {
    println!("hello, world!");
}
```

对于这个起点程序，和C语言类似，main 函数是 rust 程序的入口，函数体使用 '{}' 包围，将左大括号和函数声明放在一行是 rust 推荐的编程习惯,此外，rust 语言风格使用 4 个空格来缩进，而非 TAB，且使用分号作为每个语句表达的结束。可以使用 rustfmt 工具格式化代码为官方推荐的格式

``` shell
> rustfmt main.rs
```

和c语言有所区别的是，用于输出的语句 "println!" 非函数，而是 Rust macro，它和函数的直观上的区别在于是否有 "!"，若 "func_name" 则为一个普通的函数

对于简单的 rust 程序，可以使用 rustc 进行编译，并得到可运行的二进制文件

``` shell
> rustc main.rs
```

然而对于复杂的工程，使用 cargo 来管理项目则是更好的选择。

## Cargo

Cargo 是 Rust 语言系统中的依赖管理和构建工具。利用cargo创建新的项目，并创建所需的文件项，同时在非 git 仓库中将同时初始化 git 并添加 gitignore 文件。

``` shell
> cargo new hello_cargo
> tree hello_cargo
  |- Cargo.toml
  |- src
    |- main.rs

```
在 Cargo.toml 文件中记录了项目相关信息和依赖项目，文件为TOML (Tom’s Obvious, Minimal Language) 格式。Cargo 设计希望将源码放到 src目录中，顶层目录放置 README、LICENSE 等。

构建和运行使用 Cargo 创建的项目很简单，在第一次构建完成后，在顶层目录将创建一个 Cargo.lock 文件用以记录依赖，无需手工管理。

``` shell
> cd hello_cargo
# write binary file into target/
> cargo build 
# exec the program
> cargo run
# or exec the binary file direcly
> ./target/debug/hello_cargo
```

此外 Cargo 提供了快速检测代码但不生成二进制文件的命令，其速度快于 build，所以经常 check 一下刚刚写的代码是个很好的习惯。 

``` shell
> cargo check
```

当程序发版使用时，使用带有 --release 参数的 build 命令生成 release 版本，编译器优化将使程序具备更高效的运行效率，但是编译时间会更长。

``` shell
> cargo build --release
```

## 猜数程序

实践永远是学习新东西最快的方法。下面使用熟知的猜数游戏学习一些新的语言规则。


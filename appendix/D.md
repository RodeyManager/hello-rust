# 附录 D：实用开发工具

本附录，我们将讨论 Rust 项目提供的用于开发 Rust 代码的工具。

## 通过 `rustfmt` 自动格式化

`rustfmt` 工具根据社区代码风格格式化代码。很多项目使用 `rustfmt` 来避免编写 Rust 风格的争论：所有人都用这个工具格式化代码！

安装 `rustfmt`：

```powershell
$ rustup component add `rustfmt`
```

这会提供 `rustfmt` 和 `cargo-fmt`，类似于 Rust 同时安装 `rustc` 和 `cargo`。为了格式化整个 Cargo 项目：

```powershell
$ cargo fmt
```

运行此命令会格式化当前 crate 中所有的 Rust 代码。这应该只会改变代码风格，而不是代码语义。请查看 [该文档](https://github.com/rust-lang-nursery/rustfmt) 了解 `rustfmt` 的更多信息。

## 通过 rustfix 修复代码

如果你编写过 Rust 代码，那么你可能见过编译器警告。例如，考虑如下代码：

文件名: `src/main.rs`

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

这里调用了 `do_something` 函数 100 次，不过从未在 for 循环体中使用变量 i。Rust 会警告说：

```powershell
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

警告中建议使用 `_i` 名称：下划线表明该变量有意不使用。我们可以通过 `cargo fix` 命令使用 `rustfix` 工具来自动采用该建议：

```powershell
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

如果再次查看 `src/main.rs`，会发现 `cargo fix` 修改了代码：

文件名: `src/main.rs`

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

现在 `for` 循环变量变为 `_i`，警告也不再出现。

`cargo fix` 命令可以用于在不同 Rust 版本间迁移代码。版本在[附录 E](E.md) 中介绍。

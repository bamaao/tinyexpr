# tinyexpr

一个无第三方依赖的 Rust 算术表达式解析与求值器，使用词法分析 + 递归下降解析 + AST 求值。

## 支持范围

- `+ - * / ^`
- 圆括号
- 一元正负号
- 整数、小数（如 `12`、`3.14`、`.5`、`3.`）
- 优先级、结合性与错误定位

约定：
- `^` 右结合：`2^3^2 = 2^(3^2)`
- `* /`、`+ -` 左结合
- 指数高于前置一元负号：`-2^2 = -(2^2)`
- 允许负指数：`2^-3`

## 上下文无关文法

与 `parser.rs` 中的递归下降层次一致（`{ … }` 表示 0 次或多次，`[ … ]` 表示可选）：

```text
Expr           ::= Additive
Additive       ::= Multiplicative { ("+" | "-") Multiplicative }
Multiplicative ::= Unary { ("*" | "/") Unary }
Unary          ::= ("+" | "-") Unary
                 | Power
Power          ::= Primary [ "^" Unary ]
Primary        ::= Number
                 | "(" Additive ")"
```

`Number` 由词法分析产生（整数或小数）。`Power` 的右操作数取 `Unary`，使 `^` 右结合，并允许 `2^-3`，同时保持 `-2^2` 解析为 `-(2^2)`。

## 运行

```bash
cargo test
cargo run
```

示例：

```text
tinyexpr REPL (输入表达式计算，输入 exit 退出)
1 + 2 * 3
7
2 ^ 3 ^ 2
512
-2 ^ 2
-4
2 ^ -3
0.125
exit
```

## 库调用

```rust
use tinyexpr::{eval, parse, evaluate, Environment};

fn main() {
    let ast = parse("2 ^ 3 + 4 * (5 - 1)").unwrap();
    let value = evaluate(&ast, &Environment::new()).unwrap();
    println!("{value}");

    println!("{}", eval("-2^2").unwrap());
}
```

## 模块

- `token.rs`：Token 与源码跨度
- `lexer.rs`：词法分析
- `ast.rs`：AST 类型
- `parser.rs`：递归下降解析器
- `evaluator.rs`：AST 求值
- `error.rs`：词法、语法、求值错误
- `main.rs`：命令行 REPL

## 当前版本边界

当前仅实现数值字面量，不包含变量、函数调用、科学计数法、隐式乘法或比较/逻辑运算。`Environment` 已预留，后续增加标识符 Token 后即可接入变量求值。

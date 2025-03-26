# Geektime Rust 语言训练营

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

在我们的课程中，新的项目会使用 `tyr-rust-bootcamp/template` 模版生成基本的代码：

```bash
cargo generate tyr-rust-bootcamp/template
```

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```

- struct/structure: 结构体
- enum: 枚举
- variable: 变量
- constant: 常量
- static: 静态变量
- function: 函数
- method: 方法
- generics: 泛型
- trait: 特征/特质
- trait bound: 特征约束/trait 约束


数据结构-自定义类型
- 原生类型
  - array
  - bool
  - f32/f64
  - fn
  - i8/i16/i32/i64/i128/isize
  - u8/u16/u32/u64/u128/usize
  - pointer
  - reference
  - slice
  - str
  - tuple
  - unit
  - never
- 组合类型
  - Box<T>
  - Option<T>
  - Result<T, E>
  - Vec<T>
  - String
  - HashMap<K, V>/BTreeMap<K, V>
  - HashSet<T>/BTreeSet<T>
  - Cell<T>/RefCell<T>
  - Rc<T>/Arc<T>
  - Mutex<T>/RwLock<T>

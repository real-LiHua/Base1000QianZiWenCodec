# Base1000 千字文编码器

Base1000 是一个基于《千字文》的文本编码器，支持将任意文本编码为《千字文》字符序列，并支持解码回原始文本。

## 功能

- **编码**: 将输入文本编码为《千字文》字符序列。
- **解码**: 将《千字文》字符序列解码回原始文本。
- **Python 扩展模块**: 提供 Python 接口，支持直接在 Python 中调用。
- **命令行工具**: 提供简单易用的 CLI 工具。

## 安装

### 使用 Cargo 构建

```bash
cargo build --release
```

### 使用 Maturin 构建 Python 扩展

```bash
maturin build --release
```

## 使用方法

### 命令行工具

#### 编码

```bash
base1000 -e "114514"
```

#### 解码

```bash
base1000 -d "夜裳移柰梧"
```

### Python 扩展

#### 安装

```bash
pip install .
```

#### 示例

```python
from base1000 import base1000

# 编码
encoded = base1000.encode("114514")
print(encoded)

# 解码
decoded = base1000.decode(encoded)
print(decoded)
```

## 测试

运行单元测试：

```bash
cargo test
```

## 项目结构

- `src/lib.rs`: 核心库实现。
- `src/main.rs`: 命令行工具入口。
- `base1000/__main__.py`: Python CLI 实现。
- `base1000/base1000.pyi`: Python 类型提示文件。

# Base1000QianZiWenCodec
一个基于千字文的编码和解码工具。它将文本转换为千字文字符的组合，并支持从千字文字符组合还原原始文本。

## 功能

- **编码**: 将任意文本转换为千字文字符组合。
- **解码**: 从千字文字符组合还原原始文本。

## 使用方法

### 编译和运行

1. 确保已安装 [Rust](https://www.rust-lang.org/) 工具链。
2. 克隆项目并进入目录：
   ```bash
   git clone https://github.com/real-LiHua/Base1000QianZiWenCodec
   cd Base1000QianZiWenCodec
   ```
3. 编译项目：
   ```bash
   cargo build -r
   ```
4. 运行程序：
   ```bash
   ./target/release/base1000 --help
   ```

### 命令行用法

#### 编码文本
```bash
./target/release/base1000 --encode "Hello, world!"
```

#### 解码文本
```bash
./target/release/base1000 --decode "夜裳移柰梧"
```

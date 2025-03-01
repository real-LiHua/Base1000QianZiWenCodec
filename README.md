# Base1000QianZiWenCodec

## 简介
Base1000QianZiWenCodec 是一个用于编码和解码文本的工具，主要应用于《千字文》的文本处理。该项目利用字符元组进行编码与解码，支持从文本文件加载字符数据。

## 功能
• **文本编码**：将输入文本编码为特殊字符的字符串。  
• **文本解码**：将编码后的字符串解码回原始文本。

## 安装
确保 Python 环境已安装，项目依赖标准库，无需额外安装。

## 使用方法

### 编码
使用命令行参数 -e 进行文本编码：
```bash
python Base1000.py -e "待编码的文本"
```

### 解码
使用命令行参数 -d 进行文本解码：
```bash
python Base1000.py -d "编码后的字符串"
```

## 文件结构
• 千字文/：包含用于编码解码的文本文件。  
• 千字文/cache.json：缓存文件，存储字符元组。

## 代码示例
以下是如何使用编码和解码的示例：
```python
import Base1000

# 编码示例
encoded = Base1000.encode("元文本")
print(encoded)

# 解码示例
decoded = list(Base1000.decode(encoded))
print(decoded)
```

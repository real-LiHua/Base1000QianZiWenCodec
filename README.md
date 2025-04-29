# Base1000 Thousand Character Classic Encoder

Base1000 is a text encoder based on the "Thousand Character Classic", supporting encoding any text into a sequence of "Thousand Character Classic" characters and decoding it back to the original text.

[![Static Badge](https://img.shields.io/badge/DeepWiki-blue)](https://deepwiki.com/real-LiHua/Base1000QianZiWenCodec)


## Features

- **Encoding**: Encode input text into a sequence of "Thousand Character Classic" characters.
- **Decoding**: Decode a sequence of "Thousand Character Classic" characters back to the original text.
- **Python Extension Module**: Provides a Python interface for direct usage in Python.
- **Command Line Tool**: Offers a simple and easy-to-use CLI tool.

## Installation

### Build with Cargo
Default enabled features (`clap`, `encode`, `decode`)

```bash
cargo build --release
```

#### Enable Features

- **`clap`**: Enables command-line tool functionality. Enable it with the following command:
  ```bash
  cargo build --release --features clap
  ```

- **`pyo3`**: Enables building the Python extension module. Enable it with the following command:
  ```bash
  cargo build --release --features pyo3
  ```

- **`encode`**: Enables text encoding functionality. Enable it with the following command:
  ```bash
  cargo build --release --features encode
  ```

- **`decode`**: Enables text decoding functionality. Enable it with the following command:
  ```bash
  cargo build --release --features decode
  ```

- Enable multiple features simultaneously:
  ```bash
  cargo build --release --features "clap pyo3 encode decode"
  ```

### Build Python Extension with Maturin

```bash
maturin build --release
```

## Usage

### Command Line Tool

#### Encoding

```bash
base1000 -e "114514"
```

#### Decoding

```bash
base1000 -d "夜裳移柰梧"
```

### Python Extension

#### Installation

```bash
pip install .
```

#### Example

```python
from base1000 import base1000

# Encoding
encoded = base1000.encode("114514")
print(encoded)

# Decoding
for decoded in base1000.decode(encoded):
    print(decoded)
```

## Testing

Run unit tests:

```bash
cargo test
```

## Project Structure

- `src/lib.rs`: Core library implementation.
- `src/main.rs`: Command-line tool entry point.
- `base1000/__main__.py`: Python CLI implementation.
- `base1000/base1000.pyi`: Python type hint file.

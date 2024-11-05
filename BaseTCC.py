import argparse
import re
from itertools import product, zip_longest
from pathlib import Path
from random import choice
from typing import Generator, List, Set, Tuple

# 存储每个文件的内容
file_contents: List[str] = []

# 遍历指定目录下的所有文本文件并读取内容
for txt_file_path in Path("千字文").glob("*.txt"):
    with open(txt_file_path, encoding="utf-8") as file:
        # 去除文本中的空格、逗号和句号
        cleaned_content = re.sub(r"\s|，|。", "", file.read())
        file_contents.append(cleaned_content)  # 添加清理后的内容

# 创建字符的元组列表，以便编码和解码
str_tuples: Tuple[Tuple[str, ...]] = tuple(
    map(tuple, map(set, zip_longest(*file_contents)))
)


def encode(input_text: str) -> str:
    """将输入文本编码为字符串"""
    encoded_chars: List[str] = []  # 存储编码后的结果
    # 文件内容的数量
    num_files: int = len(file_contents)
    # 将输入文本转换为字节并转为整数
    byte_representation: str = "00" + str(int.from_bytes(input_text.encode(), "big"))

    while len(byte_representation) > 2:
        # 从随机文件中选择字符并追加到结果中
        encoded_char: str = choice(str_tuples[int(byte_representation[-3:])])
        encoded_chars.append(encoded_char)
        byte_representation = byte_representation[:-3]  # 逐步减少字节表示

    return "".join(encoded_chars[::-1])  # 返回反转后的字符串


def decode(encoded_string: str) -> Generator[str, None, None]:
    """将编码字符串解码为原始文本"""
    index_sets: List[Set[int]] = []  # 存储字符对应的索引集合

    # 遍历编码字符串中的每个字符
    for character in encoded_string:
        # 找到每个字符在所有文本中的位置
        indices: Set[int] = set(k for k, v in enumerate(str_tuples) if character in v)
        index_sets.append(indices)  # 将索引添加到集合中

    # 生成解码结果
    for index_combination in product(*index_sets):
        # 将索引组合转换为十六进制字节，并解码为字符串
        combined_index: int = int("".join(f"{idx:03d}" for idx in index_combination))
        try:
            yield combined_index.to_bytes(
                (combined_index.bit_length() + 7) >> 3
            ).decode()
        except UnicodeDecodeError:
            pass  # 解码错误则跳过


def main() -> None:
    """主程序入口"""
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group()
    group.add_argument("-e", "--encode", action="store_true", help="编码输入文本")
    group.add_argument("-d", "--decode", action="store_true", help="解码输入文本")
    parser.add_argument("text", help="需要编码或解码的文本")
    args = parser.parse_args()

    # 根据命令行参数选择编码或解码
    if args.encode:
        print(encode(args.text))
    else:
        print(*decode(args.text), sep="\n")


if __name__ == "__main__":
    main()

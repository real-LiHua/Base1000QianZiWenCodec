import re
import sys
from itertools import product
from pathlib import Path
from random import randrange
from typing import Generator, List, Set

# 存储每个文件的内容
file_contents: List[str] = []
# 遍历目录下的所有文本文件
for file_path in Path("千字文").glob("*.txt"):
    with open(file_path, encoding="utf-8") as file:
        # 去除文本中的空格、逗号和句号
        cleaned_content = re.sub(r"\s|，|。", "", file.read())
        file_contents.append(cleaned_content)


def encode(input_text: str) -> str:
    # 存储编码后的结果
    encoded_result = []
    # 文件内容的数量
    file_count = len(file_contents)
    # 将输入文本转换为字节并转为整数
    numeric_representation = "00" + str(int.from_bytes(input_text.encode()))
    while len(numeric_representation) > 2:
        # 从随机文件中选择字符
        encoded_result.append(
            file_contents[randrange(file_count)][int(numeric_representation[-3:])]
        )
        numeric_representation = numeric_representation[:-3]
    return "".join(encoded_result[::-1])  # 返回反转后的字符串


def decode(encoded_str: str) -> Generator[str, None, None]:
    # 存储字符对应的索引集合
    index_sets: List[Set[int]] = []
    # 遍历编码字符串中的每个字符
    for character in encoded_str:
        # 找到每个字符在所有文本中的位置
        indices = set(
            match.start()
            for text in file_contents
            for match in re.finditer(character, text)
        )
        index_sets.append(indices)  # 将索引添加到集合中

    # 生成解码结果
    for index_combination in product(*index_sets):
        # 将索引组合转换为十六进制字节，并解码为字符串
        combined_index = int("".join(f"{idx:03d}" for idx in index_combination))
        try:
            yield combined_index.to_bytes(
                (combined_index.bit_length() + 7) >> 3
            ).decode()
        except UnicodeDecodeError:
            pass  # 解码错误则跳过


if __name__ == "__main__":
    print(*decode(sys.argv[1]), sep="\n")

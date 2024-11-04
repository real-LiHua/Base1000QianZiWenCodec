import re
import sys
from itertools import product
from pathlib import Path

# 存储每个文件的内容
text_contents = []
for file_path in Path("千字文").glob("*.txt"):
    with open(file_path, encoding="utf-8") as file:
        # 去除文本中的空格、逗号和句号
        cleaned_content = re.sub(r"\s|，|。", "", file.read())
        text_contents.append(cleaned_content)


def decode(encoded_text: str) -> str:
    # 存储字符对应的索引集合
    index_sets = []
    for character in encoded_text:
        # 找到每个字符在所有文本中的位置
        indices = set(
            match.start()
            for text in text_contents
            for match in re.finditer(character, text)
        )
        index_sets.append(indices)

    # 生成解码结果
    for combination in product(*index_sets):
        # 将索引组合转换为十六进制字节，并解码为字符串
        combined_index = int("".join(f"{idx:03d}" for idx in combination))
        try:
            yield combined_index.to_bytes(
                (combined_index.bit_length() + 7) >> 3, byteorder="big"
            ).decode()
        except UnicodeDecodeError:
            pass


if __name__ == "__main__":
    print(*decode(sys.argv[1]), sep="\n")

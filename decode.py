import re
import sys
from pathlib import Path

strList = []
for path in Path("千字文").glob("*.txt"):
    with open(path) as f:
        strList.append(re.sub(r"\s|，|。", "", f.read()))
    path.write_text(strList[-1])


def decode(text: str) -> str:
    c = []
    for i in text:
        for 千字文 in strList:
            r = 千字文.find(i)
            if r != -1:
                break
        c.append(r)
    return bytes.fromhex(hex(int("".join(f"{i:03d}" for i in c)))[2::]).decode()


if __name__ == "__main__":
    print(decode(sys.argv[1]))
import argparse as __argparse

import base1000
from base1000.base1000 import __all__

if 'encode' in __all__:
    from base1000.base1000 import encode

if 'decode' in __all__:
    from base1000.base1000 import decode
del base1000

def _main():
    parser = __argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group()

    if 'encode' in __all__:
        group.add_argument("-e", "--encode", action="store_true", help="编码输入文本")

    if 'decode' in __all__:
        group.add_argument("-d", "--decode", action="store_true", help="解码输入文本")

    parser.add_argument("text", help="需要编码或解码的文本")
    args = parser.parse_args()

    if 'encode' in __all__ and args.encode:
        print(encode(args.text))
    elif 'decode' in __all__:
        for res in decode(args.text):
            print(res)

if __name__ == "__main__":
    _main()

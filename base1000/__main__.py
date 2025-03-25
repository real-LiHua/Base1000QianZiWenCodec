import argparse

from base1000 import base1000

parser = argparse.ArgumentParser()
group = parser.add_mutually_exclusive_group()

if 'encode' in base1000.__all__:
    group.add_argument("-e", "--encode", action="store_true", help="编码输入文本")

if 'decode' in base1000.__all__:
    group.add_argument("-d", "--decode", action="store_true", help="解码输入文本")

parser.add_argument("text", help="需要编码或解码的文本")
args = parser.parse_args()

if 'encode' in base1000.__all__ and args.encode:
    print(base1000.encode(args.text))
elif 'decode' in base1000.__all__:
    for res in base1000.decode(args.text):
        print(res)

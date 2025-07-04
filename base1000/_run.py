from base1000 import base1000
import argparse

def cli():
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group()

    if 'encode' in base1000.__all__:
        group.add_argument("-e", "--encode", action="store_true", help="编码输入文本")

    if 'decode' in base1000.__all__:
        group.add_argument("-d", "--decode", action="store_true", help="解码输入文本")

    parser.add_argument("text", nargs='?', help="需要编码或解码的文本")

    args = parser.parse_args()

    if 'encode' in base1000.__all__ and args.encode:
        print(base1000.encode(args.text))
    elif 'decode' in base1000.__all__ and args.decode:
        for res in base1000.decode(args.text):
            print(res)
    else:
        from mcp.server.fastmcp import FastMCP
        mcp = FastMCP("base1000")

        if 'encode' in base1000.__all__:
            @mcp.tool()
            def encode(text: str) -> str:
                """ Encodes the given text into a string using the "Thousand Character Classic" character matrix.
                :param text: The input text to encode.
                :return: A string representing the encoded text.
                """
                return base1000.encode(text)

        if 'decode' in base1000.__all__:
            @mcp.tool()
            def decode(text: str) -> list[str]:
                """ Decodes the given text into an iterator of possible original strings.
                :param text: The encoded text to decode.
                :return: An iterator over possible decoded strings.
                """
                return list(base1000.decode(text))

        mcp.run()

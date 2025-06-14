#import pytest
from base1000 import base1000

def test_encode():
    assert base1000.encode("114514") == "夜裳移柰梧"

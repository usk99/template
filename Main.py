def main():
    A = InputNum()
    B = InputNums(2)
    C = InputsNums(3, 2)

    D = InputString()
    E = InputStringLine()
    F = InputStrings(2)
    G = InputStringLines(2)
    H = InputsStrings(3, 2)

    print(A)
    print(B)
    print(C)
    print(D)
    print(E)
    print(F)
    print(G)
    print(H)


# いろいろ-------------------------------------------------------------
import sys, typing
from functools import singledispatch
from math import isqrt
from collections import deque
# import array, bisect, collections, copy, heapq, itertools, math, numpy, string
# from typing import overload

DY4 = [0, 1, 0, -1]
DX4 = [1, 0, -1, 0]  # (右,下,左,上）
DY8 = [0, 1, 0, -1, -1, 1, 1, -1]
DX8 = [1, 0, -1, 0, 1, 1, -1, -1]  # (右,下,左,上,右上,右下,左下,左上）
ABC = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
abc = "abcdefghijklmnopqrstuvwxyz"


class Scanner:
    _int: int = 0
    _str: typing.List[str] = []

    def Next(self) -> str:
        if self._int < len(self._str):
            self._int += 1
            return self._str[self._int - 1]

        st = input().rstrip()

        while st == "":
            st = input().rstrip()

        if st is not None:
            self._str = st.split(' ')

        if len(self._str) == 0:
            return self.Next()

        self._int = 1

        return self._str[self._int - 1]

    def NextLine(self) -> str:
        if self._int < len(self._str):
            st = " " + " ".join(self._str[self._int:len(self._str)])
            self._str = []
            return st

        st = input().rstrip()

        while st == "":
            st = input().rstrip()

        self._str = [st]
        self._int = 1
        return self._str[self._int - 1]

    def NextLong(self) -> int:
        return int(self.Next(sc))

    def NextDouble(self) -> float:
        return float(self.Next(self))


# UnionFind
class UnionFind():
    # 初期化
    def __init__(self, n):
        self.parent = [-1] * n
        self.siz = [1] * n

    # 根を求める
    def root(self, x):
        if self.parent[x] == -1:
            return x  # x が根の場合は x を返す
        else:
            self.parent[x] = self.root(self.parent[x])  # 経路圧縮
            return self.parent[x]

    # x と y が同じグループに属するか (根が一致するか)
    def isSame(self, x, y):
        return self.root(x) == self.root(y)

    # x を含むグループと y を含むグループを併合する
    def unite(self, x, y):
        # x 側と y 側の根を取得する
        rx = self.root(x)
        ry = self.root(y)
        if rx == ry:
            return False  # すでに同じグループのときは何もしない
        # union by size
        if self.siz[rx] < self.siz[ry]:  # ry 側の rank が小さくなるようにする
            rx, ry = ry, rx
        self.parent[ry] = rx  # ry を rx の子とする
        self.siz[rx] += self.siz[ry]
        return True

    # x を含む根付き木のサイズを求める
    def size(self, x):
        return self.siz[self.root(x)]


# 標準入力
sc = Scanner
def InputNum(): return sc.NextLong(sc)
def InputNums(n: int): return [InputNum() for _ in range(n)]
def InputNumsMap(): return map(int, input().rstrip().split())
def InputsNums(h: int, w: int): return [InputNums(w) for _ in range(h)]
def InputString(): return sc.Next(sc)
def InputStringLine(): return sc.NextLine(sc)
def InputStrings(n: int): return [InputString() for _ in range(n)]
def InputStringLines(n: int): return [InputStringLine() for _ in range(n)]
def InputsStrings(h: int, w: int): return [InputStrings(w) for _ in range(h)]


# 標準出力
@singledispatch
def Output(*arg, s: str = " ", e: str = "\n"): print(*arg, sep=s, end=e)
@Output.register(bool)
def _(arg): print("Yes" if arg else "No")
@Output.register(list)
def _(arg, s: str = " "): print(s.join(map(str, arg)))


if __name__ == "__main__":
    main()

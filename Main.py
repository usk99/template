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
# import array, bisect, collections, copy, heapq, itertools, math, numpy, string
# from typing import overload

DY = [0, 1, 0, -1]
DX = [1, 0, -1, 0]  # (右,下,左,上）
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

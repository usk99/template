using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.RegularExpressions;
using static System.Console;
using static System.Math;
using static Contest.PrimeNumber;

namespace Contest
{
    internal class Program
    {
        private static void Solve()
        {
            var A = InputNum();
            var B = InputNums(2);
            var C = InputsNums(3, 2);
            var D = InputString();
            var E = InputStringLine();
            var F = InputStrings(2);
            var G = InputStringLines(2);
            var H = InputsStrings(3, 2);

            Output(A);
            Output(B);
            Output(C);
            Output(D);
            Output(E);
            Output(F);
            Output(G);
            Output(H);
            return;
        }
        static void Main(string[] args)
        {
            var sw = new StreamWriter(OpenStandardOutput()) { AutoFlush = false };
            SetOut(sw);
            Solve();
            Out.Flush();
        }
        static readonly Scanner sc = new Scanner();
        static long InputNum() { return sc.NextLong(); }
        static List<long> InputNums(long n)
        {
            var li = new List<long>();
            for (var i = 0; i < n; i++) li.Add(InputNum());
            return li;
        }
        static List<List<long>> InputsNums(long h, long w)
        {
            var li = new List<List<long>>();
            for (var i = 0; i < h; i++)
                li.Add(InputNums(w));
            return li;
        }
        static string InputString() { return sc.Next(); }
        static string InputStringLine() { return sc.NextLine(); }
        static List<string> InputStrings(long n)
        {
            var li = new List<string>();
            for (var i = 0; i < n; i++) li.Add(InputString());
            return li;
        }
        static List<string> InputStringLines(long n)
        {
            var li = new List<string>();
            for (var i = 0; i < n; i++) li.Add(InputStringLine());
            return li;
        }
        static List<List<string>> InputsStrings(long h, long w)
        {
            var li = new List<List<string>>();
            for (var i = 0; i < h; i++) li.Add(InputStrings(w));
            return li;
        }
        static void Output(object n) => WriteLine(n.ToString());
        static void Output(string n) => WriteLine(n);
        static void Output(bool n, string t = "Yes", string f = "No") => WriteLine(n ? t : f);
        static void Output<T>(IEnumerable<T> ts, string sep = " ") => WriteLine(string.Join(sep, ts));
    }

    #region いろいろ
    public static class Methods
    {
        static readonly int[] DY = [0, 1, 0, -1];
        static readonly int[] DX = [1, 0, -1, 0];   // (右,下,左,上)
        const string ABC = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const string abc = "abcdefghijklmnopqrstuvwxyz";

        /// <summary>
        /// 型変換
        /// </summary>
        #region ToNum
        public static int ToInt(this object target)
        {
            _ = int.TryParse(target.ToString(), out var val);
            return val;
        }
        public static long ToLong(this object target)
        {
            _ = long.TryParse(target.ToString(), out var val);
            return val;
        }
        public static byte ToByte(this object target)
        {
            _ = byte.TryParse(target.ToString(), out var val);
            return val;
        }
        public static float ToFloat(this object target)
        {
            _ = float.TryParse(target.ToString(), out var val);
            return val;
        }
        public static double ToDouble(this object target)
        {
            _ = double.TryParse(target.ToString(), out var val);
            return val;
        }
        public static decimal ToDecimal(this object target)
        {
            _ = decimal.TryParse(target.ToString(), out var val);
            return val;
        }
        #endregion

        /// <summary>
        /// 文字配列から文字列
        /// </summary>
        #region CharsToString
        public static string CharsToString(this IEnumerable<char> n)
        {
            return string.Join("", n);
        }
        #endregion

        /// <summary>
        /// 間に含まれているかどうか
        /// </summary>
        /// <param name="isInclude">true:以上以下,false:超過未満</param>
        #region Between
        public static bool Between<T>(this T target, T item1, T item2, bool isInclude = true) where T : IComparable<T>
        {
            var low = item1;
            var high = item2;

            if (low.CompareTo(high) > 0)
            {
                low = item2;
                high = item1;
            }

            if (isInclude)
                return target.CompareTo(low) >= 0 && target.CompareTo(high) <= 0;

            else
                return target.CompareTo(low) > 0 && target.CompareTo(high) < 0;
        }
        #endregion

        /// <summary>
        /// 交換(値型,構造体のみ)
        /// </summary>
        #region Swap
        public static void Swap<T>(ref T target, ref T item) where T : struct
        {
            (target, item) = (item, target);
        }
        #endregion

        /// <summary>
        /// 順列列挙
        /// </summary>
        /// <param name="withRepetition">各要素自身の重複</param>
        #region Permutation
        public static IEnumerable<T[]> Permutation<T>(this IEnumerable<T> items, int k, bool withRepetition = false)
        {
            if (k == 1)
            {
                foreach (var item in items)
                {
                    yield return new T[] { item };
                }
                yield break;
            }
            foreach (var item in items)
            {
                var leftside = new T[] { item };
                var unused = withRepetition ? items : items.Except(leftside);
                foreach (var rightside in Permutation(unused, k - 1, withRepetition))
                {
                    yield return leftside.Concat(rightside).ToArray();
                }
            }
        }
        #endregion

        /// <summary>
        /// 組み合わせ列挙
        /// </summary>
        /// <param name="withRepetition">各要素自身の重複</param>
        #region Combination
        public static IEnumerable<T[]> Combination<T>(this IEnumerable<T> items, int k, bool withRepetition = false)
        {
            if (k == 1)
            {
                foreach (var item in items)
                    yield return new T[] { item };
                yield break;
            }
            foreach (var item in items)
            {
                var leftside = new T[] { item };
                var unused = withRepetition ? items : items.SkipWhile(e => !e.Equals(item)).Skip(1).ToList();

                foreach (var rightside in Combination(unused, k - 1, withRepetition))
                {
                    yield return leftside.Concat(rightside).ToArray();
                }
            }
        }
        #endregion

        /// <summary>
        /// フィボナッチ
        /// </summary>
        /// <param name="isZeroStart">初項0</param>
        #region GetFibonacci
        public static long GetFibonacci(int n, bool isZeroStart = true)
        {
            if (n == 0)
                return isZeroStart ? 0 : 1;

            if (n == 1)
                return 1;

            var dp = new long[n + 1];
            dp[0] = isZeroStart ? 0 : 1;
            dp[1] = 1;

            for (var i = 2; i <= n; i++)
            {
                dp[i] = dp[i - 1] + dp[i - 2];
            }

            return dp[n];
        }
        #endregion

        /// <summary>
        /// <para>Lower_Bound</para>
        /// <para>{0,0,1,1,2,2,3,3}(2)なら return 4</para>
        /// </summary>
        /// <returns>value以上の値の最小インデックス,無ければ-1</returns>
        #region BoundLower
        public static int BoundLower<T>(this IList<T> values, T value)
        {
            if (values.Count == 0)
                return -1;

            var cmp = Comparer<T>.Default;

            if (cmp.Compare(values[0], values.Last()) > 0)
                throw new Exception("BoundLower");
            if (cmp.Compare(values[0], value) >= 0)
                return 0;
            if (cmp.Compare(values.Last(), value) < 0)
                return -1;

            var left = 0;
            var right = values.Count - 1;

            while (left <= right)
            {
                var mid = (left + right) / 2;   // 中点取得
                var res = cmp.Compare(values[mid], value);
                if (res == -1)
                    left = mid + 1;
                else
                    right = mid - 1;
            }

            return left;
        }
        #endregion

        /// <summary>
        /// <para>Lower_Bound_Desc</para>
        /// <para>{3,3,2,2,1,1,0,0}(2)なら return 2</para>
        /// </summary>
        /// <returns>value以下の最小インデックス,無ければ-1</returns>
        #region BoundLowerDesc
        public static int BoundLowerDesc<T>(this IList<T> values, T value)
        {
            if (values.Count == 0)
                return -1;

            var cmp = Comparer<T>.Default;

            if (cmp.Compare(values[0], values.Last()) < 0)
                throw new Exception("BoundLowerDesc");
            if (cmp.Compare(values[0], value) <= 0)
                return 0;
            if (cmp.Compare(values.Last(), value) > 0)
                return -1;

            var left = 0;
            var right = values.Count - 1;

            while (left <= right)
            {
                var mid = (left + right) / 2;   // 中点取得
                var res = cmp.Compare(values[mid], value);
                if (res == 1)
                    left = mid + 1;
                else
                    right = mid - 1;
            }

            return left;
        }
        #endregion

        /// <summary>
        /// <para>Upper_Bound</para>
        /// <para>{0,0,1,1,2,2,3,3}(2)なら return 6</para>
        /// </summary>
        /// <returns>valueより大きい最小インデックス,無ければ-1</returns>
        #region BoundUpper
        public static int BoundUpper<T>(this IList<T> values, T value)
        {
            if (values.Count == 0)
                return -1;

            var cmp = Comparer<T>.Default;

            if (cmp.Compare(values[0], values.Last()) > 0)
                throw new Exception("BoundUpper");
            if (cmp.Compare(values[0], value) > 0)
                return 0;
            if (cmp.Compare(values[values.Count - 1], value) <= 0)
                return -1;

            var left = 0;
            var right = values.Count - 1;

            while (left <= right)
            {
                var mid = (left + right) / 2;   // 中点取得
                var res = cmp.Compare(values[mid], value);
                if (res <= 0)
                    left = mid + 1;
                else
                    right = mid - 1;
            }

            return left;
        }
        #endregion

        /// <summary>
        /// <para>Upper_Bound_Desc</para>
        /// <para>{3,3,2,2,1,1,0,0}(2)なら return 4</para>
        /// </summary>
        /// <returns>value未満の最小インデックス,無ければ-1</returns>
        #region BoundUpperDesc
        public static int BoundUpperDesc<T>(this IList<T> values, T value)
        {
            if (values.Count == 0)
                return -1;

            var cmp = Comparer<T>.Default;

            if (cmp.Compare(values[0], values.Last()) < 0)
                throw new Exception("BoundUpperDesc");
            if (cmp.Compare(values[0], value) < 0)
                return 0;
            if (cmp.Compare(values.Last(), value) >= 0)
                return -1;

            var left = 0;
            var right = values.Count - 1;

            while (left <= right)
            {
                var mid = (left + right) / 2;   // 中点取得
                var res = cmp.Compare(values[mid], value);
                if (res >= 0)
                    left = mid + 1;
                else
                    right = mid - 1;
            }

            return left;
        }
        #endregion

        /// <summary>
        /// 距離取得
        /// </summary>
        /// <param name="isEuclidean">true:直線距離,false:縦横距離</param>
        #region GetDistance
        public static double GetDistance<T>(this (T, T) point1, (T, T) point2, bool isEuclidean = true)
        {
            var X = Abs(point1.Item1.ToDouble() - point2.Item1.ToDouble());
            var Y = Abs(point1.Item2.ToDouble() - point2.Item2.ToDouble());
            return isEuclidean ? Sqrt(X * X + Y * Y) : X + Y;
        }
        #endregion

        /// <summary>
        /// bit全探索用のリスト作成
        /// </summary>
        #region CreateBitArrays
        public static IEnumerable<bool[]> CreateBitArrays(this int n)
        {
            var list = new List<bool[]>();
            for (var bit = 0; bit < (1 << n); bit++)
            {
                list.Add(new bool[n]);
                for (var i = 0; i < n; i++)
                {
                    list[bit][i] = (bit & (1 << i)) != 0;
                }
            }

            return list.AsEnumerable();
        }
        #endregion

        /// <summary>
        /// N個からR個選ぶ順列
        /// </summary>
        #region NPR
        public static long NPR(this int num1, int num2)
        {
            if (num1 < num2)
                return 0;

            var ans = 1L;
            for (var i = 0; i < num2; i++)
                ans *= num1 - i;

            return ans;
        }
        #endregion

        /// <summary>
        /// N個からR個選ぶ組み合わせ
        /// </summary>
        #region NCR
        public static long NCR(this int num1, int num2)
        {
            if (num1 < num2)
                return 0;

            if (num2 > num1 / 2)
                num2 = num1 - num2;

            var ans = 1L;
            for (var i = 0; i < num2; i++)
                ans *= num1 - i;
            for (var i = num2; i >= 2; i--)
                ans /= i;

            return ans;
        }
        #endregion
    }
    #endregion

    /// <summary>
    /// 標準入力クラス
    /// </summary>
    #region Scanner
    class Scanner
    {
        string[] _str;
        int _int;
        public Scanner()
        {
            _str = new string[0];
            _int = 0;
        }

        public string Next()
        {
            if (_int < _str.Length)
                return _str[_int++];

            var st = ReadLine()?.Trim();

            while (st == "")
                st = ReadLine();

            if (st != null)
                _str = Split(st, ' ');

            if (_str.Length == 0)
                return Next();

            _int = 0;

            return _str[_int++];
        }

        public string NextLine()
        {
            if (_int < _str.Length)
            {
                var s = " " + string.Join(" ", _str.Skip(_int++));
                _str = new string[0];
                return s;
            }

            var st = ReadLine()?.Trim();

            while (st == "")
                st = ReadLine()?.Trim();

            _str = new string[] { st };
            _int = 0;

            return _str[_int++];
        }

        private static string[] Split(string st, char sep = ' ')
        {
            var sb = new StringBuilder();
            var li = new List<string>();
            for (var i = 0; i < st.Length; i++)
            {
                if (st[i] == sep)
                {
                    if (sb.ToString() != "")
                        li.Add(sb.ToString());
                    sb.Clear();
                }
                else
                {
                    sb.Append(st[i]);
                }
            }
            if (sb.ToString() != "")
                li.Add(sb.ToString());
            return li.ToArray();
        }

        public int NextInt() { return int.Parse(Next()); }

        public long NextLong() { return long.Parse(Next()); }

        public double NextDouble() { return double.Parse(Next()); }
    }
    #endregion

    /// <summary>
    /// 素数
    /// </summary>
    #region PrimeNumber
    public static class PrimeNumber
    {
        public static long[] Primes = { 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97 };

        /// <summary>
        /// 素数判定
        /// </summary>
        #region 素数判定
        public static bool IsPrime(long num)
        {
            if (num <= 1)
                return false;

            if (Primes.Contains(num))
                return true;

            if (Primes.Any(x => num % x == 0))
                return false;

            for (var i = 101L; i * i <= num; i += 2)
            {
                if (num % i == 0)
                    return false;
            }

            return true;
        }
        #endregion

        /// <summary>
        /// 素因数分解
        /// </summary>
        #region 素因数分解
        public static List<long[]> PrimeFactorization(long n)
        {
            if (n <= 1)
                return new List<long[]>();

            var primes = new List<long[]>();

            for (var i = 2L; i * i <= n; i++)
            {
                if (!IsPrime(i))
                    continue;

                var e = (long)0;

                while (n % i == 0)
                {
                    e++;
                    n /= i;
                }

                primes.Add(new long[] { i, e });
            }

            if (n != 1)
                primes.Add(new long[] { n, 1 });

            return primes;
        }
        #endregion

        /// <summary>
        /// 2数の最大公約数
        /// </summary>
        #region 最大公約数
        public static long GreatestCommonDivisor(long num1, long num2)
        {
            var low = num1 <= num2 ? num1 : num2;
            var high = num1 <= num2 ? num2 : num1;
            var temp = (long)0;

            while (true)
            {
                if (high % low == 0)
                    return low;

                temp = high % low;
                high = low;
                low = temp;
            }
        }
        #endregion

        /// <summary>
        /// 2数の最小公倍数
        /// </summary>
        #region 最小公倍数
        public static long LeastCommonMultiple(long num1, long num2)
        {
            return num1 * num2 / GreatestCommonDivisor(num1, num2);
        }
        #endregion
    }
    #endregion
}
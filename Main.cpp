#include <bits/stdc++.h>
// #if __has_include(<atcoder/all>)
// #  include <atcoder/all>
// #endif
// using namespace atcoder;

// 定義系
namespace Define {
// 引数に応じて呼び出しを変える,for文
#define rep_overload(e1, e2, e3, e4, NAME, ...)  NAME
#define REP_OVERLOAD(e1, e2, e3, e4, NAME, ...)  NAME
#define rrep_overload(e1, e2, e3, e4, NAME, ...) NAME
#define RREP_OVERLOAD(e1, e2, e3, e4, NAME, ...) NAME
// for文
#define rep1(i, lim)          for (int i = (int)0; i < (int)lim; i++)
#define rep2(i, x, lim)       for (int i = (int)x; i < (int)lim; i++)
#define rep3(i, x, lim, gap)  for (int i = (int)x; i < (int)lim; i += gap)
#define rep(i, ...)           rep_overload(i, __VA_ARGS__, rep3, rep2, rep1)(i, __VA_ARGS__)
#define REP1(i, lim)          for (int i = (int)0; i <= (int)lim; i++)
#define REP2(i, x, lim)       for (int i = (int)x; i <= (int)lim; i++)
#define REP3(i, x, lim, gap)  for (int i = (int)x; i <= (int)lim; i += (int)gap)
#define REP(i, ...)           REP_OVERLOAD(i, __VA_ARGS__, REP3, REP2, REP1)(i, __VA_ARGS__)
#define rrep1(i, x)           for (int i = (int)x; i > 0; i--)
#define rrep2(i, x, lim)      for (int i = (int)x; i > (int)lim; i--)
#define rrep3(i, x, lim, gap) for (int i = (int)x; i > (int)lim; i -= gap)
#define rrep(i, ...)          rrep_overload(i, __VA_ARGS__, rrep3, rrep2, rrep1)(i, __VA_ARGS__)
#define RREP1(i, x)           for (int i = (int)x; i >= 0; i--)
#define RREP2(i, x, lim)      for (int i = (int)x; i >= (int)lim; i--)
#define RREP3(i, x, lim, gap) for (int i = (int)x; i >= (int)lim; i -= (int)gap)
#define RREP(i, ...)          RREP_OVERLOAD(i, __VA_ARGS__, RREP3, RREP2, RREP1)(i, __VA_ARGS__)

#define all(x)  x.begin(), x.end()
#define rall(x) x.rbegin(), x.rend()
#define el      "\n"
#define spa     " "

#define eps          (1e-10)
#define Equals(a, b) (fabs((a) - (b)) < eps)
} // namespace Define
// 拡張メソッド的なもの
namespace Extensions {
    using namespace std;
    struct string : std::string {
            using std::string::string;
            string() : std::string() {}
            string(const char *str) : std::string(str) {}
            string(const std::string str) : std::string(str) {}
            string TrimEnd() {
                string &target = *this;
                while (this->back() == ' ') {
                    this->pop_back();
                }
                return target;
            }
            string TrimStart() {
                string &target = *this;
                while (this->front() == ' ') {
                    this->erase(this->begin());
                }
                return target;
            }
            string Trim() {
                string &target = *this;
                while (this->back() == ' ') {
                    this->pop_back();
                }
                return this->TrimStart();
            }

            auto Replace(const string &oldValue, const string &newValue) {
                string &target = *this;
                if (oldValue == "")
                    return target;
                size_t pos    = 0;
                size_t offset = 0;
                size_t len    = oldValue.length();
                while ((pos = this->find(oldValue, offset)) != std::string::npos) {
                    this->replace(pos, len, newValue);
                    offset = pos + newValue.length();
                }
                return target;
            }
    };

    template<typename T, typename A = std::allocator<T>>
    class vector : public std::vector<T, A> {
            using std::vector<T, A>::vector;

        public:
            string Join(string sep = "") {
                std::ostringstream oss;
                if (!this->empty()) {
                    oss << this->at(0);
                    rep(i, 1, this->size()) oss << sep << this->at(i);
                }
                return string(oss.str().c_str());
            }
    };

    // vector型判定
    template<typename>
    struct is_vec : std::false_type {};
    template<typename T>
    struct is_vec<std::vector<T>> : std::true_type {};
    template<typename T>
    constexpr bool is_vector = is_vec<T>::value;
    template<typename T>
    constexpr bool is_num = std::is_integral_v<T> || std::is_floating_point_v<T>;
} // namespace Extensions
// いろいろ
namespace Contest {
    using namespace Extensions;
    using std::pair, std::set, std::map, Extensions::string, Extensions::vector;
    // clang-format off
	using ll   = long long;
	using pii  = pair<int, int>;   using pll  = pair<ll, ll>;     using pss = pair<string, string>;
	using vpii = vector<pii>;      using vpll = vector<pll>;      using vpss = vector<pss>;
	using psl  = pair<string, ll>; using pls  = pair<ll, string>;
	using vpsl = vector<psl>;      using vpls = vector<pls>;
	using vi   = vector<int>;      using vvi  = vector<vi>;
	using vl   = vector<ll>;       using vvl  = vector<vl>;
	using vb   = vector<bool>;     using vvb  = vector<vb>;
	using vc   = vector<char>;     using vvc  = vector<vc>;
	using vd   = vector<double>;   using vvd  = vector<vd>;
	using vs   = vector<string>;   using vvs  = vector<vs>;
    // clang-format on

    const double PI    = acos(-1);
    const int    DX[4] = { 1, 0, -1, 0 }; // （下，右，上，左）
    const int    DY[4] = { 0, 1, 0, -1 };
    const int    INF   = 1001001001;
    const ll     INFL  = 4004004003094073385LL; // (int)INFL = INF, (int)(-INFL) = -INF;
    const string ABC   = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const string abc   = "abcdefghijklmnopqrstuvwxyz";

    // 大きい値を第一引数に上書き
    template<typename T1, typename T2>
    inline bool chmax(T1 &a, T2 b) {
        bool compare = a < b;
        if (compare) a = b;
        return compare;
    }

    // 小さい値を第一引数に上書き
    template<typename T1, typename T2>
    inline bool chmin(T1 &a, T2 b) {
        bool compare = a > b;
        if (compare) a = b;
        return compare;
    }

    // 文字列区切り
    static vector<string> Split(const string &s, char delim = ' ') {
        vector<string> elems;
        string         item;
        for (char ch : s) {
            if (ch == delim) {
                if (!item.empty()) elems.push_back(item);
                item.clear();
            }
            else item += ch;
        }
        if (!item.empty()) elems.push_back(item);
        return elems;
    }

    // index取得
    template<typename T>
    ll IndexOf(vector<T> vt, T target) {
        return std::distance(vt.begin(), std::find(all(vt), target));
    }
    // 最小値取得
    template<typename T>
    T Min(vector<T> vt) {
        return *std::min_element(all(vt));
    }
    // 最大値取得
    template<typename T>
    T Max(vector<T> vt) {
        return *std::max_element(all(vt));
    }

} // namespace Contest
// 入出力
namespace IO {
    using namespace Contest;
    using Extensions::vector, Extensions::string;
    class Scanner {
        private:
            int            _int;
            vector<string> _str;

        public:
            Scanner() {
                _int = 0;
                _str.clear();
            }
            string Next() {
                if (_int < (int)_str.size()) return _str[_int++];
                string st;
                getline(std::cin, st);
                while (st == "")
                    getline(std::cin, st);
                _str = Contest::Split(st);
                if (_str.size() == 0) return Next();
                _int = 0;
                return _str[_int++];
            }
            string NextLine() {
                if (_int < (int)_str.size()) {
                    vector<string> vec(_str.size() - _int);
                    std::copy(_str.begin() + _int, _str.end(), vec.begin());
                    string s = " ";
                    s += vec.Join(" ");
                    _str.clear();
                    return s;
                }
                string st;
                getline(std::cin, st);
                while (st == "")
                    getline(std::cin, st);
                _str.clear();
                _str.emplace_back(st);
                _int = 0;
                return _str[_int++];
            }
            int    NextInt() { return std::stoi(Next()); }
            ll     NextLL() { return std::stoll(Next()); }
            double NextDouble() { return std::stod(Next()); }
    };
    auto sc = new Scanner();
    struct FastIO {
            FastIO() {
                std::cin.tie(nullptr);
                std::ios::sync_with_stdio(false);
                std::cout << std::fixed << std::setprecision(18);
            }
    } FastIOtmp;
    ll  InputNum() { return sc->NextLL(); }
    pll InputNumPair() {
        pll p(InputNum(), InputNum());
        return p;
    }
    vpll InputsNumPair(ll n) {
        vpll vec;
        rep(i, n) vec.emplace_back(InputNumPair());
        return vec;
    }
    vl InputNums(ll n) {
        vl vec;
        rep(i, n) vec.emplace_back(InputNum());
        return vec;
    }
    vvl InputsNums(ll h, ll w) {
        vvl vec;
        rep(i, h) vec.emplace_back(InputNums(w));
        return vec;
    }
    string InputString() { return sc->Next(); }
    string InputStringLine() { return sc->NextLine(); }
    vs     InputStrings(ll n) {
        vs vec;
        rep(i, n) vec.emplace_back(InputString());
        return vec;
    }
    vs InputStringLines(ll n) {
        vs vec;
        rep(i, n) vec.emplace_back(InputStringLine());
        return vec;
    }
    vvs InputsStrings(ll h, ll w) {
        vvs vec;
        rep(i, h) vec.emplace_back(InputStrings(w));
        return vec;
    }
    template<typename T>
    void Output(T n, string end = el) { std::cout << n << end; }
    void Output(bool n, string t = "Yes", string f = "No") { std::cout << (n ? t : f) << el; }
    void Output(double n, string end = el) { std::cout << n << end; }
    void Output(string n, string end = el) { std::cout << n << end; }
    void Output(const char *n) { Output(string(n)); }
    void Output(vs n, string sep = " ", string end = el) { Output(n.Join(sep), end); }
    void Output(vvs n, string sep = " ", string end = el) { rep(i, n.size()) Output(n[i].Join(sep), end); }
    void Output(ll n, string end = el) { std::cout << n << end; }
    void Output(pll n, string sep = " ", string end = el) { std::cout << n.first << sep << n.second << end; }
    void Output(vpll n, string sep = " ", string end = el) { rep(i, n.size()) Output(n[i], sep, end); }
    void Output(vl n, string sep = " ") { Output(n.Join(sep)); }
    void Output(vvl n, string sep = " ", string end = el) { rep(i, n.size()) Output(n[i].Join(sep), end); }
} // namespace IO

void Solve() {
    using namespace IO;
    using Extensions::string, Extensions::vector;

    auto A = InputNum();
    auto B = InputNums(2);
    auto C = InputsNums(3, 2);
    auto D = InputString();
    auto E = InputStringLine();
    auto F = InputStrings(2);
    auto G = InputStringLines(2);
    auto H = InputsStrings(3, 2);

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

int main() {
    Solve();
    return 0;
}
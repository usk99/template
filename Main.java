import java.util.ArrayList;
import java.util.Scanner;
import java.util.HashMap;
import java.util.HashSet;

public class Main {
    static void Solve() {
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
    }

    public static void main(String[] args) {
        Solve();
        System.out.print(stringBuilder.toString());
        sc.close();
    }

    static Scanner sc = new Scanner(System.in);
    static StringBuilder stringBuilder = new StringBuilder();
    static String el = System.getProperty("line.separator");

    // #region 入出力
    static long InputNum() {
        return sc.nextLong();
    }

    static ArrayList<Long> InputNums(long n) {
        ArrayList<Long> li = new ArrayList<>();
        for (var i = 0; i < n; i++)
            li.add(sc.nextLong());
        return li;
    }

    static ArrayList<ArrayList<Long>> InputsNums(long h, long w) {
        ArrayList<ArrayList<Long>> li = new ArrayList<ArrayList<Long>>();
        for (var i = 0; i < h; i++) {
            li.add(InputNums(w));
        }
        return li;
    }

    static String InputString() {
        return sc.next();
    }

    static String InputStringLine() {
        return sc.nextLine();
    }

    static ArrayList<String> InputStrings(long n) {
        var li = new ArrayList<String>();
        for (var i = 0; i < n; i++)
            li.add(InputString());
        return li;
    }

    static ArrayList<String> InputStringLines(long n) {
        var li = new ArrayList<String>();
        for (var i = 0; i < n; i++)
            li.add(InputStringLine());
        return li;
    }

    static ArrayList<ArrayList<String>> InputsStrings(long h, long w) {
        var li = new ArrayList<ArrayList<String>>();
        for (var i = 0; i < h; i++)
            li.add(InputStrings(w));
        return li;
    }

    static void Output(boolean n) {
        stringBuilder.append(n ? "Yes" + el : "No" + el);
    }

    static <T> void Output(T t) {
        stringBuilder.append(t.toString() + el);
    }

    static <T> void Output(T[] ts) {
        if (ts.length != 0) {
            stringBuilder.append(ts[0]);
            for (var i = 1; i < ts.length; i++)
                stringBuilder.append(" " + ts[i]);
            stringBuilder.append(el);
        }
    }

    static <T> void Output(T[] ts, String sep) {
        if (ts.length != 0) {
            stringBuilder.append(ts[0]);
            for (var i = 1; i < ts.length; i++)
                stringBuilder.append(sep + ts[i]);
            stringBuilder.append(el);
        }
    }

    static <T> void Output(ArrayList<T> ts) {
        if (!ts.isEmpty()) {
            stringBuilder.append(ts.get(0));
            for (var i = 1; i < ts.size(); i++)
                stringBuilder.append(" " + ts.get(i));
            stringBuilder.append(el);
        }
    }

    static <T> void Output(ArrayList<T> ts, String sep) {
        if (!ts.isEmpty()) {
            stringBuilder.append(ts.get(0));
            for (var i = 1; i < ts.size(); i++)
                stringBuilder.append(sep + ts.get(i));
            stringBuilder.append(el);
        }
    }
    // #endregion

    // #region 変換
    static int ToInt(long n) {
        return (int) n;
    }

    static int ToInt(String s) {
        return Integer.parseInt(s);
    }

    static String ToString(long n) {
        return String.valueOf(n);
    }
    // #endregion
}
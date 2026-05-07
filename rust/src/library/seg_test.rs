    use crate::library::seg::*;
    // -------------------------------------------------------------------------
    // 1. 各演算の個別テスト
    // -------------------------------------------------------------------------
    #[test]
    fn test_sum() {
        // Add: 足し算 (単位元 0)
        let n = 5;
        let mut seg = SegSum::new(n);
        // [1, 2, 3, 4, 5]
        for i in 0..n {
            seg.set(i, i + 1);
        }
        assert_eq!(seg.get_range(0, 5), 15); // 1+2+3+4+5
        assert_eq!(seg.get_range(1, 3), 5); // 2+3
        assert_eq!(seg.get_range(0, 0), 0); // 空区間は単位元
    }
    #[test]
    fn test_min_max() {
        // Min: 最小値 (単位元 usize::MAX)
        // Max: 最大値 (単位元 0)
        let n = 5;
        let mut seg_min = SegMin::new(n);
        let mut seg_max = SegMax::new(n);
        // [10, 20, 5, 30, 15]
        let data = [10, 20, 5, 30, 15];
        for (i, &v) in data.iter().enumerate() {
            seg_min.set(i, v);
            seg_max.set(i, v);
        }
        // Min Check
        assert_eq!(seg_min.get_range(0, 5), 5); // 全体の最小
        assert_eq!(seg_min.get_range(0, 2), 10); // [10, 20] -> 10
        assert_eq!(seg_min.get_range(3, 5), 15); // [30, 15] -> 15
        assert_eq!(seg_min.get_range(0, 0), usize::MAX); // 単位元
        // Max Check
        assert_eq!(seg_max.get_range(0, 5), 30); // 全体の最大
        assert_eq!(seg_max.get_range(0, 3), 20); // [10, 20, 5] -> 20
        assert_eq!(seg_max.get_range(0, 0), 0); // 単位元
    }
    #[test]
    fn test_bitwise() {
        // Xor: 排他的論理和 (単位元 0)
        // BitOr: 論理和 (単位元 0)
        // BitAnd: 論理積 (単位元 usize::MAX)
        let n = 4;
        let mut seg_xor = SegXor::new(n);
        let mut seg_or = SegBitOr::new(n);
        let mut seg_and = SegBitAnd::new(n);
        // Data: [1(001), 2(010), 4(100), 7(111)]
        let data = [1, 2, 4, 7];
        for (i, &v) in data.iter().enumerate() {
            seg_xor.set(i, v);
            seg_or.set(i, v);
            seg_and.set(i, v);
        }
        // Xor Check
        // 1^2^4^7 = 001 ^ 010 ^ 100 ^ 111 = 111 ^ 111 = 0
        assert_eq!(seg_xor.get_range(0, 4), 0);
        assert_eq!(seg_xor.get_range(0, 2), 3); // 1^2 = 3
        // BitOr Check
        // 1|2|4|7 = 7
        assert_eq!(seg_or.get_range(0, 4), 7);
        // BitAnd Check
        // 1&2&4&7 = 0
        assert_eq!(seg_and.get_range(0, 4), 0);
        // 4&7 = 100 & 111 = 100(4)
        assert_eq!(seg_and.get_range(2, 4), 4);
        // 単位元チェック
        assert_eq!(seg_and.get_range(0, 0), usize::MAX);
    }
    #[test]
    fn test_gcd_lcm() {
        // Gcd: 最大公約数 (単位元 0)
        // Lcm: 最小公倍数 (単位元 1)
        let n = 3;
        let mut seg_gcd = SegGcd::new(n);
        let mut seg_lcm = SegLcm::new(n);
        // Data: [12, 18, 6]
        let data = [12, 18, 6];
        for (i, &v) in data.iter().enumerate() {
            seg_gcd.set(i, v);
            seg_lcm.set(i, v);
        }
        // GCD Check
        assert_eq!(seg_gcd.get_range(0, 2), 6); // gcd(12, 18) = 6
        assert_eq!(seg_gcd.get_range(0, 3), 6); // gcd(12, 18, 6) = 6
        // LCM Check
        assert_eq!(seg_lcm.get_range(0, 2), 36); // lcm(12, 18) = 36
        assert_eq!(seg_lcm.get_range(1, 3), 18); // lcm(18, 6) = 18
        // 単位元チェック
        assert_eq!(seg_gcd.get_range(0, 0), 0);
        assert_eq!(seg_lcm.get_range(0, 0), 1);
    }

    // -------------------------------------------------------------------------
    // 2. 二分探索 (max_right / min_left) のテスト
    // -------------------------------------------------------------------------
    #[test]
    fn test_binary_search() {
        // [1, 1, 1, 1, 1] でAddを使用
        let n = 5;
        let mut seg = SegSum::new(n);
        for i in 0..n {
            seg.set(i, 1);
        }

        // max_right: 和が3未満である限界の右端
        assert_eq!(seg.max_right(0, |&s| s < 3), 2); // index 0,1,2 (sum=3) なので3で止まる
        assert_eq!(seg.max_right(2, |&s| s < 2), 3); // index 2,3 (sum=2) なので4で止まる (2+2=4)

        // min_left: 和が2以下である限界の左端
        assert_eq!(seg.min_left(5, |&s| s <= 2), 3); // index 3,4 (sum=2) OK. index 2,3,4(sum=3) NG.
    }

    // -------------------------------------------------------------------------
    // 3. ランダムストレステスト (愚直解との比較)
    // -------------------------------------------------------------------------
    struct Rng(u64);
    impl Rng {
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let len = range.end - range.start;
            range.start + (self.next() as usize % len)
        }
    }

    #[test]
    fn stress_test() {
        let n = 50;
        let q = 1000;
        let mut rng = Rng::new(123456);

        // addで構造の正しさを検証するのが一番確実
        let mut seg = SegSum::new(n);
        let mut vec = vec![0; n];

        for _ in 0..q {
            let ty = rng.gen_range(0..2);
            if ty == 0 {
                // Update
                let i = rng.gen_range(0..n);
                let x = rng.gen_range(0..100);
                seg.set(i, x);
                vec[i] = x;
            } else {
                // Query
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r);
                let vec_res: usize = vec[l..r].iter().sum();

                assert_eq!(seg_res, vec_res, "Mismatch at ({}..{})", l, r);
            }
        }
    }

    // -------------------------------------------------------------------------
    // 遅延セグ木
    // 1. 基本動作テスト (Add系: 区間加算)
    // -------------------------------------------------------------------------
    #[test]
    fn test_lazy_add_sum() {
        let n = 10;
        // SumLen系は初期化時に (値, 1) を入れないと長さが0になってしまうので注意
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegAddSum::from(init_data);

        // [0, 5) に 10 を加算
        // 区間長5 * 10 = +50
        seg.apply_range(0, 5, 10);
        assert_eq!(seg.get_range(0, 10).0, 50);

        // [2, 6) に 5 を加算
        // 重なり部分: [2, 5) は 10+5=15
        // 新規部分: [5, 6) は 0+5=5
        // 全体: [0, 2)=10, [2, 5)=15, [5, 6)=5, [6, 10)=0
        seg.apply_range(2, 6, 5);

        // 検算:
        // [0, 2) (長さ2) * 10 = 20
        // [2, 5) (長さ3) * 15 = 45
        // [5, 6) (長さ1) * 5  = 5
        // 合計: 70
        assert_eq!(seg.get_range(0, 10).0, 70);

        // 部分取得
        assert_eq!(seg.get_range(2, 5).0, 45);
    }
    #[test]
    fn test_lazy_add_max() {
        let n = 5;
        let mut seg = LazySegAddMax::new(n);

        // [0, 0, 0, 0, 0]
        seg.apply_range(0, 3, 10); // [10, 10, 10, 0, 0]
        seg.apply_range(2, 5, 20); // [10, 10, 30, 20, 20]

        assert_eq!(seg.get_range(0, 5), 30); // 全体の最大値
        assert_eq!(seg.get_range(0, 2), 10);
        assert_eq!(seg.get_range(3, 5), 20);

        // 点更新との組み合わせ
        seg.set(0, 50); // [50, 10, 30, 20, 20]
        assert_eq!(seg.get_range(0, 5), 50);
    }

    // -------------------------------------------------------------------------
    // 2. 基本動作テスト (Update系: 区間更新)
    // -------------------------------------------------------------------------
    #[test]
    fn test_lazy_update_min() {
        let n = 5;
        // 初期値は Min の単位元 (usize::MAX)
        let mut seg = LazySegUpdateMin::new(n);

        // 全体を 100 に更新
        seg.apply_range(0, 5, Some(100));
        // [100, 100, 100, 100, 100]
        assert_eq!(seg.get_range(0, 5), 100);

        // [1, 4) を 10 に更新
        seg.apply_range(1, 4, Some(10));
        // [100, 10, 10, 10, 100]
        assert_eq!(seg.get_range(0, 5), 10);
        assert_eq!(seg.get_range(0, 1), 100);

        // None (更新なし) を投げても値が変わらないこと
        seg.apply_range(0, 5, None);
        assert_eq!(seg.get_range(0, 5), 10);
    }
    #[test]
    fn test_lazy_update_sum() {
        let n = 5;
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegUpdateSum::from(init_data);

        // [0, 5) を 10 に更新 -> 合計 50
        seg.apply_range(0, 5, Some(10));
        assert_eq!(seg.get_range(0, 5).0, 50);

        // [0, 3) を 5 に更新 -> [5, 5, 5, 10, 10]
        seg.apply_range(0, 3, Some(5));

        // 検算: 5*3 + 10*2 = 15 + 20 = 35
        assert_eq!(seg.get_range(0, 5).0, 35);
    }
    #[test]
    fn test_lazy_update_xor() {
        // UpdateXor は「長さが偶数なら0、奇数なら値」になるはず
        let n = 5;
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegUpdateXor::from(init_data);

        // [0, 5) を 3 (011) に更新。長さ5(奇数) -> 結果 3
        seg.apply_range(0, 5, Some(3));
        assert_eq!(seg.get_range(0, 5).0, 3);

        // [0, 4) をチェック。長さ4(偶数) -> 結果 0
        assert_eq!(seg.get_range(0, 4).0, 0);
    }

    // -------------------------------------------------------------------------
    // 3. 遅延セグ木上の二分探索
    // -------------------------------------------------------------------------
    #[test]
    fn test_lazy_binary_search() {
        let n = 10;
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegAddSum::from(init_data);

        // 全てに 1 を加算 -> [1, 1, ..., 1]
        seg.apply_range(0, n, 1);

        // max_right: 左端0から和が 5 未満になる右端は？
        // 1+1+1+1+1 = 5 (NG) -> index 5の手前まで -> 4
        // ※ すみません、条件の境界値によります。
        // 「s < 5」なら sum([0,4))=4 (OK), sum([0,5))=5 (NG) -> 答え 4
        assert_eq!(seg.max_right(0, |s| s.0 < 5), 4);

        // min_left: 右端10から和が 3 以下になる左端は？
        // sum([7, 10)) = 3 (OK)
        // sum([6, 10)) = 4 (NG) -> 答え 7
        assert_eq!(seg.min_left(10, |s| s.0 <= 3), 7);
    }

    // -------------------------------------------------------------------------
    // 4. ランダムストレステスト (愚直解との比較)
    // -------------------------------------------------------------------------
    #[test]
    fn stress_test_add_sum() {
        let n = 30; // サイズ
        let q = 1000; // クエリ数
        let mut rng = Rng::new(2025);

        // 遅延セグ木 (値, 1) で初期化
        let init_data = vec![(0, 1); n];
        let mut seg = LazySegAddSum::from(init_data);

        // 愚直配列 (値のみ)
        let mut vec = vec![0; n];

        for _ in 0..q {
            let ty = rng.gen_range(0..3);
            if ty == 0 {
                // Range Add Apply
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);
                let val = rng.gen_range(0..100);

                seg.apply_range(l, r, val);
                for item in vec.iter_mut().take(r).skip(l) {
                    *item += val;
                }
            } else if ty == 1 {
                // Point Set
                let i = rng.gen_range(0..n);
                let val = rng.gen_range(0..100);

                seg.set(i, (val, 1)); // setのときは長さを維持する必要あり
                vec[i] = val;
            } else {
                // Range Query
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r).0;
                let vec_res: usize = vec[l..r].iter().sum();

                assert_eq!(seg_res, vec_res, "AddSum Mismatch at query {},{}", l, r);
            }
        }
    }
    #[test]
    fn stress_test_update_min() {
        let n = 30;
        let q = 1000;
        let mut rng = Rng::new(9999);

        // 初期値は適当な値で埋める
        let mut vec = vec![0; n];
        for item in vec.iter_mut() {
            *item = rng.gen_range(0..1000);
        }

        let mut seg = LazySegUpdateMin::new(n);
        for (i, item) in vec.iter().enumerate() {
            seg.set(i, *item);
        }

        for _ in 0..q {
            let ty = rng.gen_range(0..3);
            if ty == 0 {
                // Range Update Apply
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);
                let val = rng.gen_range(0..1000);

                seg.apply_range(l, r, Some(val));
                for item in vec.iter_mut().take(r).skip(l) {
                    *item = val;
                }
            } else if ty == 1 {
                // Point Set
                let i = rng.gen_range(0..n);
                let val = rng.gen_range(0..1000);

                seg.set(i, val);
                vec[i] = val;
            } else {
                // Range Query
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r);
                let vec_res = if l == r {
                    usize::MAX // 単位元
                } else {
                    *vec[l..r].iter().min().unwrap()
                };

                assert_eq!(seg_res, vec_res, "UpdateMin Mismatch at query {}..{}", l, r);
            }
        }
    }
    #[test]
    fn test_edge_cases() {
        // サイズ1
        let mut seg = SegSum::new(1);
        seg.set(0, 42);
        assert_eq!(seg.get_range(0, 1), 42);

        // 空区間
        assert_eq!(seg.get_range(0, 0), 0);
        assert_eq!(seg.get_range(1, 1), 0);

        // 境界値
        let mut seg = SegMin::new(3);
        seg.set(0, usize::MAX);
        assert_eq!(seg.get_range(0, 1), usize::MAX);
    }
    #[test]
    fn test_value_methods() {
        // ========================================
        // LazySegAddSum（区間加算・区間和）
        // ========================================

        // from_values でシンプルに初期化
        let mut seg = LazySegAddSum::from_values(vec![1, 2, 3, 4, 5]);
        assert_eq!(seg.get_range(0, 5).0, 15); // 1+2+3+4+5

        // set_value で値のみ設定
        seg.set_value(0, 10);
        assert_eq!(seg.get_range(0, 5).0, 24); // 10+2+3+4+5

        // get_value で値のみ取得
        assert_eq!(seg.get_value(0), 10);
        assert_eq!(seg.get_value(1), 2);

        // 区間加算
        seg.apply_range(1, 4, 5); // index 1,2,3 に +5
        assert_eq!(seg.get_range(0, 5).0, 39); // 10+(2+5)+(3+5)+(4+5)+5

        // new_with_size で0初期化
        let mut seg2 = LazySegAddSum::new_with_size(3);
        assert_eq!(seg2.get_range(0, 3).0, 0);
        seg2.apply_range(0, 3, 10);
        assert_eq!(seg2.get_range(0, 3).0, 30);

        // ========================================
        // LazySegUpdateSum（区間更新・区間和）
        // ========================================

        let mut seg = LazySegUpdateSum::from_values(vec![1, 2, 3, 4, 5]);
        assert_eq!(seg.get_range(0, 5).0, 15);

        // 区間更新
        seg.apply_range(1, 4, Some(10)); // index 1,2,3 を 10 に
        assert_eq!(seg.get_range(0, 5).0, 36); // 1+10+10+10+5

        seg.set_value(0, 100);
        assert_eq!(seg.get_value(0), 100);
        assert_eq!(seg.get_range(0, 5).0, 135); // 100+10+10+10+5

        // ========================================
        // LazySegUpdateXor（区間更新・区間XOR）
        // ========================================

        let mut seg = LazySegUpdateXor::from_values(vec![1, 2, 4, 8, 16]);
        // 1^2^4^8^16 = 31
        assert_eq!(seg.get_range(0, 5).0, 31);

        // 全て 7 に更新
        seg.apply_range(0, 5, Some(7));
        // 7^7^7^7^7 = 7 (奇数個なので7)
        assert_eq!(seg.get_range(0, 5).0, 7);

        // 偶数個の範囲
        assert_eq!(seg.get_range(0, 4).0, 0); // 7^7^7^7 = 0

        seg.set_value(0, 3);
        assert_eq!(seg.get_value(0), 3);
        // 3^7^7^7^7 = 3^0 = 3
        assert_eq!(seg.get_range(0, 5).0, 3);
    }
    #[test]
    fn test_value_methods_stress() {
        // LazySegAddSum のストレステスト（便利メソッド版）
        let n = 30;
        let q = 1000;
        let mut rng = Rng::new(54321);

        let mut seg = LazySegAddSum::new_with_size(n);
        let mut vec = vec![0; n];

        for _ in 0..q {
            let ty = rng.gen_range(0..3);
            if ty == 0 {
                // Range Add
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);
                let val = rng.gen_range(0..100);

                seg.apply_range(l, r, val);
                for item in vec.iter_mut().take(r).skip(l) {
                    *item += val;
                }
            } else if ty == 1 {
                // Point Set (便利メソッド)
                let i = rng.gen_range(0..n);
                let val = rng.gen_range(0..100);

                seg.set_value(i, val);
                vec[i] = val;
            } else {
                // Range Query (便利メソッド)
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n + 1);

                let seg_res = seg.get_range(l, r);
                let vec_res = vec[l..r].iter().sum::<usize>();

                assert_eq!(seg_res.0, vec_res, "ValueMethod Mismatch at {}..{}", l, r);
            }
        }
    }
    #[test]
    fn test_value_methods_comparison() {
        // 従来の方法と便利メソッドで結果が一致することを確認
        let n = 10;
        let init_values = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

        // 従来の方法
        let init_data: Vec<_> = init_values.iter().map(|&v| (v, 1)).collect();
        let mut seg_old = LazySegAddSum::from(init_data);

        // 便利メソッド
        let mut seg_new = LazySegAddSum::from_values(init_values.clone());

        // 初期状態の一致確認
        for i in 0..n {
            assert_eq!(seg_old.get(i).0, seg_new.get_value(i));
        }
        assert_eq!(seg_old.get_range(0, n).0, seg_new.get_range(0, n).0);

        // 区間加算後の一致確認
        seg_old.apply_range(2, 7, 10);
        seg_new.apply_range(2, 7, 10);

        for i in 0..n {
            assert_eq!(
                seg_old.get(i).0,
                seg_new.get_value(i),
                "Mismatch at index {}",
                i
            );
        }
        assert_eq!(seg_old.get_range(0, n).0, seg_new.get_range(0, n).0);

        // 点更新後の一致確認
        seg_old.set(5, (100, 1));
        seg_new.set_value(5, 100);

        assert_eq!(seg_old.get(5).0, seg_new.get_value(5));
        assert_eq!(seg_old.get_range(0, n).0, seg_new.get_range(0, n).0);
    }
    #[test]
    fn test_value_methods_edge_cases() {
        // サイズ1のケース
        let mut seg = LazySegAddSum::from_values(vec![42]);
        assert_eq!(seg.get_value(0), 42);
        assert_eq!(seg.get_range(0, 1).0, 42);

        seg.set_value(0, 100);
        assert_eq!(seg.get_value(0), 100);

        seg.apply_range(0, 1, 50);
        assert_eq!(seg.get_range(0, 1).0, 150);

        // 空初期化
        let mut seg = LazySegAddSum::new_with_size(5);
        assert_eq!(seg.get_range(0, 5).0, 0);

        for i in 0..5 {
            assert_eq!(seg.get_value(i), 0);
        }

        // 部分区間のテスト
        let mut seg = LazySegUpdateSum::from_values(vec![10, 20, 30, 40, 50]);

        seg.apply_range(1, 3, Some(100)); // [10, 100, 100, 40, 50]
        assert_eq!(seg.get_range(0, 5).0, 300);
        assert_eq!(seg.get_range(1, 3).0, 200);
        assert_eq!(seg.get_range(0, 1).0, 10);
        assert_eq!(seg.get_range(4, 5).0, 50);
    }
    #[test]
    fn test_value_methods_xor_logic() {
        // XOR の長さ依存ロジックをテスト
        let mut seg = LazySegUpdateXor::new_with_size(6);

        // 全て 5 に設定
        seg.apply_range(0, 6, Some(5));

        // 偶数個: 0
        assert_eq!(seg.get_range(0, 2).0, 0); // 5^5
        assert_eq!(seg.get_range(0, 4).0, 0); // 5^5^5^5
        assert_eq!(seg.get_range(0, 6).0, 0); // 5^5^5^5^5^5

        // 奇数個: 5
        assert_eq!(seg.get_range(0, 1).0, 5); // 5
        assert_eq!(seg.get_range(0, 3).0, 5); // 5^5^5
        assert_eq!(seg.get_range(0, 5).0, 5); // 5^5^5^5^5

        // 複雑なパターン
        let mut seg = LazySegUpdateXor::from_values(vec![1, 2, 3, 4]);
        seg.apply_range(0, 2, Some(7)); // [7, 7, 3, 4]
        seg.apply_range(2, 4, Some(15)); // [7, 7, 15, 15]

        assert_eq!(seg.get_range(0, 2).0, 0); // 7^7
        assert_eq!(seg.get_range(2, 4).0, 0); // 15^15
        assert_eq!(seg.get_range(0, 4).0, 0); // 7^7^15^15
        assert_eq!(seg.get_range(0, 3).0, 15); // 7^7^15 = 15
        assert_eq!(seg.get_range(1, 4).0, 7); // 7^15^15 = 7
    }

    // -------------------------------------------------------------------------
    // isize 版のテスト
    // -------------------------------------------------------------------------
    #[test]
    fn test_isize_basic() {
        // SegMaxI: 負の値を含む最大値
        let n = 5;
        let mut seg = SegMaxI::new(n);
        let data: Vec<isize> = vec![-3, 1, -5, 4, -2];
        for (i, &v) in data.iter().enumerate() {
            seg.set(i, v);
        }
        assert_eq!(seg.get_range(0, 5), 4);
        assert_eq!(seg.get_range(0, 3), 1);
        assert_eq!(seg.get_range(0, 0), isize::MIN); // 単位元

        // SegMinI: 負の値を含む最小値
        let mut seg = SegMinI::new(n);
        for (i, &v) in data.iter().enumerate() {
            seg.set(i, v);
        }
        assert_eq!(seg.get_range(0, 5), -5);
        assert_eq!(seg.get_range(3, 5), -2);
        assert_eq!(seg.get_range(0, 0), isize::MAX); // 単位元

        // SegSumI: 負の値の和
        let mut seg = SegSumI::new(n);
        for (i, &v) in data.iter().enumerate() {
            seg.set(i, v);
        }
        assert_eq!(seg.get_range(0, 5), -5); // -3+1-5+4-2
        assert_eq!(seg.get_range(1, 4), 0); // 1-5+4
    }

    #[test]
    fn test_isize_lazy_add() {
        // LazySegAddMaxI: 区間加算・区間最大値 (負のオフセット)
        let n = 5;
        let mut seg = LazySegAddMaxI::new(n);
        let data: Vec<isize> = vec![3, -1, 4, -1, 5];
        for (i, &v) in data.iter().enumerate() {
            seg.set(i, v);
        }
        // 全体に -10 を加算
        seg.apply_range(0, 5, -10);
        assert_eq!(seg.get_range(0, 5), -5); // 5-10

        // LazySegAddMinI: 区間加算・区間最小値
        let mut seg = LazySegAddMinI::new(n);
        for (i, &v) in data.iter().enumerate() {
            seg.set(i, v);
        }
        seg.apply_range(0, 5, -10);
        assert_eq!(seg.get_range(0, 5), -11); // -1-10

        // LazySegAddSumI: 区間加算・区間和
        let mut seg = LazySegAddSumI::from_values(vec![1, -2, 3, -4, 5]);
        assert_eq!(seg.get_range(0, 5).0, 3); // 1-2+3-4+5
        seg.apply_range(0, 5, -1); // 全体 -1
        assert_eq!(seg.get_range(0, 5).0, -2); // 3 - 5*1
        seg.apply_range(1, 3, 10); // index 1,2 に +10
        assert_eq!(seg.get_range(0, 5).0, 18); // -2 + 10*2
    }

    #[test]
    fn test_isize_lazy_update() {
        // LazySegUpdateMaxI: 区間更新・区間最大値
        let n = 5;
        let mut seg = LazySegUpdateMaxI::new(n);
        seg.apply_range(0, 5, Some(-100)); // 全体を -100 に
        assert_eq!(seg.get_range(0, 5), -100);
        seg.apply_range(2, 4, Some(50)); // [2,4) を 50 に
        assert_eq!(seg.get_range(0, 5), 50);
        assert_eq!(seg.get_range(0, 2), -100);

        // LazySegUpdateMinI: 区間更新・区間最小値
        let mut seg = LazySegUpdateMinI::new(n);
        seg.apply_range(0, 5, Some(100));
        assert_eq!(seg.get_range(0, 5), 100);
        seg.apply_range(1, 4, Some(-50));
        assert_eq!(seg.get_range(0, 5), -50);
        assert_eq!(seg.get_range(0, 1), 100);
        seg.apply_range(0, 5, None); // 変化なし
        assert_eq!(seg.get_range(0, 5), -50);

        // LazySegUpdateSumI: 区間更新・区間和
        let mut seg = LazySegUpdateSumI::from_values(vec![1, 2, 3, 4, 5]);
        assert_eq!(seg.get_range(0, 5).0, 15);
        seg.apply_range(0, 5, Some(-3)); // 全て -3 に更新
        assert_eq!(seg.get_range(0, 5).0, -15);
        seg.apply_range(1, 3, Some(10)); // index 1,2 を 10 に
        assert_eq!(seg.get_range(0, 5).0, -3 + 10 + 10 + (-3) + (-3)); // = 11
    }

    #[test]
    fn test_isize_value_methods() {
        // LazySegAddSumI の便利メソッド
        let mut seg = LazySegAddSumI::from_values(vec![-5, 0, 5, -10, 10]);
        assert_eq!(seg.get_range(0, 5).0, 0); // sum = 0
        assert_eq!(seg.get_value(0), -5);
        assert_eq!(seg.get_value(2), 5);

        seg.set_value(0, -100);
        assert_eq!(seg.get_value(0), -100);
        assert_eq!(seg.get_range(0, 5).0, -95);

        seg.apply_range(0, 3, 1); // [0,3) に +1
        assert_eq!(seg.get_range(0, 5).0, -92); // -95 + 3*1

        // new_with_size
        let mut seg = LazySegAddSumI::new_with_size(3);
        assert_eq!(seg.get_range(0, 3).0, 0);
        seg.apply_range(0, 3, -7);
        assert_eq!(seg.get_range(0, 3).0, -21);

        // LazySegUpdateSumI の便利メソッド
        let mut seg = LazySegUpdateSumI::from_values(vec![10, 20, 30]);
        seg.apply_range(0, 3, Some(-1));
        assert_eq!(seg.get_range(0, 3).0, -3);
        seg.set_value(1, 100);
        assert_eq!(seg.get_value(1), 100);
        assert_eq!(seg.get_range(0, 3).0, 98); // -1 + 100 + (-1)
    }

## Rust とは

- 2015 年にリリース
- 静的型付け言語
- C/C++の代わりとなるシステムプログラミング言語（ハードウェア,OS 開発などに用いられる）

## Rust の特徴

- 習得難易度が高い（システムプログラミング言語の特徴とも言える）

- Stack overflow survey で 5 年連続、開発者の好きな言語ランキング一位

- C、C++言語に次ぐ高速処理([ベンチマークサイト](https://benchmarksgame-team.pages.debian.net/benchmarksgame/box-plot-summary-charts.html))

  - C++,C,Rust が言語の中で最速
  - 以下はバイナリーサーチなどある程度時間がかかる処理を比較した結果
    - Go,Java は 3 倍の処理時間がかかる。
    - PHP は約 15 倍の処理時間がかかる。
    - Ruby/Python は 30 倍の処理時間がかかる。

- メモリ安全性が担保されている

  - 世の中のシステムにおいて重大なセキュリティ脆弱性の 70%はメモリ安全性に起因するものと言われている。
  - C/C++はメモリ操作の自由度が高すぎてバグが発生しやすい
    - 一般的なシステムプログラミング言語は GC を使用していない。
    - GC(Garbage collection)とは
      - 実行中のプログラムが確保していたメモリが不要になった際に自動的に開放してくれる機能。
      - Ruby,JS などにおいては開発者がメモリ管理を意識する必要はない。
    - その分毎回の処理でメモリ解放の処理が追加で走るため性能的には劣化する。
  - Rust はそのメモリ安全性を言語仕様として保証してくれる（コンパイル時点でチェック）

- 世の中の企業の Rust 採用
  - Google は 2021 年４月に Android や Linux カーネルを開発する言語に Rust を採用することを発表
  - Microsoft は Windows10 から部分的に C/C++から Rust にリライトを開始

[The Book](https://doc.rust-lang.org/book/ch01-01-installation.html)

[Crate.io](https://crate.io/)

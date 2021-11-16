## Rust

Rust の特徴
システムプログラミング言語（OS 開発向き）
習得難易度が高い
C、C++言語に次ぐ高速処理（ベンチマークサイト）
C++,C,Rust が言語の中で最速
Go,Java は 3 倍の処理時間
PHP は約 15 倍
Ruby/Python は 30 倍
Google は 2021 年４月に Android や Linux カーネルを開発する言語に Rust を採用することを発表
Microsoft は Windows10 から部分的に C/C++から Rust にリライトを開始
重大なセキュリティ脆弱性の 70%はメモリ安全性に起因するもの
そのメモリ安全性を言語仕様として保証してくれる（コンパイル時点でチェック）
GC(Garbage collection)未使用でもメモリ管理の人為的ミスが起きない。
GC とは：実行中のプログラムが確保していたメモリが不要になった際に自動的に開放してくれる機能。
その分毎回の処理でメモリ解放の処理が追加で走るため性能的には劣化する。

The Book
https://doc.rust-lang.org/book/ch01-01-installation.html

Crate.io
https://crate.io/

# unit-calculation-coins-2020

※本プログラムは筑波大と無関係  
※卒業できなくても責任は取れません  
※履修確認には単位の読み替えなど機械では判定しきれない部分が多々あります

![image](https://user-images.githubusercontent.com/45098934/164147062-dbcfed43-f77e-4458-9101-f63b4553bfb4.png)

2020 年度入学 coins 専用単位計算

Windows は csv の改行コードに注意

## 実行方法

```bash
cargo run <Twins からダウンロードした履修データ CSV のパス>
cargo run <Twins からダウンロードした履修データ CSV のパス> -p  # 履修中の単位も計算に含める
cargo run <Twins からダウンロードした履修データ CSV のパス> -r  # 単位計算の対象条件の正規表現を表示
```

# flatlink

flatlinkは以下のように動作します。

- srcオプションで指定したディレクトリ内のすべてのファイル(サブディレクトリも含む)を対象とします。
- dstオプションで指定したディレクトリ内に階層を作らずにハードリンクしたファイルを作成します。

ハードリンクしたファイル名は階層の区切り文字を`_`に置換します。
たとえば`testdir/file.jpg`は`testdir_file.jpg`というファイル名で作成します。

## 使用法

```sh
Usage: flatlink.exe --src <SRC> --dst <DST>

Options:
  -s, --src <SRC>  リンク元のディレクトリパス
  -d, --dst <DST>  リンク作成先のディレクトリパス
  -h, --help       Print help
  -V, --version    Print version
```

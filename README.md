# fcpst
[![MIT License](https://img.shields.io/badge/License-MIT-green)](https://github.com/clownUR/fcpst/blob/main/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/clownUR/fcpst/badge.svg?branch=main)](https://coveralls.io/github/clownUR/fcpst?branch=main)  
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/clownur/fcpst)](https://rust-reportcard.xuri.me/report/github.com/clownur/fcpst)  
![logo](img/envelope.jpg)  

ファイルやディレクトリを圧縮，展開するためのツール（コマンド）

# ソフトウェア名
fcpst(files compression tools)

## Description
圧縮フォーマットやそれらを操作するためのコマンドは複数個存在している（zipコマンドなど)．  
これらは単一の圧縮フォーマットを操作するためのコマンドであり，使い方はそれぞれ僅かに異なっている．  
fcpstでは，フォーマットの異なる圧縮ファイルの展開，各種圧縮フォーマットによるファイル・ディレクトリの圧縮を可能にする．     

## Usage
```sh
fcpst [OPTIONS] <ARGUMENTS...>
Arguments:
  [ARGUMENTS]...  圧縮もしくは解凍したいファイル・ディレクトリ. 
OPTIONS
  -m, --mode <MODE>     archive，extractモードのどちらかを選択する．defaultではautoモードが実行される．
  -f, --format <FORMAT> 圧縮フォーマットもしくは解凍したいフォーマットを選択する．defaultではzipが実行される．  
  -d, --dest <DEST>     出力先のディレクトリを指定する．存在しなければ作成する．  
  -h, --help            helpメッセージを表示する.
  -v, --version         コマンドのバージョンを表示する.

```
Supported archive formats:
- Tar
- Zip

## Install

```sh
brew install clownUR/tap/fcpst
```

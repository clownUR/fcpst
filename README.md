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
OPTIONS
  -m, --mode <MODE>     archive，extractモードのどちらかを選択する．何も選択しなければdefaultモードで実行される．  
  -d, --dest <DEST>     出力先のディレクトリを指定する．  
  -o, --output <FILE>   出力する圧縮ファイル．デフォルトでは，引数として与えられたファイルとディレクトリの名前に
                        拡張子zipをつけた圧縮ファイルが出力される.  
  -h, --help            helpメッセージを表示する.

ARGUMENTS  
  extract mode: 圧縮ファイルを展開する．
  archive mode: ファイルを圧縮する.
  auto mode:    引数に圧縮ファイルが指定されている場合,展開する．それ以外の場合，ファイルを圧縮する．
```

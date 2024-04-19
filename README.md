# fcpst
[![MIT License](https://img.shields.io/badge/License-MIT-green)](https://github.com/clownUR/fcpst/blob/main/LICENSE)   
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
  -o, --output <FILE>   出力する圧縮ファイル．デフォルトでは，引数として与えられたファイル名・ディレクトリ名に拡張子zipをつけた圧縮ファイルが出力される．  
  -h, --help            helpメッセージを表示する.

ARGUMENTS  

```

# fcpst
[![MIT License](https://img.shields.io/badge/License-MIT-green)](https://github.com/clownUR/fcpst/blob/main/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/clownUR/fcpst/badge.svg?branch=main)](https://coveralls.io/github/clownUR/fcpst?branch=main)
[![Version](https://img.shields.io/badge/version-0.1.4-blue)](https://github.com/clownUR/fcpst/releases/tag/v0.1.4)
[![DOI](https://zenodo.org/badge/785141062.svg)](https://zenodo.org/doi/10.5281/zenodo.12747297)  
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/clownur/fcpst)](https://rust-reportcard.xuri.me/report/github.com/clownur/fcpst)  
![logo](img/envelope.jpg)  

ファイルやディレクトリをアーカイブ，展開するためのツール（コマンド）

# ソフトウェア名
fcpst(files compression tools)

## Description
アーカイブフォーマットやそれらを操作するためのコマンドは複数個存在している（zipコマンドなど)．  
これらは単一のフォーマットを操作するためのコマンドであり，使い方はそれぞれ僅かに異なっている．  
fcpstでは，フォーマットの異なるアーカイブファイルの展開，各種フォーマットによるファイル・ディレクトリのアーカイブを可能にする．  
出力後の拡張子を除くアーカイブファイルのファイル名はfcpstとなる．

## Usage
```sh
fcpst [OPTIONS] <INPUT...>
Arguments:
  [INPUT]...  アーカイブもしくは展開したいファイル・ディレクトリ. 
OPTIONS
  -m, --mode <MODE>     archive，extractモードのどちらかを選択する．defaultではautoモードが実行される．
  -f, --format <FORMAT> フォーマットを選択する．defaultではzipが実行される．  
  -d, --dest <DEST>     出力先のディレクトリを指定する．存在しなければ作成する．  
  -h, --help            helpメッセージを表示する.
  -v, --version         コマンドのバージョンを表示する.

```
Supported archive formats:
- Tar
- Zip

## Install

```sh
brew install clownur/tap/fcpst
```
## :whale: Docker

```sh
docker run -it --rm -v $PWD:/workdir ghcr.io/clownur/fcpst:0.1.4 [OPTIONS] [INPUT]...
```



# fcpst
ファイルやディレクトリを圧縮，展開するためのツール（コマンド）

# ソフトウェア名
fcpst(files compression tools)

## Description
圧縮フォーマットやそれらを操作するためのコマンドは複数個存在している（zipコマンドなど)．  
これらは単一の圧縮フォーマットを操作するためのコマンドであり，使い方はそれぞれ僅かに異なっている．  
fcpstでは，フォーマットの異なる圧縮ファイルの展開，各種圧縮フォーマットによるファイル・ディレクトリの圧縮を可能にする．     

## Usage
```sh
fcpst [OPTIONS] <TARGETFILES...>
OPTIONS
  -m, --mode <MODE>     archive，extractモードのどちらかを選択する．何も選択しなければdefaultモードで実行される．    
  -h, --help            helpメッセージを表示する.

TARGETFILES  

```

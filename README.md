# fcpst
ファイルやディレクトリを圧縮，展開するためのツール（コマンド）

# ソフトウェア名
fcpst(files compression tools)

## Description
圧縮フォーマットやそれを操作するためのコマンドは複数個存在している（zipコマンドなど)．  
これらのコマンドのインターフェースは僅かに異なっている．  
fcpstでは，フォーマットの異なる圧縮ファイルの展開，各種圧縮フォーマットによるファイル・ディレクトリの圧縮を可能にする．     

## Usage
```sh
fcpst [OPTIONS] <TARGETFILES...>
OPTIONS
  -m, --mode            archive，extractモードのどちらかを選択する．何も選択しなければdefaultモードで実行される．    
  -h, --help            helpメッセージを表示する.

TARGETFILES
  archive mode: 圧縮したいディレクトリやファイル  
  extract mode: 展開したい圧縮ファイル  
  default mode: 引数が圧縮ファイルであれば展開し，そうでなければ圧縮する．  

```

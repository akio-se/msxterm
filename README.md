# msxterm とは

msxterm とは PC から MSX0 に TCP/IP 接続するための専用ターミナルソフトです。
CUI で主にプログラム作成に使用する事を想定しています。
ＢＡＳＩＣでプログラムを組む際に色々便利な機能を内蔵しています。

## 対応プラットフォーム

CUI のコマンドラインから起動するようになっており、Windows、mac, Linux に対応します。

Windows版、Mac版 のバイナリは Release ページからダウンロードしてください。

Linux の方は各自でビルドしてください。
ビルドの際には rust のインストールが必要です。

## インストール方法

zip を展開してパスの通った場所にコピーしてください。


## 起動方法

* Mac、Linux の場合通常のシェルから実行します。
* Windows の場合コマンドプロンプトあるいは powershell から実行してください。

```
> msxterm 192.168.100.2:2223
```
引数として MSX0 の「IPアドレス:ポート番号」を指定して接続します。

### 文字の入力
* 通常のプロンプトが出ている状態で文字を入力します。
* Enter を押すと文字列がＭＳＸに送信されます。
* Enter を押すまではカーソルキーやコントロールキーで文字列が編集できます。
* 送信された文字列はヒストリに記録されます。カーソルの上下で過去のヒストリを呼び出せます。
* BASICであればこのヒストリで簡単にプログラムの修正ができます。

### プログラムの流し込み
* PC側のファイルをMSX0側にロードする機能があります。
* テキストのコピペにも対応しています。
* 複数行のテキストを貼り付けても一行ずつ分解されて MSX0 側に送られます。
* その際にヒストリにも一行ずつ登録されます。

### 特に覚えておいてほしいキー
* BASICのプログラムを停止するのは Ctrl-C となります。
* ターミナル側の画面のクリアは　Ctrl-L です。
* MSX側の画面のクリアは CLS 命令を使ってください。
* Ctrl-R で目的の行を探すのが便利だと思います。

### キーバインド詳細

基本的に Emacs 相当のバインドになっています。

| key      | action                       |
| -------- | ---------------------------- |
| Ctrl-A   | 行頭に戻る                     |
| Ctrl-B   | カーソル左                     |
| Ctrl-C   | Stop                         |
| Ctrl-D   | 一文字削除                     |
| Ctrl-E   | 行末へ移動                     |
| Ctrl-F   | カーソル右                     |
| Ctrl-G   | 検索キャンセル                  |
| Ctrl-H   | BackSpace                    |
| Ctrl-I   | TAB                          |
| Ctrl-J   | 改行                          |
| Ctrl-K   | カーソル位置から行末まで削除      |
| Ctrl-L   | 表示クリア                     |
| Ctrl-M   | 改行                          |
| Ctrl-N   | カーソル下                     |
| Ctrl-O   |                              |
| Ctrl-P   | カーソル上                     |
| Ctrl-Q   |                              |
| Ctrl-R   | インクリメンタル検索 後方        |
| Ctrl-S   | インクリメンタル検索 前方        |
| Ctrl-T   | 文字入れ替え                   |
| Ctrl-U   | カーソル位置手前を削除           |
| Ctrl-V   |                              |
| Ctrl-W   | スペースで区切られた単語単位で削除 |
| Ctrl-X-U | Undo                         |
| Ctrl-Y   | ヤンクバッファからペースト        |
| Ctrl-Z   | サスペンド (Linux)             |


# ターミナルコマンド

文字入力の先頭が # から始まる行は MSX0 側には送られずターミナル側のコマンドとして解釈されます。
## quit

```
> #quit
```

msxterm を終了します。

## hex

```
> #hex 40 6e 3c 0d
```

* スペースで区切られた2桁の16進数を ASCIIコードとして MSX0 側へ送信します。
* 現状の MSX0 のASCIIコードはかなり特殊なものとなっています。
* そのへんの検証用の機能と思ってください。

## load

```
> #load ./hello_world.bas
```

* PC側にあるテキストファイルのパスを指定します。
* それを読み込んで MSX0 側に送信します。
* 同時にターミナルのヒストリ、プログラムバッファにも登録されます。

## clear_history

```
> #clear_history
```
* ヒストリバッファの履歴を全て消去します。


# プログラムバッファについて　(未実装)
ここから先はこれから実装する予定です。
ターミナル側にはヒストリバッファとは別にプログラムバッファを作成します。
行の先頭から行番号がついた文字列はプログラムとみなしてプログラムバッファに溜めます。ターミナル側にプログラムのバックアップがある状態と思ってください。

## list

```
> #list
```

ターミナル側のプログラムバッファからlistを表示します。


## save

```
> #save ./save_file.bas
```

* ターミナル側のプログラムバッファに登録されている内容を保存します。
* MSX0 側の内容とズレている場合があります。
* その場合、Reload で MSX0 側と同期をとります。

## reload_from

```
> #reload_from
```

* ターミナル側のプログラムバッファを破棄して MSX0側のプログラムを読み込みます。
* list 命令で表示している内容を取り込む形になるので途中で Ctrl-C などで止めないようにしてください。

## reload_to

```
> #reload_to
```

* MSX0側のプログラムをNEWしてターミナル側のプログラムバッファから読み込みます。
* リロードの向きを間違って消さないように気をつけてください。

# MSX0 の文字コードの問題について

現状の試作期では MSX0側の受信処理は M5 Faces Keyboard からの入力処理が流用されているようです。その為かなり特殊な文字コードとなっています。構造上どうしても送れない文字があります。

![MSX0 Key](images/MSX0_Input_ASCII.png)

また、ターミナルから MSX0 に対しての送信がやたら遅く感じるかもしれませんが、keyboard入力をエミュレートする部分がウェイトを入れて同期を取っていて、それが1文字ずつ処理しているため遅いのだと思われます。
ここは MSX0側のシステムの問題なのでいつか改善されることを切に願います。


# 文字コードの流れ

## MSXへの入力側
```
msxterm(UTF-8) → (FacesKeybord Code) → MSX0 (MSX ASCII)
```

半角の入力不可文字は間違って入力してもプログラム的な害がないよう半角スペースに変換します。

```
! % ? ` { | } ~
```

ターミナル側で全角ひらがなを入力すると、MSX ASCIIのひらがなになって届きます。
ひらがな以外の全角文字は無視されます。入力不可のかな文字は近しい文字に変換されます。

| 元 | 後 |
| -- | -- |
| ぇ | え |
| ね | れ |
| を | お |
| ・ | 　 |


## MSXからの出力側
```
MSX0 (MSX ASCII) → msxterm (UTF-8)
```
出力側は特に大きな問題はないと思います。
ひらがな、カタカナなどはターミナル側では半角ではなく全角で表示されるようになっています。
MSX特有のグラフィック文字には対応していません。

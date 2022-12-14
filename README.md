# norm formatter

[English ver](https://github.com/cou723/norm-formatter/blob/main/README-en.md)

## ~~開発終了しました~~

- ~~既存のformatterがあること~~
- ~~既存のformatterに問題、不満点がない事~~
- ~~Rustで書いているため実行速度で差別化ができるが、実際には多くても150行程度のファイルしか読み込まないため大きな差が出ないと思われるため~~

~~以上三つの理由で開発を終了します~~

## 開発再開
既存のformatterがwindows単体では動作しないため、開発を継続します

## 概要

C言語で書かれたコードをnorminetteに合わせた形で整形するCommand line toolのformatter/linterです

## インストール
### 
### Linux
もし`cargo`がインストールされていない場合は、以下のコマンドを実行して`rustup`をインストールし、`cargo`を使えるようにしてください。
```
$ curl https://sh.rustup.rs -sSf | sh
```
以下のコマンドでインストールできます
```
$ cargo install norm-formatter
```
### Mac
手元にMacがないため、検証できません。
おそらくLinuxとほぼ同じ方法でインストールできますが、動作確認のできた方は courange.c@gmail.com まで連絡お願いします。

## 使い方
<path>にフォーマットしたい`.c`または`.h`ファイルを指定してください。

```shell
$ norm-formatter.exe <path>
```

VSCodeではRun on Saveと組み合わせることにより保存時に自動的に実行できるようになります。

### 使用例

Windows + VSCodeでの使用例です。

以下の手順を踏むことにより保存時にフォーマットされます。

1. ダウンロードした実行ファイル(norm-formatter)へパスを通す
2. 以下のような設定1を`.vscode/setting.json`に追加する
3. VSCodeの拡張機能Clang-Format/xaverを追加する
4. [c_formatter_42](https://github.com/dawnbeen/c_formatter_42)の設定ファイル`c_formatter_42/c_formatter_42/data/.clang-format`をワークスペースのルートに置く
5. VSCode上の設定でformatOnSaveを有効にする

設定1
```json
  "emeraldwalk.runonsave": {
    "commands": [
      {
        "match": ".c",
        "isAsync": true,
        "cmd": "norm-formatter ${file}"
      },
    ]
  }
```

## 対応しているエラーメッセージ

全てのエラーメッセージに対応しているわけではなく [dawnbeen](https://github.com/dawnbeen)さんの[c_formatter_42](https://github.com/dawnbeen/c_formatter_42)にあるclang-formatの設定ファイル(`c_formatter_42/c_formatter_42/data/.clang-format`)とclang-formatの併用を前提に開発しています。

将来的にclang-formatを含めた一つのフォーマッターとして制作する予定です。

| エラーメッセージ     | 対策内容                                                                           |
|----------------------|------------------------------------------------------------------------------------|
| NEWLINE_PROCESS_FUNC | 関数と関数の間に空行がない場合、そこに追加します                                   |
| RETURN_PARENTHESIS   | 返り値が括弧で囲われていない場合、括弧で囲います                                   |
| NO_ARGS_VOID         | 関数定義時に引数がない場合voidを付けます                                           |
| SPACE_REPLACE_TAB    | 変数宣言の変数の型と変数名の間がspaceだった場合、spaceを削除し前後の関数とそろえる |
| SPACE_BEFORE_FUNC    | 関数定義の関数の返り値と関数名の間がspaceだった場合、タブを置き換えます            |
| BRACE_SHOULD_EOL     | 最終行に空行がなかった場合挿入します                                               |
| SPACE_AFTER_KW       | `break;`を`break ;`にする                                                          |

### NEWLINE_PROCESS_FUNC

before
```c
int	x(void)
{
	return 1;
}
int	y(void)
{
	return 2;
}
```

after
```c
int	x(void)
{
	return 1;
}

int	y(void)
{
	return 2;
}
```

### RETURN_PARENTHESIS

before
```c
int	x(void)
{
	return 1;
}
```

after
```c
int	x(void)
{
	return (1);
}
```

### NO_ARGS_VOID

before
```c
int	x(void)
{
	return (1);
}
```

after
```c
int	x(void)
{
	return (1);
}
```

### SPACE_REPLACE_TAB

before
```c
void	x(void)
{
	int x;
	double y;
}
```

after
```c
void	x(void)
{
	int		x;
	double	y;
}
```

### SPACE_BEFORE_FUNC

before
```c
int x(void)
{
	return (1);
}
```

after
```c
int	x(void)
{
	return (1);
}
```

### BRACE_SHOULD_EOL

before
```c
int	x(void)
{
	return (1);
}
```

after
```c
int	x(void)
{
	return (1);
}

```

## License

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

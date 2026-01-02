# Srapbox to NotebookLM

Srapboxの内容を、NotebookLMにインポートするために、Markdownに変換するツール。


jsonファイルを食わせると、Markdownファイルを`outdir`に出力する。
内部リンク構造は`[[Page Title]]`の形で維持される。
同様に、外部リンクは`[link text](url)`の形で維持される。

### インストール

// TODO


## 入力ファイルの要件

Scrapboxからエクスポートしたjsonファイルであること。
フォーマットは以下。（参考: https://scrapbox.io/takker/scrapbox_json_data )

```typescipt
type importData = {
   pages: {
     title: string;
     created?: number;
     updated?: number;
     id?: string;
     lines: {
       id?: string;
       userId?: string;
       text: string;
       created: number;
       updated: number;
     }[] | string[];
   }[];
 };
```


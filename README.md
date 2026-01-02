# 📄 Scrapbox to Markdown Converter for NotebookLM

It converts your Scrapbox export (JSON) into clean Markdown files that NotebookLM (and other AI appliances) can easily digest, preserving your links, code blocks, and the traces of your thought process.

## ✨ Features

- **🔗 Link Preservation**: Converts `[Page]` to `[[Page]]`, enhancing referability within NotebookLM.
- **🌐 External Link Support**: Properly formats `[Google https://google.com]` as `[Google](https://google.com)`.
- **📝 List Conversion**: Transforms Scrapbox's indent-based list style into standard Markdown bullet points.
- **💻 Code Blocks**: Handles `code:rust` syntax and converts it into fenced Markdown code blocks.
- **🛡️ Filename Sanitization**: Safely replaces OS-unfriendly characters (like `:` or `/`) in page titles.

## 🚀 Getting Started

### 1. Preparation

You'll need the Rust environment. If you don't have it, grab it at [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/your-username/scrapbox-to-notebooklm.git
cd scrapbox-to-notebooklm

# Build
cargo build --release
```

### 2. Execution

Grab your exported JSON file from Scrapbox and run the command:

```bash
# By default, files are exported to the ./out directory
./target/release/scrapbox-to-notebooklm scrapbox_export.json

# Or specify a custom output directory
./target/release/scrapbox-to-notebooklm scrapbox_export.json -o my_markdowns
```

### 3. Upload to NotebookLM

Simply upload the generated Markdown files as sources in Google NotebookLM. 
Then, sit back, grab a coffee ☕, and wait for the AI to finish learning your brain.

## 🛠️ Conversion Rules

| Scrapbox Syntax | Markdown Output | Notes |
|---|---|---|
| `[Link]` | `[[Link]]` | Internal link |
| `[http://...]` | `[url](url)` | External link |
| `[text url]` | `[text](url)` | Named link |
| `[* Bold]` | `**Bold**` | Bold text |
| `[/ Italic]` | `*Italic*` | Italic text |
| `[- Strike]` | `~~Strike~~` | Strikethrough |
| `code:rs` | ` ```rs ... ``` ` | Code blocks |
| Indentation | `- ` (List) | Nested structures preserved |

## ⚠️ Disclaimer

- This tool does not guarantee a "perfect" 1:1 reproduction of your Scrapbox. Scrapbox's flexibility is, after all, infinite.
- We are not responsible if the AI becomes too smart after reading your notes and decides to start an uprising.
- **Images (e.g., Gyazo)**: Currently, images are treated as links or may be ignored, as NotebookLM's primary strength lies in text analysis.

## 🤝 Contributing

Bug reports, feature requests, or "this code could be more idiomatic Rust" PRs are all very welcome!

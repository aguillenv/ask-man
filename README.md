# ask-man

**ask-man** is a simple and efficient command-line tool that uses OpenAI's GPT models to generate and explain shell commands from natural language instructions. Designed for developers and terminal users who want fast, accurate commands without searching documentation.

---

## 🚀 Features

* **🛠 Natural Language to Shell Command:** Describe your task in plain language and get the appropriate shell command instantly.
* **📖 Explanations (Optional):** Understand what each command does with clear, concise explanations.
* **🤖 Powered by OpenAI:** Uses GPT models via OpenAI's API for high-quality output.
* **💡 Minimal & Simple:** Clean CLI interface with minimal dependencies, written in Rust.

---

## 📥 Installation

1. **Download the latest binary via curl**

Replace `<os>` and `<arch>` with your platform values (`macos` or `linux`, `x86_64` or `aarch64`):

```sh
curl -L https://github.com/aguillenv/ask-man/releases/download/latest/ask-man-<os>-<arch>.tar.gz -o ask-man.tar.gz
```

For example, for macOS on Intel (`x86_64`):

```sh
curl -L https://github.com/aguillenv/ask-man/releases/download/latest/ask-man-macos-x86_64.tar.gz -o ask-man.tar.gz
```

2. **Extract the archive**

```sh
tar -xzf ask-man.tar.gz
```

3. **Move the binary to your `$PATH`**

```sh
sudo mv ask-man /usr/local/bin/
chmod +x /usr/local/bin/ask-man
```

4. **Verify installation**

```sh
ask-man --help
```

5. **Set your OpenAI API key**

Create a `.env` file following the `.env.template` in your working directory with:

```env
OPENAI_API_KEY=your_openai_api_key_here
```

## ⚡ Usage

To generate a shell command from natural language:

```sh
ask-man "zip a folder"
```

To receive a command **with explanation**:

```sh
ask-man "list processes running on port 3000" --explain
```

## ⚙️ Configuration

* **Prompts customization:**
  Edit the following files to customize behavior:

  * `prompt.txt` – used for command generation.
  * `explain.txt` – used for command + explanation.

* **Model selection:**
  Change the OpenAI model in `src/openai.rs` (e.g., `gpt-4`, `gpt-4o`).

## 📄 License

This project is licensed under the **MIT License**.
See the [LICENSE](./LICENSE.md) file for details.

## 🤝 Contributing

Contributions and ideas are welcome!
Feel free to open issues or submit pull requests.

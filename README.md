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

1. **Clone the repository:**

```sh
git clone https://github.com/yourusername/ask-man.git
cd ask-man
```

2. **Set up your OpenAI API key:**

Create a `.env` file in the project root:

```sh
echo "OPENAI_API_KEY=your_openai_api_key_here" > .env
```

3. **Build the project:**

```sh
cargo build --release
```

4. **(Optional) Install globally:**

```sh
cargo install --path .
```

---

## ⚡ Usage

To generate a shell command from natural language:

```sh
ask-man "list processes running on port 3000"
```

To receive a command **with explanation**:

```sh
ask-man "zip a folder" --explain
```

---

## ⚙️ Configuration

* **Prompts customization:**
  Edit the following files to customize behavior:

  * `prompt.txt` – used for command generation.
  * `explain.txt` – used for command + explanation.

* **Model selection:**
  Change the OpenAI model in `src/openai.rs` (e.g., `gpt-4`, `gpt-4o`).

---

## 📄 License

This project is licensed under the **MIT License**.
See the [LICENSE](./LICENSE.md) file for details.

---

## 🤝 Contributing

Contributions and ideas are welcome!
Feel free to open issues or submit pull requests.

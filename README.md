<div align="center">

# 🚀 quick-arch

**Universal Project Structure Generator built in Rust**

![Crates.io](https://img.shields.io/crates/v/quick-arch)
![License](https://img.shields.io/crates/l/quick-arch)
![Rust Edition](https://img.shields.io/badge/edition-2024-blue)
![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Downloads](https://img.shields.io/crates/d/quick-arch)

[Features](#-key-features) • [Installation](#-installation) • [Usage](#-usage) • [Configuration](#-configuration) • [Examples](#-examples)

**Automate your scaffolding process using JSON templates with intelligent conditional logic.**

</div>

---

## 📖 About

**quick-arch** is a blazing-fast Command Line Interface (CLI) tool built with Rust. It automates the process of creating project structures (scaffolding) by reading JSON configuration files. Whether you are building a FastAPI backend, a Flutter mobile app, or a complex Rust microservices architecture, `quick-arch` generates thousands of files and directories in seconds.

### Why quick-arch?

- **Blazing Fast:** Built on Rust for maximum performance and zero runtime overhead.
- **JSON Driven:** Define your entire project structure in a simple, human-readable JSON file.
- **Smart Conditions:** Include or exclude files/directories based on architecture choices (e.g., CQRS, DDD).
- **Zero Dependencies:** A single binary executable—no need for Python, Node, or `jq` installed on the target machine.
- **Cross-Platform:** Works seamlessly on Linux, macOS, and Windows.

---

## ✨ Key Features

- 🎯 **Flexible Templates:** Support for any language or framework via JSON configuration.
- 🧠 **Conditional Logic:** Use simple equality checks within JSON to toggle features like Kubernetes, Docker, or specific design patterns.
- 🚀 **Single Binary:** Distribute and run anywhere without installation headaches.
- 🎨 **Beautiful Output:** Color-coded terminal logs showing created vs. skipped items.
- 🛠️ **DevOps Ready:** Generate Dockerfiles, Kubernetes manifests, and CI/CD pipelines out of the box.
- 🤖 **Post-Creation Scripts:** Automatically run shell commands (like `git init` or `npm install`) after generation.

---

## 📦 Installation

### From Crates.io

```bash
cargo install quick-arch
```

### Build from Source

```bash
git clone https://github.com/AbdullahNamespace/quick-arch.git
cd quick-arch
cargo build --release
```

The binary will be available at `./target/release/quick-arch`.

---

## 📂 Included Templates

> **Note:** If you installed `quick-arch` via `cargo install`, the **templates are not included** in the binary. You must [clone the repository](https://github.com/AbdullahNamespace/quick-arch) or download the JSON files manually to use them.

The repository includes several pre-configured templates inside the `template/` directory. You can use these as-is or customize them for your needs.

**Available templates:**

- **`template/fastapi.json`**: Complete FastAPI project with Clean Architecture, CQRS, DDD, and Docker support.
- **`template/rust.json`**: Advanced Rust backend (Axum/Tonic) with Workspace, Domain-Driven Design, and gRPC support.
- **`template/flutter.json`**: Clean architecture structure for Flutter apps.
- **`template/frontend.json`**: A generic modern frontend structure (React/Vue/Next.js) ready for TypeScript.

**How to use an example:**

```bash
# Clone the repo first to get the templates
git clone https://github.com/AbdullahNamespace/quick-arch.git
cd quick-arch

# Generate a FastAPI project using the built-in template
quick-arch --config template/fastapi.json

# Generate a Rust backend
quick-arch --config template/rust.json --output ./my_rust_server
```

---

## 🚀 Usage

### Basic Usage

Generate a project using a configuration file:

```bash
quick-arch --config path/to/config.json
```

### Specify Output Directory

```bash
quick-arch --config rust.json --output ./my-rust-project
```

---

## ⚙️ Configuration

The power of `quick-arch` lies in its JSON configuration. Below is the complete structure supported by `v1.0`.

### 1. Project Metadata

Defines the basic info about the project.

```json
{
  "project": {
    "name": "my_awesome_project",
    "type": "rust", // or "fastapi", "flutter", etc.
    "description": "A high performance backend"
  }
}
```

### 2. Features (Variables)

Define flags that control what gets generated. These are the variables you will use in your conditions.

```json
{
  "features": {
    "cqrs": true,
    "ddd": true,
    "kubernetes": false
  }
}
```

### 3. Directories

List directories to create. You can use simple strings or complex objects with conditions.

```json
{
  "directories": [
    "src", // Always created

    {
      "path": "k8s",
      "condition": "$KUBERNETES == true"
      // Only created if feature "kubernetes" is true
    }
  ]
}
```

### 4. Files

List files to create. You can specify initial content inline.

```json
{
  "files": [
    "README.md",
    {
      "path": "src/main.rs",
      "content": "fn main() { println!(\"Hello World\"); }\n"
    },
    {
      "path": "Dockerfile",
      "condition": "$DOCKER == true"
    }
  ]
}
```

### 5. Post-Creation Scripts

Define commands to run **after** the files and directories are generated.

```json
{
  "custom_scripts": {
    "post_create": ["git init", "npm install", "cargo build"]
  }
}
```

- **Linux/macOS:** Commands run via `sh -c`.
- **Windows:** Commands run via `cmd /C`.
- **Execution Directory:** Commands run in the newly created project root.

---

## 🧠 Condition Logic (v1.0)

### How it Works

1.  **Features to Uppercase:** The tool automatically converts all feature keys in the JSON configuration to **UPPERCASE** internally.
2.  **Comparison:** When writing a condition, you must reference the variable in **UPPERCASE**.
3.  **Operators:** Currently, only the equality operator `==` is supported.

**Important:** Comparison is case-insensitive for values (e.g., `True`, `true`, `TRUE` are all valid), but the variable name must match the uppercase key.

### Syntax Examples

Assuming your JSON features are:

```json
{ "kubernetes": false, "docker": true }
```

✅ **Valid Conditions:**

```json
{ "condition": "$KUBERNETES == true" }
{ "condition": "$DOCKER == 'yes'" }
{ "condition": "$AUTHENTICATION == false" }
```

❌ **Not Supported (yet):**

```json
// Complex logic (AND/OR) is not supported in v1.0
{ "condition": "$KUBERNETES == true && $DOCKER == true" }

// Only == is supported
{ "condition": "$KUBERNETES != false" }
```

---

## 📚 Examples

### Example 1: Microservices Architecture

```json
{
  "project": {
    "name": "microservices_platform"
  },
  "features": {
    "user_service": true,
    "payment_service": true,
    "notification_service": false
  },
  "directories": [
    "services",
    {
      "path": "services/user-service",
      "condition": "$USER_SERVICE == true"
    },
    {
      "path": "services/payment-service",
      "condition": "$PAYMENT_SERVICE == true"
    }
  ],
  "files": [
    {
      "path": "docker-compose.yml",
      "condition": "$USER_SERVICE == true || $PAYMENT_SERVICE == true",
      "content": "version: '3.8'\nservices:\n  ..."
    }
  ]
}
```

---

## ⚠️ Known Limitations (v1.0)

- **Logic Engine:** Only supports basic equality checks (`==`). Complex logic (`&&`, `||`, `!=`, `>`) is planned for v2.0.
- **Variable Interpolation:** You cannot use variables inside file content (e.g., `Created by ${PROJECT_NAME}`) yet.
- **Templates:** Not embedded in binary (must provide JSON file path).

---

## 🐛 Troubleshooting

### Issue: "Config file not found"

**Cause:** The path provided is incorrect or relative path issues.
**Solution:**

```bash
# Use absolute path
quick-arch --config /home/user/my_config.json

# Or verify current directory
ls -la
```

### Issue: "Condition is not working"

**Cause:** Remember that variable names in conditions are converted to **UPPERCASE**.
**Solution:**

- JSON: `"docker": true`
- Condition: `$DOCKER == true` ✅
- Condition: `$docker == true` ❌

### Issue: Script Failed

**Cause:** If you see red output during the "Running Scripts" phase, the command failed on your OS.
**Solution:** Check the error message in the terminal. Verify the command syntax is correct for your operating system (Linux/macOS vs Windows) and run it manually in the project folder to debug.

---

## 📅 Roadmap

- [ ] **v1.1:** Support for nested conditions and boolean operators (`&&`, `||`).
- [ ] **v1.2:** Variable interpolation in file contents (e.g., `${PROJECT_NAME}`).
- [ ] **v2.0:** Embedded templates (no need to clone repo).
- [ ] **v2.0:** Interactive mode (`quick-arch init`).
- [ ] **v2.0:** Flag to skip script execution (`--no-scripts`).

---

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---

## 👤 Author

**Abdullah Abdulkhaleq**

- GitHub: [@AbdullahNamespace](https://github.com/AbdullahNamespace)
- Project: [https://github.com/AbdullahNamespace/quick-arch](https://github.com/AbdullahNamespace/quick-arch)

---

<div align="center">
  <b>Made with ❤️ and 🦀</b>
</div>

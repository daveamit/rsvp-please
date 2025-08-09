# Contributing to `rsvp please!` 🎉

First and foremost, thank you for considering contributing to `rsvp please!`! We're thrilled to have your interest and help in building a robust and user-friendly RSVP system. Your contributions make a real difference, and we appreciate your time and effort.

This document outlines the guidelines for contributing to this project. By participating, you agree to abide by our Code of Conduct.

---

## 🤝 Code of Conduct

We are committed to providing a welcoming and inclusive environment for everyone. Please read and adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## ✨ How to Contribute

There are many ways you can contribute to `rsvp please!`, even if you're not a developer:

1.  **🐛 Bug Reports:** Found a bug? Please open an issue!
2.  **💡 Feature Requests:** Have an idea for a new feature or improvement? Let us know by opening an issue.
3.  **✍️ Documentation Improvements:** Help us improve our documentation, examples, or tutorials.
4.  **💻 Code Contributions:** Submit pull requests for bug fixes, new features, or performance improvements.

---

## 🐞 Reporting Bugs

When reporting a bug, please include as much detail as possible:

* A clear and concise description of the bug.
* Steps to reproduce the behavior.
* Expected behavior.
* Screenshots or animated GIFs if applicable.
* Your operating system, Rust version, and any other relevant environment details.

---

## 🚀 Suggesting Features

When suggesting a new feature, please:

* Clearly describe the feature and its purpose.
* Explain why you think it would be valuable to the project.
* Provide any mockups or examples if you have them.

---

## 🛠️ Setting Up Your Development Environment

To contribute code, you'll need to set up your local development environment.

### Prerequisites

Make sure you have the following installed:

* **Rust & Cargo:** Follow the official Rust installation guide. This method works across **Windows, Linux, and macOS**:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
    On Windows, you might need to install build tools (like Visual Studio Build Tools). `rustup` will typically guide you through this.

* **Containerization Tool (Docker or Podman):** Used for running the database. Choose one that suits your environment:
    * **Docker:**
        * **Windows & macOS:** Install [Docker Desktop](https://www.docker.com/products/docker-desktop/).
        * **Linux:** Follow the specific instructions for your distribution on the [Docker Engine installation page](https://docs.docker.com/engine/install/).
    * **Podman:**
        * **Windows & macOS:** Install [Podman Desktop](https://podman-desktop.io/downloads).
        * **Linux:** Install via your distribution's package manager (e.g., `sudo apt install podman` on Debian/Ubuntu, `sudo dnf install podman` on Fedora). More details at [podman.io](https://podman.io/docs/installation).

### Getting Started

1.  **Fork the Repository:** Click the "Fork" button at the top right of the [GitHub repository](https://github.com/daveamit/rsvp-please).
2.  **Clone Your Fork:**
    ```bash
    git clone [https://github.com/daveamit/rsvp-please.git](https://github.com/daveamit/rsvp-please.git)
    cd rsvp-please
    ```
    (Remember to replace `daveamit` with your GitHub username if you're working on a different fork.)

### Building and Running the Project (All Rust)

The entire project is built with Rust.

1.  **Build the Project:**
    ```bash
    cargo build
    ```
2.  **Run Tests:**
    ```bash
    cargo test
    ```
3.  **Run the Application:**
    ```bash
    cargo run
    ```
    This command will build and execute the main `rsvp please!` binary. Depending on the project, this might start a web server (e.g., on `http://127.0.0.1:8080`), a command-line application, or another service. Check the console output for specific details.

### Database (PostgreSQL with Docker/Podman)

We use PostgreSQL for the database. Using a containerization tool like Docker or Podman is the recommended way to get it running locally.

#### Using Docker Compose (Recommended)

This method typically uses a `docker-compose.yml` file to manage the database container.

1.  **Start the Database Container:**
    ```bash
    docker-compose up -d postgres
    ```
    This will start a PostgreSQL container in the background. You might need to wait a moment for it to initialize.

#### Using Podman Compose (Alternative)

If you prefer Podman, you might use `podman-compose` or directly use `podman` commands if `docker-compose.yml` is not explicitly compatible.

1.  **Start the Database Container with Podman Compose:**
    ```bash
    podman-compose up -d postgres # If podman-compose is installed and configured
    ```
2.  **Alternative (Direct Podman Command):**
    If `podman-compose` isn't used, you might manually start a PostgreSQL container:
    ```bash
    podman run -d --name rsvp-postgres -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -p 5432:5432 postgres:latest
    ```
    (Adjust user/password/port as needed for the project's configuration).

3.  **Database Migrations:** After the database container is running, you'll need to apply migrations to set up the database schema. This typically involves a Rust-native migration tool.
    * *(Example using a hypothetical `sqlx-cli` or `diesel_cli` for Rust-based migrations)*:
        ```bash
        # Assuming sqlx-cli or similar is used for migrations
        cargo install sqlx-cli # if not already installed
        sqlx database setup
        ```
    * *(Alternatively, if your project uses raw SQL files, provide instructions for connecting to the DB and running them).*

---

## 🧪 Running Tests

Ensure your changes pass all existing tests before submitting a pull request.

* **Run All Tests:**
    ```bash
    cargo test
    ```

---

## ⬆️ Submitting Pull Requests

When you're ready to submit your code, please follow these steps:

1.  **Create a New Branch:**
    ```bash
    git checkout -b feature/your-feature-name-or-bug-fix
    ```
    Choose a descriptive branch name.
2.  **Make Your Changes:** Implement your feature or fix.
3.  **Write Tests:** Add or update tests to cover your changes.
4.  **Run Tests & Linting:** Ensure all tests pass and your code adheres to our style guidelines.
    ```bash
    cargo clippy -- -D warnings # Rust linter
    cargo fmt --check # Rust formatter check
    ```
5.  **Commit Your Changes:** Write clear, concise commit messages.
    ```bash
    git commit -m "feat: Add new awesome feature"
    ```
    (See [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for guidance on commit messages).
6.  **Push to Your Fork:**
    ```bash
    git push origin feature/your-feature-name-or-bug-fix
    ```
7.  **Open a Pull Request:** Go to your fork on GitHub and click "New Pull Request" against the `main` branch of the upstream repository.
    * Provide a clear title and description for your PR.
    * Reference any related issues (e.g., "Closes #123").

---

## 📏 Code Style and Linting

We use automated tools to maintain code consistency:

* **Rust:** We use `rustfmt` for formatting and `clippy` for linting. Please run these before committing:
    ```bash
    cargo fmt
    cargo clippy --fix
    ```

---

## 📧 Contact

If you have any questions or need further clarification, feel free to open an issue or reach out to the project maintainers directly.

---

## ⚖️ License

By contributing to `rsvp please!`, you agree that your contributions will be licensed under the project's [LICENSE](LICENSE) file.

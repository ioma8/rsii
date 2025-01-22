# 🌟🚀 Rust Smart Interactive Interface (RSII)

Ever wanted your terminal to be smarter? RSII uses Rust and OpenAI to turn your queries into system commands. It's like having a clever assistant in your command line.

## Features

- Fetches system information and user queries.
- Integrates with OpenAI's Chat Completion API.
- Executes commands based on AI responses.
- Copies commands to clipboard for easy execution.
- Supports macOS, Linux, and Windows for command execution.

## Installation

1. Ensure Rust is installed on your system. You can download and install Rust from [rust-lang.org](https://www.rust-lang.org/learn/get-started).
2. Install the RSII package using Cargo:
    ```sh
    cargo install rsii
    ```

3. Edit a configuration file at `~/.rsii/config.toml`:
    ```toml
    [default]
    model = "your-model" # gpt-4o-mini
    api-key = "your-api-key" # sk-....
    ```

## Usage

Run the application with a user query:
```sh
rsii "Your query here"
```

The application will fetch system information, process the query using OpenAI's API, and execute the corresponding command.

## License

This project is licensed under the MIT License.
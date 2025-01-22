# RSII - Rust AI Command-line Assistant

RSII is a Rust-based application that integrates with OpenAI's API to process user queries and execute commands. It leverages the power of AI to provide intelligent responses and automate tasks.

## Features

- Fetches system information and user queries.
- Integrates with OpenAI's Chat Completion API.
- Executes commands based on AI responses.
- Copies commands to clipboard for easy execution.
- Supports macOS, Linux, and Windows for command execution.

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/ioma8/rsii.git
    cd rsii
    ```

2. Install dependencies:
    ```sh
    cargo build
    ```

3. Create a configuration file `rsii_config.toml` in your home directory with the following content:
    ```toml
    [default]
    model = "your-model" # gpt-4o-mini
    api-key = "your-api-key" # sk-....
    system-prompt = "your-system-prompt" # copy sample from default.config.toml
    ```

## Usage

Run the application with a user query:
```sh
cargo run -- "Your query here"
```

The application will fetch system information, process the query using OpenAI's API, and execute the corresponding command.

## License

This project is licensed under the MIT License.
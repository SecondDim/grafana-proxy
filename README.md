# Grafana to Power Automate Webhook Proxy

This is a simple Rust-based proxy server that receives webhook notifications from Grafana and forwards them to a specified Microsoft Power Automate webhook URL.

## Features

* Forwards Grafana alerts to Microsoft Power Automate(Teams).
* Configurable Power Automate webhook URL via command-line arguments.
* Accepts invalid SSL certificates for easier testing in development environments.
* Cross-platform compilation support.

## Prerequisites

* Rust programming language and Cargo package manager. You can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

## Development Environment

* **Rust Version:** `rustc 1.88.0 (6b00bc388 2025-06-23)`
* **Cargo Version:** `cargo 1.88.0 (873a06493 2025-05-10)`
* **Operating System:** `Windows`

## Installation & Building

1. **Clone the repository:**

    ```bash
    git clone https://github.com/SecondDim/grafana-proxy.git
    cd grafana-proxy
    ```

1. **Build for development:**

    ```bash
    cargo build
    ```

1. **Build for release:**

    ```bash
    cargo build --release
    ```

1. **Build for a specific target (e.g., x86_64 Linux):**

    ```bash
    rustup target add x86_64-unknown-linux-gnu
    cargo build --release --target=x86_64-unknown-linux-gnu
    ```

## Usage

To run the proxy, you need to provide the Power Automate webhook URL as a command-line argument. You can also optionally specify the binding IP address and port.

```bash
./target/release/grafana-proxy --webhook-url <your-power-automate-webhook-url> [--bind-ip <ip-address>] [--bind-port <port>]
```

By default, the server will start on `http://0.0.0.0:3080`.

### Example

```bash
# Run with default IP and port
./target/release/grafana-proxy --webhook-url "https://prod-123.westeurope.logic.azure.com:443/workflows/..."

# Run with a specific IP and port
./target/release/grafana-proxy --webhook-url "https://prod-123.westeurope.logic.azure.com:443/workflows/..." --bind-ip 127.0.0.1 --bind-port 8888
```

Then, configure your Grafana notification channel to send webhooks to `http://<your-server-ip>:<port>/grafana-webhook`.

## License

This project is licensed under the MIT License.

---

## Grafana 到 Power Automate Webhook 代理

這是一個基於 Rust 的簡易代理伺服器，可接收來自 Grafana 的 Webhook 通知，並將其轉發到指定的 Microsoft Power Automate Webhook URL

## 功能

* 將 Grafana 警示轉發到 Microsoft Power Automate(Teams)。
* 可透過命令列參數設定 Power Automate Webhook URL。
* 接受無效的 SSL 憑證，以便在開發環境中輕鬆測試。
* 支援跨平台編譯。

## 先決條件

* Rust 程式語言和 Cargo 套件管理器。您可以從 [rust-lang.org](https://www.rust-lang.org/tools/install) 安裝它們。

## 開發環境

* **Rust 版本:** `rustc 1.88.0 (6b00bc388 2025-06-23)`
* **Cargo 版本:** `cargo 1.88.0 (873a06493 2025-05-10)`
* **作業系統:** `Windows`

## 安裝與建置

1. **複製儲存庫：**

    ```bash
    git clone https://github.com/SecondDim/grafana-proxy.git
    cd grafana-proxy
    ```

1. **開發建置：**

    ```bash
    cargo build
    ```

1. **發行建置：**

    ```bash
    cargo build --release
    ```

1. **針對特定目標進行建置 (例如 x86_64 Linux)：**

    ```bash
    rustup target add x86_64-unknown-linux-gnu
    cargo build --release --target=x86_64-unknown-linux-gnu
    ```

## 使用方式

若要執行代理，您需要提供 Power Automate Webhook URL 作為命令列參數。您也可以選擇性地指定綁定的 IP 位址和埠號。

```bash
./target/release/grafana-proxy --webhook-url <your-power-automate-webhook-url> [--bind-ip <ip-address>] [--bind-port <port>]
```

預設情況下，伺服器將在 `http://0.0.0.0:3080` 上啟動。

### 範例

```bash
# 使用預設 IP 和埠號執行
./target/release/grafana-proxy --webhook-url "https://prod-123.westeurope.logic.azure.com:443/workflows/..."

# 使用指定的 IP 和埠號執行
./target/release/grafana-proxy --webhook-url "https://prod-123.westeurope.logic.azure.com:443/workflows/..." --bind-ip 127.0.0.1 --bind-port 8888
```

然後，設定您的 Grafana 通知頻道，將 Webhook 發送到 `http://<your-server-ip>:<port>/grafana-webhook`。

## 授權

本專案採用 MIT 授權。

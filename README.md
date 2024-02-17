---

# NewsGrid: Decentralized Autonomous System for News Articles

NewsGrid is a decentralized autonomous system designed to empower users with full control over what news articles trend and what is considered important. Built with Rust and utilizing the Internet Computer Protocol (ICP), NewsGrid offers a revolutionary approach to news curation and dissemination.

## Features

- Decentralized news curation: Users have full control over the trending news articles.
- Transparency and trust: Utilizing blockchain technology ensures transparency in the selection process.
- Community-driven: Users collectively decide the importance and relevance of news articles.

## Requirements

- Rust programming language
- Internet Computer Protocol (ICP)
- DFINITY Canister SDK (DFX)

## Getting Started

To run NewsGrid locally, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/newsgrid.git
    ```

2. Navigate to the project directory:

    ```bash
    cd newsgrid
    ```

3. Build the project:

    ```bash
    cargo build
    ```

4. Deploy the project using DFX:

    ```bash
    dfx deploy
    ```

5. Interact with the deployed canister using DFX:

    ```bash
    dfx canister call newsgrid <method-name> <arguments>
    ```

## Usage

Once the project is deployed, users can interact with the NewsGrid system using the provided DFX commands. Here are some example commands:

- Submit a news article:
    ```bash
    dfx canister call newsgrid submit_article "<article-url>" "<article-title>"
    ```

- Upvote a news article:
    ```bash
    dfx canister call newsgrid upvote_article "<article-id>"
    ```

- Downvote a news article:
    ```bash
    dfx canister call newsgrid downvote_article "<article-id>"
    ```

## Contributing

We welcome contributions from the community! If you have any suggestions, feature requests, or bug reports, please submit them via GitHub issues.

## License

This project is licensed under the [MIT License](LICENSE).

---

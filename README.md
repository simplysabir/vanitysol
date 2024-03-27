# VanitySol

## Overview

VanitySol is a command-line interface (CLI) tool developed in Rust, specifically designed to generate Solana vanity addresses with a desired prefix. Leveraging the Solana CLI tools, it streamlines the process of creating personalized Solana addresses, making it both quick and straightforward for users to obtain vanity addresses that start with specific characters of their choosing.

## Features

- **Vanity Address Generation**: Generate Solana addresses that start with a custom prefix specified by the user.
- **Integration with Solana CLI**: Utilizes `solana-keygen grind` for reliable and secure address generation.
- **Simple and Efficient**: A user-friendly command-line interface simplifies the process, making vanity address generation accessible to everyone.

## Installation

Before using VanitySol, make sure Rust and the Solana CLI tools are installed on your system. Follow these steps to set up VanitySol:

```sh
git clone https://github.com/yourgithub/vanitysol.git
cd vanitysol
cargo build --release
```

Ensure you have `solana-keygen` installed as part of the Solana CLI tools. If not, refer to the Solana documentation for installation instructions.

The compiled binary will be located in the `target/release/` directory. This directory will contain the `vanitysol` executable.

## Usage

To generate a Solana vanity address with your desired prefix, run the following command:

```sh
./target/release/vanitysol --vanity PREFIX_HERE generate
```

Replace `PREFIX_HERE` with the prefix you want your Solana vanity address to start with. For example, if you want an address that starts with "sol", you would use:

```sh
./target/release/vanitysol --vanity sol generate
```

or simply download install the crate from [here](https://crates.io/crates/vanitysol)
and run this command
```sh
vanitysol --vanity `PREFIX_HERE` generate
```

The tool will output the path to the file containing the generated keypair. Ensure to note this path and handle the keypair file securely.

## Error Handling

VanitySol provides informative error messages to help diagnose issues related to Solana CLI tool integration, network errors, or invalid prefixes.

## Contributions

Contributions are highly appreciated! If you're interested in enhancing VanitySol, please fork the repository and submit a pull request with your updates. For major changes or suggestions, please open an issue first to discuss what you would like to change.

Feel free to explore more projects and contribute via [GitHub](https://github.com/simplysabir).

## License

VanitySol is released under the MIT License. For more details, see the LICENSE file in the repository.

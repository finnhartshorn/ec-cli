# ec-cli

A command-line interface for [Everybody Codes](https://everybody.codes) puzzles, inspired by [aoc-cli](https://github.com/scarvalhojr/aoc-cli).

## Features

- Download and decrypt puzzle inputs and descriptions
- Display puzzle descriptions in the terminal
- Submit puzzle answers
- Automatic sample/example extraction
- Cookie-based authentication
- File organization matching the [Golang EC CLI](https://github.com/robryanx/everybodycodes/tree/main/cli)

## Installation

```bash
git clone https://github.com/yourusername/ec-cli.git
cd ec-cli
cargo build --release
```

The binary will be available at `target/release/ec-cli`.

## Setup

### Authentication

You need to provide your Everybody Codes session cookie. The CLI looks for it in this order:

1. `EC_COOKIE` environment variable
2. `~/.everybodycodes.cookie` file
3. Platform-specific config directory (`~/.config/everybodycodes/cookie` on Linux)

To get your cookie:

1. Log into https://everybody.codes in your browser
2. Open browser developer tools (F12)
3. Go to the Network or Application tab
4. Find the cookie value
5. Set it as an environment variable or save to a file:

```bash
# Option 1: Environment variable
export EC_COOKIE="your-cookie-value"

# Option 2: Save to file
echo "your-cookie-value" > ~/.everybodycodes.cookie
```

## Usage

### Fetch Command

Download and decrypt puzzle inputs and descriptions:

```bash
# Fetch input for day 5, part 2 (2024 by default)
ec-cli fetch -d 5 -p 2

# Fetch for a specific year
ec-cli fetch -y 2024 -d 10 -p 1

# Fetch description only (skip input)
ec-cli fetch -d 3 -p 1 --description-only

# Fetch input only (skip description)
ec-cli fetch -d 7 -p 3 --input-only
```

Files are saved to:
- Inputs: `{year}/inputs/{day}-{part}.txt`
- Descriptions: `{year}/descriptions/{day}.html`
- Samples: `{year}/samples/{day}-{part}.txt` (extracted from description)

### Read Command

Display a puzzle description in your terminal:

```bash
# Read day 5 description (fetches if not already downloaded)
ec-cli read -d 5

# Specify terminal width for text wrapping
ec-cli read -d 5 -w 100

# Read from a specific year
ec-cli read -y 2024 -d 1
```

### Submit Command

Submit an answer for a puzzle:

```bash
# Submit answer for day 5, part 2
ec-cli submit -d 5 -p 2 "your-answer"

# Submit for a specific year
ec-cli submit -y 2024 -d 10 -p 3 "12345"
```

The command will display:
- Whether the answer is correct
- Global placement and score
- Timing information
- Any server messages

### Global Options

```bash
# Enable debug logging
ec-cli -d fetch -d 5 -p 1

# Quiet mode (errors only)
ec-cli -q fetch -d 5 -p 1

# Show version
ec-cli --version

# Show help
ec-cli --help
ec-cli fetch --help
```

## File Organization

The CLI organizes files by year, matching the Golang EC CLI structure:

```
2024/
├── inputs/
│   ├── 1-1.txt
│   ├── 1-2.txt
│   └── 1-3.txt
├── descriptions/
│   └── 1.html
└── samples/
    ├── 1-1.txt
    ├── 1-2.txt
    └── 1-3.txt
```

## Examples

### Typical Workflow

```bash
# 1. Fetch puzzle and input
ec-cli fetch -d 5 -p 1

# 2. Read the description
ec-cli read -d 5

# 3. Solve the puzzle (use your own solver)
# The input is at: 2024/inputs/5-1.txt
# Samples are at: 2024/samples/5-1.txt

# 4. Submit your answer
ec-cli submit -d 5 -p 1 "your-answer"
```

### Fetch All Parts for a Day

```bash
# Fetch all three parts
ec-cli fetch -d 5 -p 1
ec-cli fetch -d 5 -p 2
ec-cli fetch -d 5 -p 3
```

## Architecture

The CLI is built with:

- **Language**: Rust 2021 edition
- **CLI Framework**: Clap v4 (derive macros)
- **HTTP Client**: reqwest
- **Async Runtime**: tokio
- **Encryption**: AES-128-CBC with PKCS7 padding
- **HTML Parsing**: html2text for terminal display

### Project Structure

```
src/
├── main.rs       # CLI entry point and command routing
├── cli.rs        # Clap command definitions
├── client.rs     # API client for Everybody Codes
├── crypto.rs     # AES-CBC decryption
├── storage.rs    # File I/O operations
├── display.rs    # HTML parsing and formatting
├── error.rs      # Error types
└── models.rs     # Data structures
```

## Comparison with Other CLIs

### vs. aoc-cli (for Advent of Code)

**Similarities:**
- Multi-command structure (fetch/read/submit)
- Smart defaults and validation
- Cookie-based authentication
- Terminal-friendly display

**Differences:**
- EC-CLI handles encrypted content (AES-CBC decryption)
- EC-CLI has 3 parts per day (vs. 2 for AoC)
- EC-CLI extracts samples from HTML
- Different API endpoints and data models

### vs. Golang EC CLI

**Similarities:**
- Same authentication method (cookie)
- Same file organization structure
- Same API endpoints and encryption

**Differences:**
- EC-CLI uses multi-command structure (vs. single command with flags)
- EC-CLI written in Rust (vs. Go)
- EC-CLI has terminal display features
- Future: EC-CLI will support day/part ranges

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT

## Credits

- Inspired by [aoc-cli](https://github.com/scarvalhojr/aoc-cli) by @scarvalhojr
- Based on the [Golang EC CLI](https://github.com/robryanx/everybodycodes/tree/main/cli) by @robryanx
- Built for the [Everybody Codes](https://everybody.codes) platform

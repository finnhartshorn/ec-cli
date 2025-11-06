# EC-CLI Architecture

## Overview

EC-CLI is a Rust-based command-line interface for Everybody Codes puzzles, designed with a modular architecture that separates concerns and enables easy maintenance and extension.

## Design Principles

1. **Separation of Concerns**: Each module has a single, well-defined responsibility
2. **Error Handling**: Comprehensive error types with helpful messages
3. **User-Friendly CLI**: Multi-command structure inspired by aoc-cli
4. **Secure Authentication**: Cookie-based auth with multiple fallback locations
5. **File Organization**: Matches the Golang EC CLI structure for compatibility

## Module Structure

```
src/
├── main.rs       # Application entry point and command routing
├── cli.rs        # CLI command definitions (Clap)
├── client.rs     # HTTP client and API interactions
├── crypto.rs     # AES-CBC decryption
├── storage.rs    # File system operations
├── display.rs    # HTML parsing and formatting
├── error.rs      # Error types and Result alias
└── models.rs     # Data structures
```

## Module Details

### main.rs

**Responsibility**: Application bootstrapping and command routing

**Key Functions**:
- `main()`: Entry point, parses CLI args, sets up logging
- `handle_fetch()`: Orchestrates fetching puzzles and inputs
- `handle_read()`: Displays puzzle descriptions
- `handle_submit()`: Submits answers to the server

**Flow**:
```
Parse CLI args → Validate → Route to handler → Execute → Display result
```

### cli.rs

**Responsibility**: CLI interface definition using Clap

**Commands**:
- `fetch`: Download and decrypt puzzles/inputs
- `read`: Display puzzle in terminal
- `submit`: Submit answers

**Validation**:
- Year: 2024-2030
- Day: 1-20
- Part: 1-3

**Design Notes**:
- Uses Clap's derive macros for clean, declarative syntax
- Global options (--debug, --quiet) available to all commands
- Validation happens before command execution

### client.rs

**Responsibility**: HTTP API interactions with Everybody Codes

**Key Struct**: `EcClient`

**Methods**:
- `new()`: Create client, load authentication
- `get_user_seed()`: Fetch and cache user seed
- `fetch_quest_keys()`: Get decryption keys (key1, key2, key3)
- `fetch_input()`: Download and decrypt puzzle input
- `fetch_description()`: Download and decrypt puzzle description
- `submit_answer()`: POST answer to server

**API Endpoints**:
```
GET  /api/user/me                                   # User seed
GET  /api/event/{year}/quest/{day}                 # Quest keys
GET  /assets/{year}/{day}/input/{seed}.json        # Encrypted input (CDN)
GET  /assets/{year}/{day}/description.json         # Encrypted description (CDN)
POST /api/event/{year}/quest/{day}/part/{part}/answer  # Submit answer
```

**Authentication Flow**:
1. Load cookie (env var → ~/.everybodycodes.cookie → config dir)
2. Send cookie with all requests
3. Cache user seed after first fetch

### crypto.rs

**Responsibility**: AES-CBC decryption with PKCS7 padding

**Function**: `decrypt_aes_cbc(ciphertext_hex: &str, key: &str) -> Result<String>`

**Algorithm**:
1. Decode hex-encoded ciphertext
2. Extract key (first 16 bytes)
3. Use key as IV (AES-128-CBC convention)
4. Decrypt with PKCS7 unpadding
5. Convert to UTF-8 string

**Dependencies**:
- `aes`: AES cipher implementation
- `cbc`: CBC mode
- `hex`: Hex encoding/decoding

### storage.rs

**Responsibility**: File system operations

**Key Struct**: `Storage`

**Directory Structure**:
```
{base_path}/
└── {year}/
    ├── inputs/{day}-{part}.txt
    ├── descriptions/{day}.html
    └── samples/{day}-{part}.txt
```

**Methods**:
- `save_input()`: Save decrypted input
- `save_description()`: Save HTML description
- `save_sample()`: Save extracted samples
- `load_input()`: Read input from disk
- `load_description()`: Read description from disk
- `has_input()`: Check if file exists
- `has_description()`: Check if file exists

**Design Notes**:
- Automatically creates directories as needed
- Returns `PathBuf` after save for logging
- Base path defaults to current directory

### display.rs

**Responsibility**: HTML parsing and terminal formatting

**Functions**:
- `extract_samples(html: &str) -> Vec<String>`: Extract sample data from HTML
- `html_to_text(html: &str, width: usize) -> String`: Convert HTML to text
- `format_submit_response(response: &SubmitResponse) -> String`: Format submission result

**Sample Extraction**:
- Uses regex: `<pre class="note">(.*?)</pre>`
- Extracts all examples/samples from puzzle description
- Returns as vector for separate storage

**HTML Conversion**:
- Uses `html2text` crate
- Wraps to terminal width
- Preserves formatting where possible

### error.rs

**Responsibility**: Error types and conversions

**Error Variants**:
- `MissingCookie`: Authentication not configured
- `InvalidDay/Part/Year`: Validation failures
- `DecryptionError`: Crypto failures
- `ApiError`: HTTP request failures
- `AlreadySubmitted`: 409 response from server
- `IoError`: File system errors
- `JsonError`: JSON parsing errors
- `HexError`: Hex decoding errors
- `HttpError`: Non-success status codes

**Design Notes**:
- Uses `thiserror` for clean error definitions
- Automatic `From` implementations for library errors
- Custom `Result<T>` type alias for convenience

### models.rs

**Responsibility**: Data structures for API responses

**Structs**:
- `User`: User profile with seed
- `QuestKeys`: Decryption keys (key1, key2, key3)
- `AnswerPayload`: Submission request body
- `SubmitResponse`: Submission result
- `Quest`: Quest identifier (year, day, part)

**Design Notes**:
- Uses `serde` for JSON serialization/deserialization
- `QuestKeys::get_key(part)` helper for selecting correct key
- `camelCase` mapping for API compatibility

## Data Flow

### Fetch Command

```
User input (day, part)
    ↓
CLI validation
    ↓
EcClient::new() → Load cookie
    ↓
EcClient::get_user_seed() → API call, cache result
    ↓
EcClient::fetch_quest_keys() → API call for keys
    ↓
EcClient::fetch_description() → CDN download
    ↓
decrypt_aes_cbc() → Decrypt with key1
    ↓
Storage::save_description() → Write to disk
    ↓
extract_samples() → Parse HTML
    ↓
Storage::save_sample() → Write samples to disk
    ↓
EcClient::fetch_input() → CDN download (with seed)
    ↓
decrypt_aes_cbc() → Decrypt with part-specific key
    ↓
Storage::save_input() → Write to disk
```

### Read Command

```
User input (day)
    ↓
Storage::has_description() → Check cache
    ↓
If cached:
    Storage::load_description() → Read from disk
Else:
    EcClient::fetch_description() → Download
    Storage::save_description() → Cache for later
    ↓
html_to_text() → Convert to terminal-friendly format
    ↓
Print to stdout
```

### Submit Command

```
User input (day, part, answer)
    ↓
EcClient::new() → Load cookie
    ↓
EcClient::submit_answer() → POST to API
    ↓
Parse SubmitResponse
    ↓
format_submit_response() → Format for display
    ↓
Print to stdout
```

## Dependencies

### Core Dependencies
- `clap` (4.5): CLI framework with derive macros
- `reqwest` (0.12): HTTP client with async support
- `tokio` (1.43): Async runtime
- `serde` (1.0): Serialization framework
- `serde_json` (1.0): JSON support

### Crypto Dependencies
- `aes` (0.8): AES cipher
- `cbc` (0.1): CBC mode
- `hex` (0.4): Hex encoding/decoding

### Utility Dependencies
- `thiserror` (1.0): Error handling
- `anyhow` (1.0): Error context
- `env_logger` (0.11): Logging backend
- `log` (0.4): Logging facade
- `html2text` (0.12): HTML to text conversion
- `regex` (1.11): Regular expressions
- `dirs` (5.0): Home/config directory discovery
- `term_size` (0.3): Terminal dimensions

## Future Enhancements

### Planned Features
1. **Range Support**: Fetch multiple days/parts in one command
   - Already validated in CLI, needs implementation in handlers
   - Would add parallel downloads for efficiency

2. **Progress Tracking**: Visual display of completed quests
   - Track submissions locally
   - Display calendar-style view

3. **Solver Integration**: Execute solver programs and capture output
   - Similar to Golang EC CLI
   - `--solver` flag to specify program

4. **Caching**: Reduce API calls
   - Cache quest keys (currently fetched each time)
   - Optional input caching
   - TTL-based cache invalidation

5. **Configuration File**: User preferences
   - Default year
   - Terminal width
   - Cookie storage preference

6. **Leaderboard Support**: If API supports it
   - Personal stats
   - Friends leaderboard

### Architecture Considerations for Extensions

**Range Support**:
```rust
// cli.rs
#[derive(Debug, Clone)]
pub enum DaySpec {
    Single(i32),
    Range(i32, i32),
}

// main.rs
async fn handle_fetch_range(days: Vec<i32>, parts: Vec<i32>) -> Result<()> {
    // Parallel downloads with tokio::spawn
}
```

**Progress Tracking**:
```rust
// Add new module: src/progress.rs
pub struct ProgressTracker {
    db: SqliteConnection, // or JSON file
}

impl ProgressTracker {
    pub fn record_submission(&mut self, quest: Quest, correct: bool);
    pub fn get_completions(&self, year: i32) -> Vec<Completion>;
    pub fn display_calendar(&self, year: i32);
}
```

**Solver Integration**:
```rust
// Add to client.rs or new module
pub async fn run_solver(path: &Path, input_path: &Path) -> Result<String> {
    let output = Command::new(path)
        .arg(input_path)
        .output()
        .await?;

    // Extract answer from last line of stdout
    let answer = output.stdout.lines().last().unwrap();
    Ok(answer)
}
```

## Testing Strategy

### Unit Tests
- `crypto.rs`: Test encryption/decryption with known pairs
- `display.rs`: Test HTML parsing and sample extraction
- `cli.rs`: Test validation logic
- `storage.rs`: Test file operations with temp directories

### Integration Tests
- Mock HTTP server for API tests
- Test full fetch → decrypt → save flow
- Test error handling (missing cookie, network errors)

### E2E Tests
- Requires test credentials
- Test against real API (in CI with secrets)
- Verify full workflows

## Performance Considerations

1. **Async I/O**: All API calls use async/await
2. **Caching**: User seed cached after first fetch
3. **Parallel Downloads**: Future range support will use tokio::spawn
4. **Minimal Dependencies**: Only essential crates included
5. **Release Build**: Use `--release` for production (~10x faster)

## Security Considerations

1. **Cookie Storage**: Files have restricted permissions
2. **No Credential Logging**: Cookie never logged, even in debug mode
3. **HTTPS Only**: All API calls use HTTPS
4. **Input Validation**: All user inputs validated before use
5. **No SQL Injection**: No database (yet), but would use parameterized queries

## Comparison with Reference Implementations

### vs. aoc-cli (Rust)
- **Shared**: Multi-command structure, cookie auth, terminal display
- **EC-CLI specific**: Encryption handling, 3 parts vs 2, sample extraction

### vs. Golang EC CLI
- **Shared**: File structure, API endpoints, encryption algorithm
- **EC-CLI specific**: Multi-command vs single command, Rust vs Go, richer CLI

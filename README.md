# URL Shortener Backend

A high-performance URL shortening service built with Rust using the Axum web framework and Sled embedded database.


## Getting Started

This ENTIRELY is a personal project for me. I'm going to make a URL Shortner in Rust using Sled as the primary database and axum and tokio for networking.

### Installation

1. Clone the repository:

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:3000` by default

## API Endpoints

### GET /
Returns a welcome message.

**Response:**
```
HTTP/1.1 200 OK
Welcome to URL SHORTNER API
```

### POST /new-url
Creates a new shortened URL.

**Request:**
```json
{
  "url": "https://example.com/very/long/url/path"
}
```

**Response:**
```json
{
  "shortened_key": "a"
}
```

**Status Codes:**
- `200 OK`: URL successfully shortened
- `400 Bad Request`: Invalid request or error during processing

## Project Structure

```
backend/
├── src/
│   ├── main.rs              # Server initialization and routing
│   ├── lib.rs               # AppState definition
│   └── url_management/
│       ├── mod.rs           # Module exports
│       ├── api.rs           # HTTP request handlers
│       └── new_url.rs       # URL shortening logic
├── db/                      # Sled database files (created at runtime)
├── test/
│   └── api.test.js          # Integration tests (Bun)
├── Cargo.toml               # Project dependencies
└── README.md                # This file
```

## How It Works

### URL Shortening Algorithm

The service generates sequential, incrementing keys using a base-26 alphabet (a-z):

1. **First URL**: `a`
2. **Second URL**: `b`
3. **After 26 URLs**: `ba`
4. **Pattern**: Similar to Excel column naming (a, b, ..., z, ba, bb, ...)

This approach ensures:
- **Compact keys**: Short, human-readable identifiers
- **Deterministic**: Same length URLs generate sequential keys
- **Efficient storage**: O(1) key generation

### Data Flow

1. Client sends `POST /new-url` with a long URL
2. `new_url_api` handler receives and validates the request
3. `new_url` function generates a new key by incrementing the last key
4. URL and key are stored in Sled tree
5. Shortened key is returned to client

## Database

The project uses **Sled**, an embedded key-value store:

- **Location**: `./db` directory (created automatically on first run)
- **Tree**: `url` - stores the mapping of shortened keys to full URLs
- **Persistence**: Data persists between server restarts
- **No network dependency**: Database is embedded, no external DB required

## Testing

Run the test suite using Bun:

```bash
bun test test/api.test.js
```

Or with a custom server URL:

```bash
BASE_URL=http://localhost:3000 bun test test/api.test.js
```

Current tests:
- `GET /` - Verifies welcome message endpoint

## Development

### Building

```bash
cargo build
```

### Running in Development Mode

```bash
cargo run
```

### Linting and Formatting

```bash
cargo clippy
cargo fmt
```


## Troubleshooting

### Database Lock Issues
If you encounter database lock errors, ensure:
- Only one instance of the server is running
- The `./db` directory is not corrupted
- Try removing the `./db` directory and restarting (data will be lost)
``

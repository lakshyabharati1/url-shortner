# URL Shortener API Tests

This directory contains integration tests for the URL Shortener backend API.

## Running Tests

### Prerequisites

- Bun runtime installed
- Backend server running on `http://127.0.0.1:3000`

### Start the Backend Server

In a separate terminal, run:

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

### Run All Tests

```bash
bun test test/api.test.js
```

### Run Tests with Custom Server URL

```bash
BASE_URL=http://localhost:8080 bun test test/api.test.js
```

### Run Tests with Verbose Output

```bash
bun test --verbose test/api.test.js
```

## Test Coverage

The test suite includes:

### Home Endpoint Tests
- ✅ `GET /` returns `URL_SHORTNER_API`
- ✅ `GET /` returns 200 OK status

### Redirect Tests
- ✅ `GET /{shortened_key}` returns a permanent redirect
- ✅ Redirect `Location` header matches the stored URL

### URL Shortening Tests
- ✅ `POST /new-url` creates a shortened URL
- ✅ `POST /new-url` returns different keys for different URLs
- ✅ `POST /new-url` with valid URL creates shortened key
- ✅ `POST /new-url` with long URL creates shortened key
- ✅ `POST /new-url` response has correct JSON structure
- ✅ `POST /new-url` handles various URL formats

### Admin Endpoint Tests
- ✅ `GET /admin/get_entries` requires admin_verification_code
- ✅ `GET /admin/get_entries` returns stored entries as arrays of [key, url]
- ✅ `GET /admin/get_entries` respects the count parameter

### Error Handling Tests
- ✅ `GET /nonexistent` returns 404


## Admin Endpoint Notes

The admin endpoint is a `GET` route and accepts these query parameters:

```text
/admin/get_entries?admin_verification_code=<code>&search=<optional search prefix>&count=<number>
```

By default the server sets `admin_verification_code` to `1234` in `src/main.rs`. When running the tests against a locally started server, the admin tests use this default code. If the server is configured with a different code, set it accordingly before running the tests.

## Rust Unit Tests

The backend also includes `cargo test` coverage for the URL increment helper and the internal route lookup helper. These tests run without starting the HTTP server.

## Test Structure

Each test is independent and can be run in any order. Tests make HTTP requests to the running backend server and verify:

- HTTP status codes
- Response body format and content
- Response headers
- Data persistence and consistency

## Troubleshooting

### Connection Refused
Make sure the backend server is running on `http://127.0.0.1:3000`

```bash
curl http://127.0.0.1:3000/
```

### Tests Failing
- Clear the database: `rm -rf db/`
- Restart the backend server: `cargo run`
- Re-run tests

### Database State Issues
The database persists between server restarts. If tests are failing due to unexpected state:

1. Stop the backend server
2. Delete the database: `rm -rf db/`
3. Restart the backend server: `cargo run`
4. Re-run the tests

## Adding New Tests

To add new tests:

1. Open `test/api.test.js`
2. Add a new `test()` block using Bun's test API
3. Use the existing `request()` helper function
4. Follow the naming convention: `"METHOD /path description"`

Example:

```javascript
test("POST /new-url handles special characters", async () => {
  const { response, body } = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({
      url: "https://example.com/test?special=!@#$%",
    }),
  });

  expect(response.status).toBe(200);
  expect(body.shortened_key).toBeTruthy();
});
```

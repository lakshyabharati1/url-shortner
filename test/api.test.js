import { expect, test } from "bun:test";

const BASE_URL = process.env.BASE_URL ?? "http://127.0.0.1:3000";

async function request(path, options = {}) {
  const url = new URL(path, BASE_URL);

  if (options.query) {
    for (const [key, value] of Object.entries(options.query)) {
      if (value === undefined || value === null) continue;
      url.searchParams.set(key, String(value));
    }
  }

  const response = await fetch(url, {
    ...options,
    headers: {
      accept: "application/json",
      "content-type": "application/json",
      ...(options.headers ?? {}),
    },
  });

  const contentType = response.headers.get("content-type") ?? "";

  const body = contentType.includes("application/json")
    ? await response.json()
    : await response.text();

  return { response, body };
}

test("GET / returns welcome message", async () => {
  const { response, body } = await request("/");

  expect(response.status).toBe(200);
  expect(body).toBe("Welcome to URL SHORTNER API");
});

test("POST /new-url creates a shortened URL", async () => {
  const { response, body } = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({
      url: "https://example.com/test",
    }),
  });

  expect(response.status).toBe(200);
  expect(body).toHaveProperty("shortened_key");
  expect(typeof body.shortened_key).toBe("string");
  expect(body.shortened_key.length).toBeGreaterThan(0);
});

test("POST /new-url handles repeated requests", async () => {
  const first = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({
      url: "https://example.com/one",
    }),
  });

  const second = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({
      url: "https://example.com/two",
    }),
  });

  expect(first.response.status).toBe(200);
  expect(second.response.status).toBe(200);
  expect(first.body.shortened_key).toBeTruthy();
  expect(second.body.shortened_key).toBeTruthy();
});

test("POST /new-url accepts common URL formats", async () => {
  const urls = [
    "https://google.com",
    "http://localhost:8080",
    "https://api.example.com/v1/users?id=123",
    "https://example.com:8443/path#fragment",
  ];

  for (const url of urls) {
    const { response, body } = await request("/new-url", {
      method: "POST",
      body: JSON.stringify({ url }),
    });

    expect(response.status).toBe(200);
    expect(body.shortened_key).toBeTruthy();
  }
});

test("POST /new-url returns correct JSON structure", async () => {
  const { response, body } = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({
      url: "https://github.com",
    }),
  });

  expect(response.status).toBe(200);
  expect(body).toEqual({
    shortened_key: expect.any(String),
  });
});

test("POST /new-url handles very long URLs", async () => {
  const longUrl = `https://example.com/${"a".repeat(1000)}`;

  const { response, body } = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({
      url: longUrl,
    }),
  });

  expect(response.status).toBe(200);
  expect(body.shortened_key).toBeTruthy();
});

// Admin API tests - new in sync with src/admin/api.rs
// The server's default admin_verification_code is "1234" in src/main.rs

test("GET /admin/get_entries requires admin verification", async () => {
  const { response } = await request("/admin/get_entries", {
    method: "GET",
    query: {
      admin_verification_code: "wrong-code",
      search: "",
      count: 10,
    },
  });

  expect(response.status).toBe(401);
});

test("GET /admin/get_entries returns stored entries with correct structure", async () => {
  // Create two entries to ensure there is data to fetch
  const a = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({ url: "https://example.com/alpha" }),
  });
  const b = await request("/new-url", {
    method: "POST",
    body: JSON.stringify({ url: "https://example.com/beta" }),
  });

  expect(a.response.status).toBe(200);
  expect(b.response.status).toBe(200);
  expect(a.body.shortened_key).toBeTruthy();
  expect(b.body.shortened_key).toBeTruthy();

  const { response, body } = await request("/admin/get_entries", {
    method: "GET",
    query: {
      admin_verification_code: "1234",
      search: "",
      count: 10,
    },
  });

  expect(response.status).toBe(200);
  // Response is Vec<(String, String)> serialized as array of 2-item arrays
  expect(Array.isArray(body)).toBe(true);
  expect(body.length).toBeGreaterThan(0);

  for (const entry of body) {
    expect(Array.isArray(entry)).toBe(true);
    expect(entry.length).toBe(2);
    expect(typeof entry[0]).toBe("string");
    expect(typeof entry[1]).toBe("string");
  }

  // At least one of the returned entries should match one of the created keys
  const keys = body.map((e) => e[0]);
  expect(keys).toEqual(expect.arrayContaining([a.body.shortened_key]));
});

test("GET unknown route returns 404", async () => {
  const { response } = await request("/does-not-exist");

  expect(response.status).toBe(404);
});

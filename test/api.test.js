import { expect, test } from "bun:test";

const BASE_URL = process.env.BASE_URL ?? "http://127.0.0.1:3000";

async function request(path, options = {}) {
  const response = await fetch(new URL(path, BASE_URL), {
    ...options,
    headers: {
      accept: "application/json",
      ...(options.headers ?? {}),
    },
  });

  const contentType = response.headers.get("content-type") ?? "";
  const body = contentType.includes("application/json")
    ? await response.json()
    : await response.text();

  return { response, body };
}

test("GET / returns the welcome message", async () => {
  const { response, body } = await request("/");

  expect(response.status).toBe(200);
  expect(body).toBe("Welcome to URL SHORTNER API");
});

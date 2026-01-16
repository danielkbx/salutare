# Salutare

**Salutare** is a small public service that provides a daily greeting in a different language.

Each day, a single greeting is selected and returned in a predictable and reliable way.
The service is intentionally simple, lightweight, and easy to integrate into websites,
scripts, and applications.

Salutare is publicly available at:

**https://salutare.danielkbx.com**

---

## Overview

Salutare exposes a minimal HTTP API that returns:

- one greeting per day
- selected deterministically
- consistent for all users worldwide
- without repeating greetings until all available ones have been used

The service is read-only and does not require authentication.

---

## API Documentation

### Base URL

```
https://salutare.danielkbx.com/api/v1
```

All endpoints described below are versioned under `/api/v1`.

---

### `GET /greeting`

Returns the greeting of the day as a JSON document.

This endpoint is designed to be:
- deterministic
- globally consistent
- safe to call from browsers and non-browser clients

#### Query Parameters

| Name     | Type    | Required | Description |
|----------|---------|----------|-------------|
| `offset` | integer | no       | Shifts the deterministic selection by a number of days. Allows consumers to obtain a different, stable sequence of greetings. |

The `offset` parameter is validated and limited to a safe range.

#### Semantics of `offset`

The offset modifies the internal day number before selecting the greeting.
For a fixed offset value, the sequence of greetings:

- is deterministic
- does not repeat until all greetings have been used once
- is independent from other offsets

This makes it possible for multiple consumers to use the same API without receiving identical daily results.

#### Example Requests

```http
GET /api/v1/greeting
```

```http
GET /api/v1/greeting?offset=17
```

```http
GET /api/v1/greeting?offset=-3
```

#### Successful Response

**Status:** `200 OK`  
**Content-Type:** `application/json`

```json
{
  "date_utc": "2026-01-16",
  "day_number_utc": 20454,
  "offset": 0,
  "id": 42,
  "greeting": "Bonjour.",
  "language": {
    "de": "Französisch",
    "en": "French"
  }
}
```

#### Response Fields

| Field            | Type    | Description |
|------------------|---------|-------------|
| `date_utc`       | string  | UTC date used for selection (`YYYY-MM-DD`). |
| `day_number_utc` | integer | Number of days since the Unix epoch (UTC). |
| `offset`         | integer | Offset applied to the day number. |
| `id`             | integer | Identifier of the selected greeting row. |
| `greeting`       | string  | Greeting text in the selected language. |
| `language.de`    | string  | Name of the language in German. |
| `language.en`    | string  | Name of the language in English. |

---

### `GET /healthz`

Lightweight health and readiness endpoint.

#### Response

**Status:** `200 OK`  
**Content-Type:** `text/plain`

Example:

```
OK – 134 greetings loaded
```

The response confirms that:
- the service is running
- the greetings data was successfully loaded at startup

---

## Day Definition

The concept of “today” is defined strictly as:

- **00:00 UTC → 23:59:59 UTC**

The greeting changes exactly at UTC midnight.
Local server timezones and client timezones are intentionally ignored.

---

## Greeting Selection Guarantees

The service guarantees that:

- greetings do not repeat until all available greetings have been used once
- the same request parameters always produce the same result
- results are stable across restarts

These guarantees apply independently for each distinct `offset` value.

---

## CORS Policy

The API is intentionally public and can be consumed directly from browsers.

- `Access-Control-Allow-Origin: *`
- `GET` and `OPTIONS` methods allowed
- No credentials

Abuse protection and usage limits are enforced at the server and proxy level.

---

## Usage Examples

All examples use the public base URL:

- **Base URL:** `https://salutare.danielkbx.com/api/v1`

### curl

**Greeting of the day**
```bash
curl -s "https://salutare.danielkbx.com/api/v1/greeting"
```

**Greeting with offset**
```bash
curl -s "https://salutare.danielkbx.com/api/v1/greeting?offset=17"
```

**Health check**
```bash
curl -s "https://salutare.danielkbx.com/api/v1/healthz"
```

---

### JavaScript (Browser)

```html
<script>
  async function fetchSalutareGreeting(offset = 0) {
    const url = new URL("https://salutare.danielkbx.com/api/v1/greeting");
    if (offset !== 0) url.searchParams.set("offset", String(offset));

    const res = await fetch(url.toString(), {
      method: "GET",
      headers: { "Accept": "application/json" }
    });

    if (!res.ok) {
      // If the service returns a JSON error (400), try to read it.
      const text = await res.text();
      throw new Error(`Salutare error ${res.status}: ${text}`);
    }

    return await res.json();
  }

  (async () => {
    try {
      const data = await fetchSalutareGreeting(17);
      console.log("Salutare greeting:", data);
      // Example: render it into the page
      document.body.innerText = `${data.greeting} (${data.language.en})`;
    } catch (err) {
      console.error(err);
    }
  })();
</script>
```

---

### JavaScript (Node.js 18+)

Node 18+ ships with `fetch()` built-in.

```js
async function fetchSalutareGreeting(offset = 0) {
  const url = new URL("https://salutare.danielkbx.com/api/v1/greeting");
  if (offset !== 0) url.searchParams.set("offset", String(offset));

  const res = await fetch(url, {
    headers: { "Accept": "application/json" }
  });

  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Salutare error ${res.status}: ${text}`);
  }

  return await res.json();
}

(async () => {
  const data = await fetchSalutareGreeting(17);
  console.log(data);
})();
```

---

### Swift (iOS/macOS)

```swift
import Foundation

struct GreetingResponse: Decodable {
    struct LanguageInfo: Decodable {
        let de: String
        let en: String
    }

    let date_utc: String
    let day_number_utc: Int
    let offset: Int
    let id: Int
    let greeting: String
    let language: LanguageInfo
}

func fetchSalutareGreeting(offset: Int? = nil) async throws -> GreetingResponse {
    var components = URLComponents(string: "https://salutare.danielkbx.com/api/v1/greeting")!
    if let offset = offset, offset != 0 {
        components.queryItems = [URLQueryItem(name: "offset", value: String(offset))]
    }

    var request = URLRequest(url: components.url!)
    request.httpMethod = "GET"
    request.setValue("application/json", forHTTPHeaderField: "Accept")

    let (data, response) = try await URLSession.shared.data(for: request)

    guard let http = response as? HTTPURLResponse else {
        throw URLError(.badServerResponse)
    }
    guard (200...299).contains(http.statusCode) else {
        let body = String(data: data, encoding: .utf8) ?? ""
        throw NSError(domain: "Salutare", code: http.statusCode, userInfo: [
            NSLocalizedDescriptionKey: "HTTP \(http.statusCode): \(body)"
        ])
    }

    return try JSONDecoder().decode(GreetingResponse.self, from: data)
}

// Example usage:
// Task {
//     do {
//         let greeting = try await fetchSalutareGreeting(offset: 17)
//         print("\(greeting.greeting) (\(greeting.language.en))")
//     } catch {
//         print("Error:", error)
//     }
// }
```

---

### Rust (reqwest)

Add dependency:

```toml
# Cargo.toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Example code:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GreetingResponse {
    date_utc: String,
    day_number_utc: i64,
    offset: i64,
    id: u32,
    greeting: String,
    language: LanguageInfo,
}

#[derive(Debug, Deserialize)]
struct LanguageInfo {
    de: String,
    en: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let offset = 17i64;
    let url = format!(
        "https://salutare.danielkbx.com/api/v1/greeting?offset={}",
        offset
    );

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;

    if !status.is_success() {
        return Err(format!("Salutare error {}: {}", status, body).into());
    }

    let parsed: GreetingResponse = serde_json::from_str(&body)?;
    println!("{} ({})", parsed.greeting, parsed.language.en);

    Ok(())
}
```

---

### Go (net/http)

```go
package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"
)

type GreetingResponse struct {
	DateUTC      string `json:"date_utc"`
	DayNumberUTC int64  `json:"day_number_utc"`
	Offset       int64  `json:"offset"`
	ID           int    `json:"id"`
	Greeting     string `json:"greeting"`
	Language     struct {
		DE string `json:"de"`
		EN string `json:"en"`
	} `json:"language"`
}

func fetchSalutareGreeting(offset int64) (*GreetingResponse, error) {
	u, _ := url.Parse("https://salutare.danielkbx.com/api/v1/greeting")
	if offset != 0 {
		q := u.Query()
		q.Set("offset", fmt.Sprintf("%d", offset))
		u.RawQuery = q.Encode()
	}

	client := &http.Client{Timeout: 10 * time.Second}
	req, _ := http.NewRequest(http.MethodGet, u.String(), nil)
	req.Header.Set("Accept", "application/json")

	res, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	body, _ := io.ReadAll(res.Body)
	if res.StatusCode < 200 || res.StatusCode > 299 {
		return nil, fmt.Errorf("Salutare error %d: %s", res.StatusCode, string(body))
	}

	var parsed GreetingResponse
	if err := json.Unmarshal(body, &parsed); err != nil {
		return nil, err
	}
	return &parsed, nil
}

func main() {
	data, err := fetchSalutareGreeting(17)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%s (%s)\n", data.Greeting, data.Language.EN)
}
```

---

## Error Responses

All error responses are returned as JSON and use standard HTTP status codes.
The API does not return HTML error pages.

### Error Format

**Content-Type:** `application/json`

```json
{
  "error": "human-readable error message"
}
```

### `400 Bad Request`

Returned when request parameters are invalid or outside allowed limits.

#### Typical Causes
- `offset` is not a valid integer
- `offset` is outside the allowed range

#### Example: Offset out of range

```http
GET /api/v1/greeting?offset=999999
```

**Response:**
```http
HTTP/1.1 400 Bad Request
Content-Type: application/json
```

```json
{
  "error": "offset out of range (allowed: -100000..100000)"
}
```

### `404 Not Found`

Returned when an unknown endpoint is requested.

```http
GET /api/v1/unknown
```

### `500 Internal Server Error`

Returned only in case of an unexpected server-side failure.

This typically indicates:
- invalid or missing configuration at startup
- internal invariants being violated

In normal operation, these errors should not occur.

---

## Configuration

Salutare is configured via environment variables at startup.

| Variable        | Description                            | Default              |
|-----------------|----------------------------------------|----------------------|
| `CSV_PATH`      | Path to the greetings CSV file         | `greetings.csv`     |
| `SALUTARE_SALT` | Salt used for deterministic selection  | dev-only default     |
| `BIND_ADDR`     | Address and port to bind the service   | `127.0.0.1:8080`    |

The service fails fast if configuration is invalid.

---

## Development

### Run locally

```bash
cargo run
```

### Run tests

```bash
cargo test
```

Integration tests verify the core invariants of the greeting selection logic.

---

## Status

- Core functionality complete
- Public API stable (`/api/v1`)
- Ready for deployment and operation


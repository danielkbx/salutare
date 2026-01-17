# Salutare

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
![Status](https://img.shields.io/badge/status-beta-brightgreen)
![API](https://img.shields.io/badge/API-public-blue)

**Salutare** delivers a daily greeting — always *“Good morning”* — in a different language each day.

The service is intentionally simple and calm: one greeting, one language, one moment per day.
It is designed as a small public utility rather than a product.

The live service is available at  
👉 **https://salutare.danielkbx.com**

The API is publicly available at  
👉 **https://salutare.danielkbx.com/api/v1**

---

## Concept

- One greeting per day
- No repetition until all greetings are used
- UTC-based day change
- Simple, public, and reliable

Salutare can be used directly in a browser, embedded into other projects, or accessed programmatically via its API.

---

## API Overview

### Base URL

```
https://salutare.danielkbx.com
```

---

## Endpoint: Greeting of the Day

### `GET /api/v1/greeting`

Returns the greeting of the current UTC day.

#### Query Parameters

| Name   | Type    | Required | Description |
|-------|---------|----------|-------------|
| offset | integer | no | Optional deterministic offset. Allows callers to retrieve a different greeting without affecting global order. |

Example:

```
/api/v1/greeting?offset=3
```

---

## Response Format

```json
{
  "date_utc": "2026-01-17",
  "day_number_utc": 19745,
  "offset": 0,
  "id": 42,
  "greeting": "Buenos días",
  "language": {
    "de": "Spanisch",
    "en": "Spanish"
  }
}
```

### Fields

| Field | Description |
|-----|-------------|
| `date_utc` | Current date in UTC |
| `day_number_utc` | Sequential day number since Unix epoch (UTC) |
| `offset` | Applied offset (0 if none) |
| `id` | Internal greeting identifier |
| `greeting` | The greeting text |
| `language.de` | Language name in German |
| `language.en` | Language name in English |

---

## Error Responses

Errors are returned as JSON:

```json
{
  "error": "Invalid offset parameter"
}
```

HTTP status codes follow standard semantics (`400`, `429`, `500`, etc.).

---

## Usage Examples

### curl

```bash
curl https://salutare.danielkbx.com/api/v1/greeting
```

With offset:

```bash
curl "https://salutare.danielkbx.com/api/v1/greeting?offset=5"
```

---

### JavaScript (Browser)

```html
<script>
fetch("/api/v1/greeting")
  .then(r => r.json())
  .then(data => {
    console.log(`${data.greeting} (${data.language.en})`);
  });
</script>
```

---

### JavaScript (Node.js)

```js
const fetch = require("node-fetch");

async function run() {
  const res = await fetch("https://salutare.danielkbx.com/api/v1/greeting");
  const data = await res.json();
  console.log(`${data.greeting} (${data.language.en})`);
}

run();
```

---

### Python (Standard Library only)

```python
import json
import urllib.request
from urllib.parse import urlencode

API_URL = "https://salutare.danielkbx.com/api/v1/greeting"

def fetch_greeting(offset=None):
    url = API_URL
    if offset is not None:
        url += "?" + urlencode({"offset": offset})

    with urllib.request.urlopen(url, timeout=5) as response:
        data = json.loads(response.read().decode("utf-8"))

    greeting = data["greeting"]
    language = data["language"]["en"]

    print(f"{greeting} ({language})")

if __name__ == "__main__":
    fetch_greeting()
```

Notes:
- Uses only Python’s standard library
- UTF-8 handling is automatic
- `offset` is optional

---

### Swift

```swift
import Foundation

let url = URL(string: "https://salutare.danielkbx.com/api/v1/greeting")!

URLSession.shared.dataTask(with: url) { data, _, _ in
    guard let data = data else { return }
    let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any]
    if
        let greeting = json?["greeting"] as? String,
        let language = (json?["language"] as? [String: String])?["en"]
    {
        print("\(greeting) (\(language))")
    }
}.resume()
```

---

### Rust

```rust
use reqwest::blocking::get;
use serde_json::Value;

fn main() {
    let resp = get("https://salutare.danielkbx.com/api/v1/greeting")
        .unwrap()
        .text()
        .unwrap();

    let json: Value = serde_json::from_str(&resp).unwrap();

    let greeting = json["greeting"].as_str().unwrap();
    let language = json["language"]["en"].as_str().unwrap();

    println!("{} ({})", greeting, language);
}
```

---

### Go

```go
package main

import (
    "encoding/json"
    "fmt"
    "net/http"
)

func main() {
    resp, _ := http.Get("https://salutare.danielkbx.com/api/v1/greeting")
    defer resp.Body.Close()

    var data map[string]interface{}
    json.NewDecoder(resp.Body).Decode(&data)

    greeting := data["greeting"].(string)
    lang := data["language"].(map[string]interface{})["en"].(string)

    fmt.Printf("%s (%s)\n", greeting, lang)
}
```

---

## Rate Limiting

The API is publicly accessible but rate-limited per IP to ensure fair usage.

If you exceed the limit, the API will respond with `429 Too Many Requests`.

---

## License

Apache License 2.0  
© 2026 Daniel Wetzel (danielkbx)

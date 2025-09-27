# image-to-json

A tiny CLI that converts an image file into a JSON object with base64-encoded data, ready to store in a database.

## What it does

- Reads an image from a file path.
- Detects the MIME type from the filename (e.g., `image/png`).
- Base64-encodes the image bytes.
- Outputs a pretty-printed JSON object with:
  - `filename`
  - `mime_type`
  - `size_bytes`
  - `encoding` (always `base64`)
  - `data` (either raw base64 or a Data URL)

## The problem it solves

Many databases and APIs prefer JSON and do not accept raw binary. This tool turns image bytes into a JSON document so you can store and move images safely through JSON-based systems (e.g., document stores, message queues, REST APIs) without dealing with multipart uploads or separate file hosting.

## Why it's useful

- Store small images/avatars directly in JSON (e.g. Couchbase).
- Send images over JSON APIs without multipart.
- Self-contained records: image + metadata in one object.
- Data URL option lets front-ends render immediately via `<img src="...">`.


## Example output

```json
{
  "mime_type": "image/png",
  "size_bytes": 47351,
  "encoding": "base64",
  "data": "data:image/png;base64,iVBORw0KGgoAAA..."
}
```

# Language Model Logging API

This is a quick example of a REST API written in Rust with Axum and SQLx.

## Running the API

To run the API, make sure you have Rust and Cargo installed. Then, clone the repository and run:

```bash
cargo run
```

The API will be available at `http://localhost:5000` by default.

## Example GET /swagger

This endpoint displays the API's documentation.

## POST /api/v1/lm/log

This endpoint allows you to post a new log record.

```python
import requests

url = "http://localhost:5000/api/v1/lm/log"

response = requests.post(url, json={
    "model_provider": "OpenAI",
    "model_name": "gpt-4o-mini",
    "model_version": "2024-12-31",
    "app_name": "majorly important business application",
    "app_project": "LLM Logger",
    "app_version": "1.0",
    "prompt": [
        {
            "role": "system",
            "content": "You are a helpful assistant."
        },
        {
            "role": "user",
            "content": "Lorem ipsum..."
        }
    ],
    "response": "dolor sin amet",
    "prompt_user_id": "xx12345",
    "prompt_app_hostname": "lpae365a",
    "prompt_submit_ts": "2024-12-12T00:00:00.0",
    "response_receipt_ts": "2024-12-13T00:00:05.0",
    "input_tokens": 0,
    "output_tokens": 0,
    "total_tokens": 0
})

print(response.status_code)
print(response.json())
```

It also allows you to get a individual log record by its ID.

```python
import requests

url = "http://localhost:5000/api/v1/lm/log/1"

response = requests.get(url)

print(response.status_code)
print(response.json())
```

You can also put updates to an existing log record by its ID.

```python
import requests

url = "http://localhost:5000/api/v1/lm/log/1"

response = requests.put(url, json={
    "model_provider": "OpenAI",
    "model_name": "gpt-4o-mini",
    "model_version": "2024-12-31",
    "app_name": "majorly important business application",
    "app_project": "LLM Logger",
    "app_version": "1.0",
    "prompt": [
        {
            "role": "system",
            "content": "You are a helpful assistant."
        },
        {
            "role": "user",
            "content": "Lorem ipsum..."
        }
    ],
    "response": "dolor sin amet",
    "prompt_user_id": "xx12345",
    "prompt_app_hostname": "lpae365a",
    "prompt_submit_ts": "2024-12-12T00:00:00.0",
    "response_receipt_ts": "2024-12-13T00:00:05.0",
    "input_tokens": 0,
    "output_tokens": 0,
    "total_tokens": 0
})

print(response.status_code)
print(response.json())
```

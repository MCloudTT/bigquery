# BigQuery adapter for MCloudTT
This is a service to log messages from [MCloudTT](https://github.com/MCloudTT/mcloudtt) to BigQuery. It works by subscribing to the `sync` redis pub/sub channel, where all brokers publish incoming messages.

## Where this fits in
![cluster_overview_bq_adapter_dark](https://user-images.githubusercontent.com/60036186/222136295-20aabeb8-1725-41d6-baa2-5e101dd5ad6c.png)

## How to use
1. Create a service account with role `bigquery.dataEditor`
2. Create a key for that service account and put it in the root folder of the project
3. Adjust the config.toml file
4. Build the binary using `cargo build --release --target x86_64-unknown-linux-musl`; make sure to have the `x86_64-unknown-linux-musl` toolchain installed
5. Build the docker container using `docker build -t mcloudtt-bq . --no-cache`

⚠️NOTICE: The image contains the service-account-key. It is also possible to use the `gcp-bigquery-client` by utilizing a policy-binding between the GKE-Cluster and the service account. In future versions it is planned to pass the config.toml and sa.key as arguments to the container.

## BigQuery setup
The service expects the schema of the table to be as follows:
```json
"schema": {
    "fields": [
      {
        "maxLength": "64",
        "mode": "REQUIRED",
        "name": "topic",
        "type": "STRING"
      },
      {
        "maxLength": "256",
        "mode": "REQUIRED",
        "name": "message",
        "type": "STRING"
      },
      {
        "mode": "REQUIRED",
        "name": "datetime",
        "type": "DATETIME"
      }
    ]
  }
```

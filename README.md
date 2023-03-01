# BigQuery adapter for MCloudTT
This is a service to log messages from [MCloudTT](https://github.com/MCloudTT/mcloudtt) to BigQuery. It works by subscribing to the `sync` redis pub/sub channel, where all brokers publish incoming messages.

## Where this fits in
![cluster_overview_bq_adapter_dark](https://user-images.githubusercontent.com/60036186/222136295-20aabeb8-1725-41d6-baa2-5e101dd5ad6c.png)

## How to use
1. Create a service account with role `bigquery.dataEditor`
2. Create a key for that service account and put it in the root folder of the project
3. Adjust the config.toml file

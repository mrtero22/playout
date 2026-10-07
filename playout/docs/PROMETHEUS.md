# What is Prometheus?

Prometheus is an open-source systems monitoring and alerting toolkit. It was originally built at SoundCloud in 2012 and is now a graduated project of the Cloud Native Computing Foundation (CNCF). It has become the de facto standard for monitoring in the cloud-native ecosystem, especially with Kubernetes.

Prometheus collects and stores metrics as time series data. Each metric is recorded with a timestamp and optional key-value pairs called labels. This design supports high-dimensionality data and flexible querying.

## Key Features

- **Multi-dimensional data model** - Time series are identified by a metric name plus a set of key/value labels.
- **PromQL** - A powerful and flexible query language for selecting and aggregating time series data.
- **Pull-based collection** - Prometheus scrapes metrics from targets over HTTP at configured intervals. Pushing is supported via an intermediary gateway (useful for short-lived jobs).
- **Autonomous single-server design** - No reliance on distributed storage. Individual Prometheus servers run independently.
- **Service discovery** - Targets can be discovered dynamically (Kubernetes, Consul, etc.) or configured statically.
- **Alerting** - Rules can be defined to trigger alerts, which are handled by the Alertmanager.
- **Visualization** - Built-in expression browser plus excellent integration with tools like Grafana.

## Core Components

| Component              | Purpose                                                                 |
|------------------------|-------------------------------------------------------------------------|
| Prometheus Server      | Scrapes, stores, and queries metrics. Evaluates alerting rules.        |
| Exporters              | Expose metrics from systems that do not natively support Prometheus.   |
| Client Libraries       | Instrument applications to expose custom metrics.                      |
| Pushgateway            | Allows short-lived jobs to push metrics.                               |
| Alertmanager           | Handles alerts - grouping, routing, silencing, and notification.       |

## Metric Types

Prometheus supports four main metric types:

- **Counter** - A cumulative value that only increases (e.g., total requests served).
- **Gauge** - A value that can go up or down (e.g., current memory usage).
- **Histogram** - Samples observations and counts them in configurable buckets (useful for request durations).
- **Summary** - Similar to histograms but calculates quantiles on the client side.

## How Prometheus Works

1. Targets (applications or exporters) expose metrics on an HTTP endpoint, usually `/metrics`.
2. The Prometheus server periodically scrapes these endpoints.
3. Metrics are stored locally in a time-series database.
4. Users query the data with PromQL.
5. Alerting rules evaluate expressions and send alerts to Alertmanager when conditions are met.

## Common Use Cases

- Monitoring Kubernetes clusters and microservices.
- Tracking application performance (latency, error rates, throughput).
- Infrastructure monitoring (CPU, memory, disk, network).
- Alerting on service-level objectives (SLOs) and error budgets.

## Ecosystem and Integrations

Prometheus works well with:

- **Grafana** - The most popular tool for building dashboards.
- **Thanos or Cortex** - For long-term storage and high availability.
- **Kubernetes** - Native service discovery and first-class support.
- Hundreds of exporters for databases, message queues, hardware, and more.

## Summary

Prometheus is a reliable, scalable, and widely adopted open-source monitoring system. Its pull model, dimensional data model, and PromQL make it especially well-suited for dynamic, cloud-native environments. It focuses on metrics (one of the three pillars of observability) and pairs naturally with logging and tracing tools for complete observability.

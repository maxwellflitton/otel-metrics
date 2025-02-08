# otel-metrics
basic repo working out the otel setup for Rust and prometheus


## Sources

The main setup for open telemetry is found below:

https://github.com/open-telemetry/opentelemetry-collector-contrib/blob/main/examples/demo/README.md


## Prometheus

To see everything exported by the otel collector run the following query:

```
{job="otel-collector"}
```

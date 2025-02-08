curl -X POST http://localhost:4318/v1/metrics \
  -H "Content-Type: application/json" \
  -d '{
    "resourceMetrics": [
      {
        "scopeMetrics": [
          {
            "metrics": [
              {
                "name": "basic_test_metric",
                "gauge": {
                  "dataPoints": [
                    {
                      "asDouble": 2.0
                    }
                  ]
                }
              }
            ]
          }
        ]
      }
    ]
  }'

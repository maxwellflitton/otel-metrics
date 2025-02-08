curl -X POST http://localhost:4318/v1/metrics \
     -H "Content-Type: application/json" \
     -d '{
  "resourceMetrics": [
    {
      "resource": {
        "attributes": [
          { "key": "service.name", "value": { "stringValue": "my_rust_service" } }
        ]
      },
      "scopeMetrics": [
        {
          "metrics": [
            {
              "name": "test_counter",
              "sum": {
                "dataPoints": [
                  {
                    "asInt": 1,
                    "attributes": []
                  }
                ],
                "aggregationTemporality": 2,
                "isMonotonic": true
              }
            }
          ]
        }
      ]
    }
  ]
}'

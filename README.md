# rust-micro-todo

Implementation of a todo application backend using rust, service-oriented architecture and API gateway pattern.

# Project structure

### proto

Contains protobuf service and messages definitions.

### todo

Contains implementation of todo server according to proto file.

Uses tonic, slog-rs logging, libxid for id generation.

There are two implementations of todos repository: one based on std::collections::HashMap and one based on postgres using sqlx.

### api

Implementation of http server which uses todo client.

Uses warp as a http framework with slog-rs logging.

### helm_chart

Contains helm chart to deploy application in kubernetes.
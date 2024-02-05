# host-analyzer

This host-analyzer operates as an independant binary that can communicate with K8sGPT over the [defined](https://github.com/k8sgpt-ai/schemas.git) protobuf gRPC API.
This is an example of how to build external, use-case language specific plugins that can be used with K8sGPT.

<img src=./images/diagram.png width="400px;" />

## Usage

- Operate as a binary or `cargo run`
- Set configuration options through K8sGPT ( _not covered here_)

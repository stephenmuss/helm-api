# helm-api

[![Build Status](https://travis-ci.org/stephenmuss/helm-api.svg?branch=master)](https://travis-ci.org/stephenmuss/helm-api)
[![docs](https://docs.rs/helm-api/badge.svg)](https://stephenmuss.github.io/helm-api/doc/helm_api/index.html)

helm-api is a Helm client library for Rust.

Rust modules are automatically generated from the `.proto` files in the [Helm project](https://github.com/helm/helm/).

## Examples

There are some basic usage examples in the examples directory. These examples will only work if you have configured tiller in specific ways.

### Simple

To run the simple example you will need to have tiller running at localhost:44134. You can either run tiller locally or use `kubectl port-forward` to accomplish the same thing.

The simple example will not work with a Tiller installtion using TLS.

To run use

```sh
cargo run --example simple
```

## License

This project is licensed under Apache License, Version 2.0, ([LICENSE](https://github.com/stephenmuss/helm-api/blob/master/LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

Helm is licensed under Apache License, Version 2.0, ([LICENSE](https://github.com/helm/helm/blob/master/LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

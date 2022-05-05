# GraphqlParser

## [![Hex pm](http://img.shields.io/hexpm/v/graphql_parser.svg?style=flat)](https://hex.pm/packages/graphql_parser) [![Hex Docs](https://img.shields.io/badge/hex-docs-9768d1.svg)](https://hexdocs.pm/graphql_parser) [![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)![.github/workflows/elixir.yml](https://github.com/maartenvanvliet/graphql_parser/workflows/.github/workflows/elixir.yml/badge.svg)

<!-- MDOC !-->

GraphQL parser implemented as a nif using Rustler. Wraps `graphql_parser` Rust package.
Converts GraphQL documents to an Absinthe.Language representation.

It's fully functional but also an experiment to try out Rust and Rustler.

## Installation

```elixir
def deps do
  [
    {:graphql_parser, "~> 0.1.0"}
  ]
end
```

## Benchmark

`graphql_parser` is a bit faster than the default Absinthe GraphQL parser. Although,
when the input documents become bigger the speed difference increases.

In general the parsing overhead for executing a GraphQL document is not that
big compared to all other phases the document goes through, e.g. validation,
resolving and encoding. Using this package is thus rarely necessary.

The benchmark can be run with `mix run bench/test.ex`

```
Operating System: macOS
CPU Information: Intel(R) Core(TM) i9-9880H CPU @ 2.30GHz
Number of Available Cores: 16
Available memory: 16 GB
Elixir 1.13.1
Erlang 24.0.3

Benchmark suite executing with the following configuration:
warmup: 1 s
time: 5 s
memory time: 300 ms
reduction time: 0 ns
parallel: 1
inputs: Basic, Introspection, Kitchensink
Estimated total run time: 37.80 s

Benchmarking absinthe with input Basic ...
Benchmarking absinthe with input Introspection ...
Benchmarking absinthe with input Kitchensink ...
Benchmarking rust with input Basic ...
Benchmarking rust with input Introspection ...
Benchmarking rust with input Kitchensink ...

##### With input Basic #####
Name               ips        average  deviation         median         99th %
rust          118.96 K        8.41 μs   ±106.22%           7 μs          37 μs
absinthe       96.03 K       10.41 μs    ±65.88%          10 μs          36 μs

Comparison:
rust          118.96 K
absinthe       96.03 K - 1.24x slower +2.01 μs

Memory usage statistics:

Name        Memory usage
rust             1.52 KB
absinthe        19.73 KB - 13.02x memory usage +18.22 KB

**All measurements for memory usage were the same**

##### With input Introspection #####
Name               ips        average  deviation         median         99th %
rust            6.50 K      153.88 μs    ±22.54%         147 μs      313.76 μs
absinthe        1.92 K      521.35 μs    ±14.94%         491 μs         851 μs

Comparison:
rust            6.50 K
absinthe        1.92 K - 3.39x slower +367.47 μs

Memory usage statistics:

Name        Memory usage
rust             1.49 KB
absinthe       987.80 KB - 661.98x memory usage +986.31 KB

**All measurements for memory usage were the same**

##### With input Kitchensink #####
Name               ips        average  deviation         median         99th %
rust            7.58 K      131.95 μs    ±22.98%         127 μs         268 μs
absinthe        1.87 K      536.13 μs    ±17.89%         496 μs      889.80 μs

Comparison:
rust            7.58 K
absinthe        1.87 K - 4.06x slower +404.19 μs

Memory usage statistics:

Name        Memory usage
rust          0.00145 MB
absinthe         1.01 MB - 694.82x memory usage +1.01 MB

**All measurements for memory usage were the same**
```

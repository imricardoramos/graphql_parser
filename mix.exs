defmodule GraphqlParser.MixProject do
  use Mix.Project

  @url "https://github.com/maartenvanvliet/graphql_parser"

  def project do
    [
      app: :graphql_parser,
      version: "0.1.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      source_url: @url,
      homepage_url: @url,
      name: "GraphQLParser",
      description:
        "GraphQL parser implemented as a nif in Rust using Rustler. Converts GraphQL documents
to an Absinthe.Language representation.",
      package: package(),
      docs: [
        main: "Quarto",
        canonical: "http://hexdocs.pm/quarto",
        source_url: @url
      ]
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      maintainers: ["Maarten van Vliet"],
      links: %{"GitHub" => @url},
      files: ~w(
        LICENSE
        README.md
        config
        lib
        mix.exs
        native/graphqlparser_native/src
        native/graphqlparser_native/Cargo.toml
        native/graphqlparser_native/Cargo.lock
      )
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:absinthe, "~> 1.7"},
      {:rustler, "~> 0.25"},
      {:benchee, "~> 1.1", optional: true},
      {:ex_doc, ">= 0.0.0", only: [:dev, :test]}
    ]
  end
end

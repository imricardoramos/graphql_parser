Benchee.run(
  %{
    "absinthe" => fn input ->
      {:ok, _} = GraphQLParser.AbsintheParser.parse(input)
    end,
    "rust" => fn input ->
      {:ok, _} = GraphQLParser.parse_query(input)

    end
  },
  warmup: 1,
  time: 5,
  memory_time: 0.3,
  inputs: %{
    "Introspection" => File.read!("test/introspection.graphql"),
    "Kitchensink" => File.read!("test/kitchensink.graphql"),
    "Basic" => "{ id name }",
  }
)

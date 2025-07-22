defmodule GraphQLParserTest do
  use ExUnit.Case

  @document """
  query me($a: String = "deftest", $b: Some!, $c: Int! = 42, $d: [String] = ["a", "b", "c"], $e: [Float!]!) @aa {
    id @deprecated(reason: "something")
    name(str: "lkj", int: 1, float: 1.1, bool: TRUE, var: $var, list: [1, 2, 3], object: { a: 1, b: "str" })
  }

  mutation {
    createUser(accountId: 1, name: "bob") {
      id
      account {
        name
      }
    }
  }

  subscription {
    a {
      test
    }
  }
  """
  test "basic query" do
    {:ok, doc} = GraphQLParser.parse_query(@document)

    assert @document == render(doc)
  end

  @document """
  {
    id
    name
  }
  """
  test "shorthand" do
    {:ok, doc} = GraphQLParser.parse_query(@document)

    assert @document == render(doc)
  end

  @document """
  fragment FooFields on Foo {
    foo
    bar
  }
  """
  test "fragment" do
    {:ok, doc} = GraphQLParser.parse_query(@document)

    assert @document == render(doc)
  end

  @document """
  query Foo {
    ... on A {
      __typename
      a
    }
    ... {
      b
    }
    bar
  }
  """
  test "fragment spread" do
    {:ok, doc} = GraphQLParser.parse_query(@document)

    assert @document == render(doc)
  end

  test "kitchensink" do
    graphql = File.read!("test/kitchensink.graphql")
    {:ok, doc} = GraphQLParser.parse_query(graphql)

    assert graphql == render(doc)
  end

  test "parse error" do
    query = "{ id name"

    {:error, error} = GraphQLParser.parse_query(query)

    assert %GraphQLParser.Error{
             locations: [%{column: 10, line: 1}],
             message: "Parse error\nUnexpected end of input\nExpected }\n"
           } == error
  end

  def render(doc) do
    Absinthe.Language.Render.inspect(doc, %{pretty: true})
  end
end

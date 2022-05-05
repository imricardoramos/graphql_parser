defmodule GraphQLParser do
  @external_resource "./README.md"
  @moduledoc """
  #{File.read!(@external_resource) |> String.split("<!-- MDOC !-->", parts: 2) |> List.last()}
  """

  @doc """
  Parse a GraphQL query.

  ## Examples

      {:ok, %Absinthe.Language.Document{}} = GraphQLParser.parse("{ id name }")

      {:error, error} = GraphQLParser.parse("{ id name ")

  """
  def parse_query(doc) do
    case GraphQLParser.Native.parse_query(doc) do
      {:ok, doc} -> {:ok, doc}
      {:error, error} -> {:error, format_raw_parse_error(error)}
    end
  end

  defp format_raw_parse_error("query parse error: Parse error at " <> message) do
    [pos, message] = String.split(message, "\n", trim: true, parts: 2)
    [line, column] = String.split(pos, ":", parts: 2) |> Enum.map(&String.to_integer/1)

    %GraphQLParser.Error{
      message: "Parse error\n" <> message,
      locations: [%{line: line, column: column}]
    }
  end

  defp format_raw_parse_error(error) do
    %GraphQLParser.Error{
      message: error,
      locations: []
    }
  end
end

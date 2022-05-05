defmodule GraphQLParser.AbsintheParser do
  @moduledoc """
  Extracted GrapqhQL parser from Absinthe.

  """
  alias Absinthe.Language
  alias Absinthe.Phase

  def parse(input) when is_binary(input) do
    parse(%Language.Source{body: input})
  end

  def parse(input) do
    try do
      case tokenize(input.body) do
        {:ok, []} ->
          {:ok, %Language.Document{}}

        {:ok, tokens} ->
          case :absinthe_parser.parse(tokens) do
            {:ok, _doc} = result ->
              result

            {:error, raw_error} ->
              {:error, format_raw_parse_error(raw_error)}
          end

        other ->
          other
      end
    rescue
      error ->
        {:error, format_raw_parse_error(error)}
    end
  end

  defp tokenize(input) do
    case Absinthe.Lexer.tokenize(input) do
      {:error, rest, loc} ->
        {:error, format_raw_parse_error({:lexer, rest, loc})}

      other ->
        other
    end
  end

  @spec format_raw_parse_error({{integer, integer}, :absinthe_parser, [charlist]}) ::
          Phase.Error.t()
  defp format_raw_parse_error({{line, column}, :absinthe_parser, msgs}) do
    message = msgs |> Enum.map(&to_string/1) |> Enum.join("")
    %Phase.Error{message: message, locations: [%{line: line, column: column}], phase: __MODULE__}
  end

  @spec format_raw_parse_error({integer, :absinthe_parser, [charlist]}) ::
          Phase.Error.t()
  defp format_raw_parse_error({line, :absinthe_parser, msgs}) do
    message = msgs |> Enum.map(&to_string/1) |> Enum.join("")
    %Phase.Error{message: message, locations: [%{line: line, column: 0}], phase: __MODULE__}
  end

  @spec format_raw_parse_error({:lexer, String.t(), {line :: pos_integer, column :: pos_integer}}) ::
          Phase.Error.t()
  defp format_raw_parse_error({:lexer, rest, {line, column}}) do
    sample_slice = String.slice(rest, 0, 10)
    sample = if String.valid?(sample_slice), do: sample_slice, else: inspect(sample_slice)

    message = "Parsing failed at `#{sample}`"
    %Phase.Error{message: message, locations: [%{line: line, column: column}], phase: __MODULE__}
  end

  @unknown_error_msg "An unknown error occurred during parsing"
  @spec format_raw_parse_error(map) :: Phase.Error.t()
  defp format_raw_parse_error(%{} = error) do
    detail =
      if Exception.exception?(error) do
        ": " <> Exception.message(error)
      else
        ""
      end

    %Phase.Error{message: @unknown_error_msg <> detail, phase: __MODULE__}
  end
end

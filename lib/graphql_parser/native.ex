defmodule GraphQLParser.Native do
  @moduledoc false
  use Rustler,
    otp_app: :graphql_parser,
    crate: :graphqlparser_native

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def parse_query(_arg1), do: :erlang.nif_error(:nif_not_loaded)
end

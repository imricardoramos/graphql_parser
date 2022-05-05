defmodule GraphQLParser.Error do
  @moduledoc false

  @enforce_keys [:message]
  defstruct [
    :message,
    locations: []
  ]

  @type loc_t :: %{optional(any) => any, line: pos_integer, column: pos_integer}

  @type t :: %__MODULE__{
          message: String.t(),
          locations: [loc_t]
        }
end

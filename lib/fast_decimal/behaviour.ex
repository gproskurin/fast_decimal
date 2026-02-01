defmodule FastDecimal.Behaviour do

@opaque t :: binary()

@callback new(String.t()) :: t()
@callback new(integer(), integer(), integer()) :: t()
@callback mult(term(), term()) :: term()
@callback div(term(), term()) :: term()
@callback equal?(term(), term()) :: boolean()
@callback gt?(term(), term()) :: boolean()
@callback to_string(term()) :: String.t()

end


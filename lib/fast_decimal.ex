defmodule FastDecimal do

@impl_mod Application.compile_env!(:fast_decimal, :decimal_impl)

defdelegate new(str), to: @impl_mod
defdelegate new(sign, mantissa, scale), to: @impl_mod
defdelegate mult(d1, d2), to: @impl_mod
defdelegate div(d1, d2), to: @impl_mod
defdelegate equal?(d1, d2), to: @impl_mod
defdelegate gt?(d1, d2), to: @impl_mod
defdelegate to_string(d), to: @impl_mod

end


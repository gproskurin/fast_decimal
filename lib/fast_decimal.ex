defmodule FastDecimal.Behaviour do
@opaque type t
@callback new(String.t()) :: t()
@callback new(integer(), integer(), integer()) :: t()
@callback mult(term(), term()) :: term()
@callback div(term(), term()) :: term()
@callback equal?(term(), term()) :: boolean()
@callback gt?(term(), term()) :: boolean()
@callback to_string(term()) :: String.t()
end


defmodule FastDecimal do
@impl_mod Application.compile_env(FastDecimal, :decimal_impl)
defdelegate new(str), to: @impl_mod
defdelegate new(sign, mantissa, scale), to: @impl_mod
defdelegate mult(d1, d2), to: @impl_mod
defdelegate div(d1, d2), to: @impl_mod
defdelegate equal?(d1, d2), to: @impl_mod
defdelegate gt?(d1, d2), to: @impl_mod
defdelegate to_string(d), to: @impl_mod
end


defmodule FastDecimal.Impl.RustDecimal do

@behaviour FastDecimal.Behaviour
use Rustler, otp_app: FastDecimal, crate: "fastdecimal_impl_rustdecimal"

@impl FastDecimal.Behaviour
def new(_str), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def new(_sign, _mantissa, _scale), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def mult(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def div(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def equal?(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def gt?(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def to_string(_dec), do: :erlang.nif_error(:nif_not_loaded)

end


defmodule FastDecimal.Impl.Fastnum do

@behaviour FastDecimal.Behaviour
use Rustler, otp_app: FastDecimal, crate: "fastdecimal_impl_fastnum"

@impl FastDecimal.Behaviour
def new(_str), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def new(_sign, _mantissa, _scale), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def mult(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def div(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def equal?(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def gt?(_dec1, _dec2), do: :erlang.nif_error(:nif_not_loaded)
@impl FastDecimal.Behaviour
def to_string(_dec), do: :erlang.nif_error(:nif_not_loaded)

end


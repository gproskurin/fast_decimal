defmodule FastDecimal.Impl.RustDecimal do

@behaviour FastDecimal.Behaviour
use Rustler, otp_app: :fast_decimal, crate: "fastdecimal_impl_rustdecimal"

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
use Rustler, otp_app: :fast_decimal, crate: "fastdecimal_impl_fastnum"

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


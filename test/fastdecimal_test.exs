defmodule FastDecimalTest do
use ExUnit.Case

@mods [Decimal, FastDecimal.Impl.RustDecimal, FastDecimal.Impl.Fastnum, FastDecimal]


test "decimal multiplication" do
    do_op(:mult, ["123.456", "12.34"], "1523.44704")
    do_op(:mult, ["0.000000000003", "987654.3210987"], "0.0000029629629632961")
end


test "decimal division" do
    do_op(:div, ["123.456", "0.4"], "308.64")
    do_op(:div, ["0.0001234567", "0.00008000"], "1.54320875")
end


test "new from string" do
    do_op_new = fn str, exp_strs ->
        Enum.each(
            @mods,
            fn mod ->
                dec = :erlang.apply(mod, :new, [str])
                # string representation should match at least one of expected strings
                dec_str = :erlang.apply(mod, :to_string, [dec])
                assert Enum.member?(exp_strs, dec_str)
            end
        )
    end
    do_op_new.("123.456", ["123.456"])
    do_op_new.("-0.00123456789", ["-0.00123456789"])
    do_op_new.("0", ["0"])
end


test "new raise" do
    ex_map = %{
        Decimal => Decimal.Error,
        FastDecimal.Impl.Fastnum => ArgumentError,
        FastDecimal.Impl.RustDecimal => ArgumentError,
        FastDecimal => ArgumentError
    }
    do_op_new = fn str ->
        Enum.each(
            @mods,
            fn mod ->
                exp_ex = Map.fetch!(ex_map, mod)
                assert_raise exp_ex, fn -> :erlang.apply(mod, :new, [str]) end
            end
        )
    end
    do_op_new.("")
    do_op_new.("qwe")
    do_op_new.("123qwe")
    do_op_new.("123 1")
end


test "decimal from mantissa & scale" do
    do_op_new = fn(mantissa, scale, exp_strs) ->
        Enum.each(
            @mods,
            fn mod ->
                dec = :erlang.apply(mod, :new, [1, mantissa, scale])
                # string representation should match at least one of expected strings
                dec_str = :erlang.apply(mod, :to_string, [dec])
                assert Enum.member?(exp_strs, dec_str)
            end
        )
    end
    do_op_new.(12345, 0, ["12345", "1.2345E+4"])
    do_op_new.(23456, 3, ["23456000", "2.3456E+7"])
    do_op_new.(34567, -3, ["34.567"])
    do_op_new.(0, 0, ["0", "0E+3", "0000"])
    do_op_new.(0, -3, ["0.000"])
    do_op_new.(0, 3, ["0", "0E+3", "0000"])
end


defp do_op(func_op, args_str, exp_str) do
    Enum.each(
        @mods,
        fn mod ->
            fn_new = fn x -> :erlang.apply(mod, :new, [x]) end
            args_dec = Enum.map(args_str, fn_new)
            exp_dec = fn_new.(exp_str)
            res_dec = :erlang.apply(mod, func_op, args_dec)
            assert :erlang.apply(mod, :equal?, [res_dec, exp_dec])
        end
    )
end


end


dec_e_1 = Decimal.new("123.456")
dec_e_2 = Decimal.new("12.34")

dec_rd_1 = Trdex.Decimal.RustDecimal.new("123.456")
dec_rd_2 = Trdex.Decimal.RustDecimal.new("12.34")

dec_fn_1 = Trdex.Decimal.FastNum.new("123.456")
dec_fn_2 = Trdex.Decimal.FastNum.new("12.34")

Benchee.run(%{
    "Decimal mult elixir/decimal" => fn -> Decimal.mult(dec_e_1, dec_e_2) end,
    "Decimal mult rust/rust_decimal" => fn -> Trdex.Decimal.RustDecimal.mult(dec_rd_1, dec_rd_2) end,
    "Decimal mult rust/fastnum" => fn -> Trdex.Decimal.FastNum.mult(dec_fn_1, dec_fn_2) end,
})

Benchee.run(%{
    "Decimal div elixir/decimal" => fn -> Decimal.div(dec_e_1, dec_e_2) end,
    "Decimal div rust/rust_decimal" => fn -> Trdex.Decimal.RustDecimal.div(dec_rd_1, dec_rd_2) end,
    "Decimal div rust/fastnum" => fn -> Trdex.Decimal.FastNum.div(dec_fn_1, dec_fn_2) end,
})

Benchee.run(%{
    "Decimal from mantissa/coef elixir/decimal" => fn -> Decimal.new(1, 123456, 5) end,
    "Decimal from mantissa/coef rust/rust_decimal" => fn -> Trdex.Decimal.RustDecimal.new(1, 123456, 5) end,
    "Decimal from mantissa/coef rust/fastnum" => fn -> Trdex.Decimal.FastNum.new(1, 123456, 5) end,
})

Benchee.run(%{
    "Decimal combo elixir/decimal" => fn -> Decimal.new(1, 123456, 5) |> Decimal.mult(dec_e_1) |> Decimal.div(dec_e_2) end,
    "Decimal combo rust/rust_decimal" => fn -> Trdex.Decimal.RustDecimal.new(1, 123456, 5) |> Trdex.Decimal.RustDecimal.mult(dec_rd_1) |> Trdex.Decimal.RustDecimal.div(dec_rd_2) end,
    "Decimal combo rust/fastnum" => fn -> Trdex.Decimal.FastNum.new(1, 123456, 5) |> Trdex.Decimal.FastNum.mult(dec_fn_1) |> Trdex.Decimal.FastNum.div(dec_fn_2) end,
})


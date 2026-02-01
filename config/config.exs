Config.config :logger,
    level: :info

Config.config :logger, :console,
    metadata: [:module, :pid]


Config.config :fast_decimal,
    #decimal_impl: Decimal
    decimal_impl: FastDecimal.Impl.RustDecimal
    #decimal_impl: FastDecimal.Impl.Fastnum


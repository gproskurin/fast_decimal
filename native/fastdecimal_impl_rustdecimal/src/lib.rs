use rust_decimal::Decimal;
use rustler;
use rustler::{Binary, Env};
use std::str::FromStr;
use fastdecimal_impl_lib;


/// Parse a decimal string into a compact 20-byte **binary**.
#[rustler::nif(name="new")]
fn from_str<'a>(env: Env<'a>, s: String) -> rustler::NifResult<Binary<'a>> {
    match Decimal::from_str(&s)
    {
        Ok(d) => {
            Ok(fastdecimal_impl_lib::dec_to_binary(env, &d))
        }
        Err(_) => {
            Err(rustler::Error::BadArg)
        }
    }
}


const POW_10: [i128; 32] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
    100000000000000000000,
    1000000000000000000000,
    10000000000000000000000,
    100000000000000000000000,
    1000000000000000000000000,
    10000000000000000000000000,
    100000000000000000000000000,
    1000000000000000000000000000,
    10000000000000000000000000000,
    100000000000000000000000000000,
    1000000000000000000000000000000,
    10000000000000000000000000000000,
];
#[rustler::nif(name="new")]
fn new_from_elixir<'a>(env: Env<'a>, _sign: i8, coef: i128, exp: i32) -> Binary<'a> {
    let (final_mantissa, scale) =
        if exp < 0 {
            // Exponent -2 becomes Scale 2
            (coef, exp.abs() as u32)
        } else if exp > 0 {
            // Positive exponent means multiplying the mantissa (e.g., 10^2)
            // Note: This may overflow i128 if exp is very large
            //let multiplier = 10i128.pow(exp as u32);
            let multiplier = POW_10[exp as usize];
            (coef * multiplier, 0)
        } else {
            (coef, 0)
        };

    let d = Decimal::from_i128_with_scale(final_mantissa, scale);
    fastdecimal_impl_lib::dec_to_binary(env, &d)
}

/// Multiply two compact decimal **binaries**.
/// NO overflow check: uses `a * b` (may panic on overflow).
#[rustler::nif]
fn mult<'a>(env: Env<'a>, a_bin: Binary<'a>, b_bin: Binary<'a>) -> Binary<'a> {
    let a = fastdecimal_impl_lib::bytes_to_dec::<Decimal>(a_bin.as_ptr());
    let b = fastdecimal_impl_lib::bytes_to_dec::<Decimal>(b_bin.as_ptr());
    let res = a * b; // no checked_mul
    fastdecimal_impl_lib::dec_to_binary(env, &res)
}

#[rustler::nif]
fn div<'a>(env: Env<'a>, a_bin: Binary<'a>, b_bin: Binary<'a>) -> Binary<'a> {
    let a = fastdecimal_impl_lib::bytes_to_dec::<Decimal>(a_bin.as_ptr());
    let b = fastdecimal_impl_lib::bytes_to_dec::<Decimal>(b_bin.as_ptr());
    let res = a / b;
    fastdecimal_impl_lib::dec_to_binary(env, &res)
}

#[rustler::nif(name="equal?")]
fn eq(a: Binary, b: Binary) -> bool {
    return fastdecimal_impl_lib::bytes_to_dec::<Decimal>(a.as_ptr()) == fastdecimal_impl_lib::bytes_to_dec::<Decimal>(b.as_ptr());
}


#[rustler::nif(name="gt?")]
fn gt(a: Binary, b: Binary) -> bool
{
    return fastdecimal_impl_lib::bytes_to_dec::<Decimal>(a.as_ptr()) > fastdecimal_impl_lib::bytes_to_dec::<Decimal>(b.as_ptr());
}


/// Optional: to_string for debugging/interop.
/// NO error handling: assumes valid 20-byte binary.
#[rustler::nif]
fn to_string(bin: Binary<'_>) -> String {
    fastdecimal_impl_lib::bytes_to_dec::<Decimal>(bin.as_ptr()).to_string()
}


rustler::init!("Elixir.FastDecimal.Impl.RustDecimal");


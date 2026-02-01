use rust_decimal::Decimal;
use rustler::types::binary::OwnedBinary;
use rustler::{Binary, Env};
use std::str::FromStr;

const DEC_BIN_LEN: usize = 20;

/// Encode Decimal as <<mantissa::little-128, scale::little-32>> into a VM-owned Binary.
fn encode_decimal_into_owned<'a>(env: Env<'a>, d: &Decimal) -> Binary<'a> {
    let mantissa = d.mantissa(); // i128 (signed)
    let scale = d.scale();       // u32

    let mut out = OwnedBinary::new(DEC_BIN_LEN).unwrap();
    let buf = out.as_mut_slice();
    buf[0..16].copy_from_slice(&mantissa.to_le_bytes());
    buf[16..20].copy_from_slice(&scale.to_le_bytes());

    out.release(env)
}

/// Decode from <<mantissa::little-128, scale::little-32>> into Decimal.
/// NO checks (assumes exactly 20 bytes, valid contents).
fn decode_decimal_unchecked(bin: Binary<'_>) -> Decimal {
    let mut mantissa_bytes = [0u8; 16];
    mantissa_bytes.copy_from_slice(&bin.as_slice()[0..16]);
    let mantissa = i128::from_le_bytes(mantissa_bytes);

    let mut scale_bytes = [0u8; 4];
    scale_bytes.copy_from_slice(&bin.as_slice()[16..20]);
    let scale = u32::from_le_bytes(scale_bytes);

    // Construct via from_i128_with_scale as requested
    Decimal::from_i128_with_scale(mantissa, scale)
}

/// Parse a decimal string into a compact 20-byte **binary**.
/// NO error handling: will panic on invalid string.
#[rustler::nif(name="new")]
fn from_str<'a>(env: Env<'a>, s: String) -> Binary<'a> {
    let d = Decimal::from_str(&s).unwrap();
    encode_decimal_into_owned(env, &d)
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
    encode_decimal_into_owned(env, &d)
}

/// Multiply two compact decimal **binaries**.
/// NO overflow check: uses `a * b` (may panic on overflow).
#[rustler::nif]
fn mult<'a>(env: Env<'a>, a_bin: Binary<'a>, b_bin: Binary<'a>) -> Binary<'a> {
    let a = decode_decimal_unchecked(a_bin);
    let b = decode_decimal_unchecked(b_bin);
    let res = a * b; // no checked_mul
    encode_decimal_into_owned(env, &res)
}

#[rustler::nif]
fn div<'a>(env: Env<'a>, a_bin: Binary<'a>, b_bin: Binary<'a>) -> Binary<'a> {
    let a = decode_decimal_unchecked(a_bin);
    let b = decode_decimal_unchecked(b_bin);
    let res = a / b;
    encode_decimal_into_owned(env, &res)
}

#[rustler::nif(name="equal?")]
fn eq(a: Binary, b: Binary) -> bool {
    return decode_decimal_unchecked(a) == decode_decimal_unchecked(b);
}


#[rustler::nif(name="gt?")]
fn gt(a: Binary, b: Binary) -> bool
{
    return decode_decimal_unchecked(a) > decode_decimal_unchecked(b);
}


/// Optional: to_string for debugging/interop.
/// NO error handling: assumes valid 20-byte binary.
#[rustler::nif]
fn to_string(bin: Binary<'_>) -> String {
    let d = decode_decimal_unchecked(bin);
    d.to_string()
}


rustler::init!("Elixir.FastDecimal.Impl.RustDecimal");


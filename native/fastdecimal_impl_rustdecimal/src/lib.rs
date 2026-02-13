use rust_decimal::Decimal;
use rustler;
use rustler::{Binary, Env};
use std::str::FromStr;

const TOTAL_SIZE: usize = std::mem::size_of::<Decimal>();


fn bytes_to_dec(bytes_ptr: *const u8) -> Decimal {
    let mut d = std::mem::MaybeUninit::<Decimal>::uninit();
    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes_ptr,
            d.as_mut_ptr() as *mut u8,
            TOTAL_SIZE
        );
    }
    unsafe { d.assume_init() }
}


#[inline]
fn dec_to_binary(d: &Decimal) -> rustler::OwnedBinary
{
    let mut bin = rustler::OwnedBinary::new(TOTAL_SIZE).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(
            d as *const Decimal as *const u8,
            bin.as_mut_ptr(),
            TOTAL_SIZE
        );
    }
    bin
}


/// Parse a decimal string into a compact 20-byte **binary**.
#[rustler::nif(name="new")]
fn from_str<'a>(env: Env<'a>, s: String) -> rustler::NifResult<Binary<'a>> {
    match Decimal::from_str(&s)
    {
        Ok(d) => {
            Ok(dec_to_binary(&d).release(env))
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
    dec_to_binary(&d).release(env)
}

/// Multiply two compact decimal **binaries**.
/// NO overflow check: uses `a * b` (may panic on overflow).
#[rustler::nif]
fn mult<'a>(env: Env<'a>, a_bin: Binary<'a>, b_bin: Binary<'a>) -> Binary<'a> {
    let a = bytes_to_dec(a_bin.as_ptr());
    let b = bytes_to_dec(b_bin.as_ptr());
    let res = a * b; // no checked_mul
    dec_to_binary(&res).release(env)
}

#[rustler::nif]
fn div<'a>(env: Env<'a>, a_bin: Binary<'a>, b_bin: Binary<'a>) -> Binary<'a> {
    let a = bytes_to_dec(a_bin.as_ptr());
    let b = bytes_to_dec(b_bin.as_ptr());
    let res = a / b;
    dec_to_binary(&res).release(env)
}

#[rustler::nif(name="equal?")]
fn eq(a: Binary, b: Binary) -> bool {
    return bytes_to_dec(a.as_ptr()) == bytes_to_dec(b.as_ptr());
}


#[rustler::nif(name="gt?")]
fn gt(a: Binary, b: Binary) -> bool
{
    return bytes_to_dec(a.as_ptr()) > bytes_to_dec(b.as_ptr());
}


/// Optional: to_string for debugging/interop.
/// NO error handling: assumes valid 20-byte binary.
#[rustler::nif]
fn to_string(bin: Binary<'_>) -> String {
    bytes_to_dec(bin.as_ptr()).to_string()
}


rustler::init!("Elixir.FastDecimal.Impl.RustDecimal");


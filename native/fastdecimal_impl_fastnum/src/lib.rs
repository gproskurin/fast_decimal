//#![cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]

#[cfg(not(any(feature="dec_type_d64", feature="dec_type_d128")))]
compile_error!("At least some dec_type_* feature must be enabled");

use fastnum;
use rustler;


#[cfg(feature="dec_type_d64")]
pub type Dec = fastnum::decimal::D64;

#[cfg(feature="dec_type_d128")]
pub type Dec = fastnum::decimal::D128;

const TOTAL_SIZE: usize = std::mem::size_of::<Dec>();


#[inline]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn bytes_to_dec(bytes_ptr: *const u8) -> Dec
{
    let mut d = std::mem::MaybeUninit::<Dec>::uninit();
    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes_ptr,
            d.as_mut_ptr() as *mut u8,
            TOTAL_SIZE
        );
    }
    unsafe { d.assume_init() }
}


#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn dec_to_binary<'a>(env: rustler::Env<'a>, d: &Dec) -> rustler::Binary<'a>
{
    let mut nb = rustler::NewBinary::new(env, TOTAL_SIZE);
    unsafe {
        nb.as_mut_slice().copy_from_slice(std::slice::from_raw_parts(d as *const Dec as *const u8, TOTAL_SIZE));
    }
    nb.into()
}


#[rustler::nif(name="new")]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
pub fn from_str<'a>(env: rustler::Env<'a>, s: String) -> rustler::NifResult<rustler::Binary<'a>>
{
    let ctx = fastnum::decimal::Context::default();
    match Dec::from_str(&s, ctx)
    {
        Ok(d) => {
            Ok(dec_to_binary(env, &d))
        },
        Err(_) => {
            Err(rustler::Error::BadArg)
        }
    }
}


// Semantics: value = mantissa * 10^(scale)
#[rustler::nif(name="new")]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn from_mantissa_scale<'a>(env: rustler::Env<'a>, _sign: i8, mantissa: u64, scale: i32) -> rustler::Binary<'a> {
    let ctx = fastnum::decimal::Context::default();
    let value: Dec = Dec::from_u64(mantissa) * Dec::quantum(scale, ctx); // FIXME u64
    dec_to_binary(env, &value)
}


#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn do_op2<'a, F>(
    f: F,
    env: rustler::Env<'a>,
    a: rustler::Binary<'a>,
    b: rustler::Binary<'a>
) -> rustler::NifResult<rustler::Binary<'a>>
where
    F: Fn(&Dec, &Dec) -> Dec
{
    if a.len() != TOTAL_SIZE || b.len() != TOTAL_SIZE {
        return Err(rustler::Error::BadArg);
    }
    let da = bytes_to_dec(a.as_ptr());
    let db = bytes_to_dec(b.as_ptr());
    let r = f(&da, &db);
    Ok(dec_to_binary(env, &r))
}


#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn do_op2_bool<'a, F>(f: F, a: rustler::Binary<'a>, b: rustler::Binary<'a>) -> rustler::NifResult<bool>
where
    F: Fn(&Dec, &Dec) -> bool
{
    if a.len() != TOTAL_SIZE || b.len() != TOTAL_SIZE {
        return Err(rustler::Error::BadArg);
    }
    let da = bytes_to_dec(a.as_ptr());
    let db = bytes_to_dec(b.as_ptr());
    return Ok(f(&da, &db));
}



#[rustler::nif()]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn mult<'a>(env: rustler::Env<'a>, a: rustler::Binary<'a>, b: rustler::Binary<'a>) -> rustler::NifResult<rustler::Binary<'a>>
{
    do_op2(|x: &Dec, y: &Dec| *x * *y, env, a, b)
}


#[rustler::nif()]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn div<'a>(env: rustler::Env<'a>, a: rustler::Binary<'a>, b: rustler::Binary<'a>) -> rustler::NifResult<rustler::Binary<'a>>
{
    do_op2(|x: &Dec, y: &Dec| {*x / *y}, env, a, b)
}


#[rustler::nif(name="equal?")]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn eq<'a>(a: rustler::Binary<'a>, b: rustler::Binary<'a>) -> rustler::NifResult<bool>
{
    do_op2_bool(|x: &Dec, y: &Dec| {*x == *y}, a, b)
}


#[rustler::nif(name="gt?")]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn gt<'a>(a: rustler::Binary<'a>, b: rustler::Binary<'a>) -> rustler::NifResult<bool>
{
    do_op2_bool(|x: &Dec, y: &Dec| {*x > *y}, a, b)
}


#[rustler::nif]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn to_string(bin: rustler::Binary) -> rustler::NifResult<String> {
    if bin.len() != TOTAL_SIZE {
        return Err(rustler::Error::BadArg);
    }
    let d = bytes_to_dec(bin.as_ptr());
    Ok(d.to_string())
}


rustler::init!("Elixir.FastDecimal.Impl.Fastnum");


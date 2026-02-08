//#![cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]

#[cfg(not(any(feature="dec_type_d64", feature="dec_type_d128")))]
compile_error!("At least some dec_type_* feature must be enabled");

use fastnum;
use rustler;

pub trait DecTraits {
    const _COEF_SIZE: usize;
    const _CONTROL_BLOCK_SIZE: usize = 8;
    const TOTAL_SIZE: usize = Self::_COEF_SIZE + Self::_CONTROL_BLOCK_SIZE;
}

impl DecTraits for fastnum::decimal::D64 {
    const _COEF_SIZE: usize = 8;
}

impl DecTraits for fastnum::decimal::D128 {
    const _COEF_SIZE: usize = 16;
}


#[cfg(feature="dec_type_d64")]
pub type Dec = fastnum::decimal::D64;

#[cfg(feature="dec_type_d128")]
pub type Dec = fastnum::decimal::D128;


const TOTAL_SIZE: usize = <Dec as DecTraits>::TOTAL_SIZE;


#[inline]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
unsafe fn bytes_to_dec(bytes: &[u8]) -> Dec
{
    let mut tmp = std::mem::MaybeUninit::<Dec>::uninit();
    std::ptr::copy_nonoverlapping(
        bytes.as_ptr(),
        tmp.as_mut_ptr() as *mut u8,
        TOTAL_SIZE
    );
    tmp.assume_init()
}


#[inline]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
unsafe fn dec_to_bytes(d: &Dec) -> [u8; TOTAL_SIZE]
{
    let mut out = [0u8; TOTAL_SIZE];
    std::ptr::copy_nonoverlapping(
        d as *const Dec as *const u8,
        out.as_mut_ptr(),
        TOTAL_SIZE
    );
    out
}


#[rustler::nif(name="new")]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
pub fn from_str<'a>(env: rustler::Env<'a>, s: String) -> rustler::Binary<'a>
{
    let ctx = fastnum::decimal::Context::default();
    let d = Dec::from_str(&s, ctx).unwrap(); // TODO add check
    let raw = unsafe { dec_to_bytes(&d) };
    let mut bin = rustler::OwnedBinary::new(TOTAL_SIZE).unwrap();
    bin.as_mut_slice().copy_from_slice(&raw);
    bin.release(env)
}


// Semantics: value = mantissa * 10^(scale)
#[rustler::nif(name="new")]
#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn from_mantissa_scale<'a>(env: rustler::Env<'a>, _sign: i8, mantissa: u64, scale: i32) -> rustler::Binary<'a> {
    let ctx = fastnum::decimal::Context::default();
    let value: Dec = Dec::from_u64(mantissa) * Dec::quantum(scale, ctx); // FIXME u64

    let raw = unsafe { dec_to_bytes(&value) };
    let mut bin = rustler::OwnedBinary::new(TOTAL_SIZE).unwrap();
    bin.as_mut_slice().copy_from_slice(&raw);
    bin.release(env)
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
    let da = unsafe { bytes_to_dec(a.as_slice()) };
    let db = unsafe { bytes_to_dec(b.as_slice()) };
    let r = f(&da, &db);
    let bytes = unsafe { dec_to_bytes(&r) };
    let mut bin = rustler::OwnedBinary::new(TOTAL_SIZE).unwrap();
    bin.as_mut_slice().copy_from_slice(&bytes);
    Ok(bin.release(env))
}


#[cfg(any(feature="dec_type_d64", feature="dec_type_d128"))]
fn do_op2_bool<'a, F>(f: F, a: rustler::Binary<'a>, b: rustler::Binary<'a>) -> rustler::NifResult<bool>
where
    F: Fn(&Dec, &Dec) -> bool
{
    if a.len() != TOTAL_SIZE || b.len() != TOTAL_SIZE {
        return Err(rustler::Error::BadArg);
    }
    let da = unsafe { bytes_to_dec(a.as_slice()) };
    let db = unsafe { bytes_to_dec(b.as_slice()) };
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
    let d = unsafe { bytes_to_dec(bin.as_slice()) };
    Ok(d.to_string())
}


rustler::init!("Elixir.FastDecimal.Impl.Fastnum");


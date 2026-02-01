use fastnum::decimal::Context;
use fastnum::UD64;
use rustler::{Binary, Env, OwnedBinary};


const RAW_SIZE: usize = 16;


#[inline]
unsafe fn bytes_to_ud64_raw(bytes: &[u8]) -> UD64 {
    let mut tmp = std::mem::MaybeUninit::<UD64>::uninit();
    std::ptr::copy_nonoverlapping(
        bytes.as_ptr(),
        tmp.as_mut_ptr() as *mut u8,
        RAW_SIZE,
    );
    tmp.assume_init()
}


#[inline]
unsafe fn ud64_to_bytes_raw(d: &UD64) -> [u8; 16] {
    let mut out = [0u8; 16];
    std::ptr::copy_nonoverlapping(
        d as *const UD64 as *const u8,
        out.as_mut_ptr(),
        RAW_SIZE,
    );
    out
}


#[rustler::nif(name="new")]
pub fn from_str<'a>(env: Env<'a>, s: String) -> Binary<'a> {
    let ctx = Context::default();
    let d = UD64::from_str(&s, ctx).unwrap();
    let raw = unsafe { ud64_to_bytes_raw(&d) };
    let mut bin = OwnedBinary::new(RAW_SIZE).unwrap();
    bin.as_mut_slice().copy_from_slice(&raw);
    bin.release(env)
}


// Semantics: value = mantissa * 10^(scale)
#[rustler::nif(name="new")]
fn from_mantissa_scale<'a>(env: Env<'a>, _sign: i8, mantissa: u64, scale: i32) -> Binary<'a> {
    let ctx = Context::default();
    let value: UD64 = UD64::from_u64(mantissa) * UD64::quantum(scale, ctx);

    let raw = unsafe { ud64_to_bytes_raw(&value) };
    let mut bin = OwnedBinary::new(RAW_SIZE).unwrap();
    bin.as_mut_slice().copy_from_slice(&raw);
    bin.release(env)
}


#[rustler::nif()]
fn mult<'a>(env: Env<'a>, a: Binary<'a>, b: Binary<'a>) -> Binary<'a> {
    let da = unsafe { bytes_to_ud64_raw(a.as_slice()) };
    let db = unsafe { bytes_to_ud64_raw(b.as_slice()) };
    let r = da * db;
    let bytes = unsafe { ud64_to_bytes_raw(&r) };
    let mut bin = OwnedBinary::new(RAW_SIZE).unwrap();
    bin.as_mut_slice().copy_from_slice(&bytes);
    bin.release(env)
}


#[rustler::nif()]
fn div<'a>(env: Env<'a>, a: Binary<'a>, b: Binary<'a>) -> Binary<'a> {
    let da = unsafe { bytes_to_ud64_raw(a.as_slice()) };
    let db = unsafe { bytes_to_ud64_raw(b.as_slice()) };
    let r = da / db;
    let bytes = unsafe { ud64_to_bytes_raw(&r) };
    let mut bin = OwnedBinary::new(RAW_SIZE).unwrap();
    bin.as_mut_slice().copy_from_slice(&bytes);
    bin.release(env)
}


#[rustler::nif(name="equal?")]
fn eq(a: Binary, b: Binary) -> bool {
    let da = unsafe { bytes_to_ud64_raw(a.as_slice()) };
    let db = unsafe { bytes_to_ud64_raw(b.as_slice()) };
    da == db
}


#[rustler::nif(name="gt?")]
fn gt(a: Binary, b: Binary) -> bool {
    let da = unsafe { bytes_to_ud64_raw(a.as_slice()) };
    let db = unsafe { bytes_to_ud64_raw(b.as_slice()) };
    da > db
}


#[rustler::nif]
fn to_string(ud_raw: Binary) -> String {
    let d = unsafe { bytes_to_ud64_raw(ud_raw.as_slice()) };
    d.to_string()
}


rustler::init!("Elixir.FastDecimal.Impl.Fastnum");


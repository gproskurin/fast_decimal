use rustler;


pub fn bytes_to_dec<D>(bytes_ptr: *const u8) -> D
{
    let mut d = std::mem::MaybeUninit::<D>::uninit();
    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes_ptr,
            d.as_mut_ptr() as *mut u8,
            std::mem::size_of::<D>()
        );
    }
    unsafe { d.assume_init() }
}


pub fn dec_to_binary<'a, D>(env: rustler::Env<'a>, d: &D) -> rustler::Binary<'a>
{
    let mut nb = rustler::NewBinary::new(env, std::mem::size_of::<D>());
    unsafe {
        nb.as_mut_slice().copy_from_slice(std::slice::from_raw_parts(d as *const D as *const u8, std::mem::size_of::<D>()));
    }
    nb.into()
}


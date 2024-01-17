extern crate gmp_mpfr_sys;

use gmp_mpfr_sys::gmp;
use num_bigint::BigUint;
use std::ptr::NonNull;
use std::ffi::c_void;

fn main() {
        let mut mpz = gmp::mpz_t { alloc: 0, size: 0, d: NonNull::dangling() };
        unsafe {gmp::mpz_init_set_str(&mut mpz, b"12345678901234567890\0".as_ptr() as *const i8, 10) };

        let mut buffer = vec![0u8; 100];

        // Export the raw bytes using mpz_export
        let mut count: usize = 0;
        unsafe { gmp::mpz_export(
            buffer.as_mut_ptr() as *mut c_void,
            &mut count,
            1,
            1,
            0,
            0,
            &mut mpz,
        ); }

        // Resize the buffer based on the actual count
        buffer.resize(count, 0);

        // Convert the raw bytes to a BigUint
        let big_uint = BigUint::from_bytes_be(&buffer);

        // Print or use the BigUint as needed
        println!("BigUint: {}", big_uint);
}
extern crate gmp_mpfr_sys;

use gmp_mpfr_sys::gmp;
use num_bigint::BigUint;
use std::ffi::c_void;
use std::ptr::NonNull;
use rug::Integer;

fn main() {
        // Create an Integer from a string
        // let rug_integer = Integer::from_str_radix("12345678901234567890", 10).unwrap();
        // println!("Rug Integer: {}", rug_integer);
        // println!("Rug integer multiplied by 2: {}", rug_integer * 2);

        let mut mpz = gmp::mpz_t { alloc: 0, size: 0, d: NonNull::dangling() };
        unsafe {gmp::mpz_init_set_str(&mut mpz, b"12345678901234567890\0".as_ptr() as *const i8, 10) };

        // Create an Integer from the mpz_t pointer
        let rug_integer2 = unsafe { Integer::from_raw(mpz) };
        let rug_integer3 = rug_integer2.clone();
        println!("Rug Integer: {}", rug_integer2);
        println!("Rug integer multiplied by 2: {}", rug_integer3 * 2);


        // We can calculate the size of the buffer needed to export the mpz
        //let size = unsafe { gmp::mpz_sizeinbase(&mpz, 256) } as usize;
        // Or we can just use a fixed size as a maximum (32 bytes for 256 bits)
        let size = 32;
        let mut buffer = vec![0u8; size];

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

        // Convert the raw bytes to a BigUint
        let big_uint = BigUint::from_bytes_be(&buffer[..count]);

        // Print or use the BigUint as needed
        println!("BigUint: {}", big_uint);
}
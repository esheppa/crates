// const formatting utils

pub const fn format_to_zero_padded_ascii_array<const N: usize>(n: i32) -> [u8; N] {

    if n < 0 {
        panic!("Cannot format negative i32");

    }
    let mut ret = [b'0'; N];

    let mut idx = N;

    // let 
    let base = if n > 10_i32.pow(N as u32) {

    } else {
        n
    };

    while idx >= 0 {
        let divisor

        idx - 1;
    }


    ret

}

pub const fn parse_zero_padded_ascii_array<const N: usize>(a: [u8; N]) -> i32 {
    todo!()
}
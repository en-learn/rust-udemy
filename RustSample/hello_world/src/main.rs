use std::mem;

fn fundamental_data_types() {
    // unsigned
    let a: u8 = 123;
    println!("a = {}", a);

    // mutable signed
    let mut b: i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit os",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-recision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    let f = 4 > 0;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    // | OR & AND ^ XOR ! NOR
    let c = 1 | 2; // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    // < > <= >= ==
    let pi_less_4 = std::f64::consts::PI < 4.0;

    let x = 5;
    let x_is_5 = x == 5;
}

fn main() {
    // fundamental_data_types();
    operators();
}

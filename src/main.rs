mod cpu;

fn main() {
    /*
    let mut r = cpu::Registers{a:255, b: 0};
    println!("{:#018b}", r.get_ab());

    let mut a = r.a as u16;
    println!("{:#018b}", a);
    a = a << 8;
    println!("{:#018b}", a);

    let b = ((a & 0xFF00) >> 8) as u8;
    println!("b: {:#010b}", b);

    r.set_ab(255);
    println!("{:#018b}", r.get_ab());
    r.set_ab(u16::MAX);
    println!("{:#018b}", r.get_ab());
    */
    println!("Hello, world!");
}

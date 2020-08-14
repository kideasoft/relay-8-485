use rand;
use relay_8_485::{ChannelNum, Relay};

fn main() {
    let mut relay = Relay::new("/dev/ttyUSB1", 0x01, 115200).unwrap();
    let r = relay.get_coils().unwrap();
    println!("{:?}", r);

    relay.on(ChannelNum::Four).unwrap();

    for _ in 0..100 {
        let i = rand::random::<u8>() % 8;
        let chan = match i {
            0 => ChannelNum::One,
            1 => ChannelNum::Two,
            2 => ChannelNum::Three,
            3 => ChannelNum::Four,
            4 => ChannelNum::Five,
            5 => ChannelNum::Six,
            6 => ChannelNum::Seven,
            _ => ChannelNum::Eight,
        };

        let sw = rand::random::<bool>();
        if sw {
            relay.on(chan).unwrap();
        } else {
            relay.off(chan).unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    // for i in 0..8 {
    //     println!("switch on {}", i);
    //     let chan = match i {
    //         0 => ChannelNum::One,
    //         1 => ChannelNum::Two,
    //         2 => ChannelNum::Three,
    //         3 => ChannelNum::Four,
    //         4 => ChannelNum::Five,
    //         5 => ChannelNum::Six,
    //         6 => ChannelNum::Seven,
    //         _ => ChannelNum::Eight,
    //     };
    //     relay.on_reg(chan).unwrap();
    //     println!("{:?}", relay.get_coils().unwrap());
    //     std::thread::sleep(std::time::Duration::from_millis(20));
    // }

    // for i in 0..8 {
    //     println!("switch off {}", i);
    //     let chan = match i {
    //         0 => ChannelNum::One,
    //         1 => ChannelNum::Two,
    //         2 => ChannelNum::Three,
    //         3 => ChannelNum::Four,
    //         4 => ChannelNum::Five,
    //         5 => ChannelNum::Six,
    //         6 => ChannelNum::Seven,
    //         _ => ChannelNum::Eight,
    //     };
    //     relay.off_reg(chan).unwrap();
    //     println!("{:?}", relay.get_coils().unwrap());
    //     std::thread::sleep(std::time::Duration::from_millis(20));
    // }

    for i in 0..8 {
        println!("switch on {}", i);
        let chan = match i {
            0 => ChannelNum::One,
            1 => ChannelNum::Two,
            2 => ChannelNum::Three,
            3 => ChannelNum::Four,
            4 => ChannelNum::Five,
            5 => ChannelNum::Six,
            6 => ChannelNum::Seven,
            _ => ChannelNum::Eight,
        };
        relay.on(chan).unwrap();
        println!("{:?}", relay.get_coils().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    for i in 0..8 {
        println!("switch off {}", i);
        let chan = match i {
            0 => ChannelNum::One,
            1 => ChannelNum::Two,
            2 => ChannelNum::Three,
            3 => ChannelNum::Four,
            4 => ChannelNum::Five,
            5 => ChannelNum::Six,
            6 => ChannelNum::Seven,
            _ => ChannelNum::Eight,
        };
        relay.off(chan).unwrap();
        println!("{:?}", relay.get_coils().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    for i in 0..8 {
        println!("switch on {}", i);
        let chan = match i {
            0 => ChannelNum::One,
            1 => ChannelNum::Two,
            2 => ChannelNum::Three,
            3 => ChannelNum::Four,
            4 => ChannelNum::Five,
            5 => ChannelNum::Six,
            6 => ChannelNum::Seven,
            _ => ChannelNum::Eight,
        };
        relay.on(chan).unwrap();
        println!("{:?}", relay.get_coils().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

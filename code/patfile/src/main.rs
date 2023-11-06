use std::fs::File;

use patfile::{pwrite, pscan};

mod test;

fn main() {
    // write to file
    {
        let mut file = File::create("data").unwrap();
        pwrite!(&mut file, "test1 {} test1", 1).unwrap();
        pwrite!(&mut file, "test2 {} test2", 2).unwrap();
    }

    // read from file
    {
        let file = File::open("data").unwrap();
        let mut a: i32 = 0;
        let mut b: i32 = 0;

        use std::io::Read;
        let it: &mut dyn Iterator<Item = u8> = &mut file.bytes().map(std::result::Result::unwrap);

        pscan!(it => "test1 {} test1", a).map_err(|err| panic!("{}", err)).unwrap();
        pscan!(it => "test2 {} test2", b).map_err(|err| panic!("{}", err)).unwrap();

        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }
}



#[cfg(test)]
use {std::fs::File, patfile::Error, crate::{pscan, pwrite}};


#[test]
fn test_scan_simple() {
    let input = "{1}";
    let input: &mut dyn Iterator<Item = u8> = &mut input.bytes();

    let mut a: i32 = 0;
    pscan!(input => "{}", a)
        .map_err(|err| panic!("{}", err))
        .unwrap();

    assert_eq!(a, 1);
}

#[test]
fn test_scan_multiple() {
    let input = "{1} {2}";

    let input: &mut dyn Iterator<Item = u8> = &mut input.bytes();

    let mut a: i32 = 0;
    let mut b: i32 = 0;
    pscan!(input => "{} {}", a, b)
        .map_err(|err| panic!("{}", err))
        .unwrap();

    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

#[test]
fn test_scan_missing_input_brace() {
    let input = "1";

    let input: &mut dyn Iterator<Item = u8> = &mut input.bytes();

    let mut _a: i32 = 0;
    match pscan!(input => "{}", _a) {
        Ok(_) => panic!("Scan should return error"),
        Err(err) => match err {
            Error::MissingOpeningBrace => return,
            _ => panic!("Wrong error returned"),
        },
    }
}

#[test]
fn test_scan_nested() {
    let input = "{1} {2}";

    let input: &mut dyn Iterator<Item = u8> = &mut input.bytes();

    let mut a: i32 = 0;
    let mut b: i32 = 0;
    pscan!(input => "{} {}", a, b)
        .map_err(|err| panic!("{}", err))
        .unwrap();

    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

#[test]
fn test_write_file_simple() {
    // write to file
    {
        let mut file = File::create("testfiles/test_write_file_simple").unwrap();
        file.set_len(0).unwrap(); // clear file
        pwrite!(&mut file, "[{}] [{}]\n", 1, 2).unwrap();
    }

    // read from file
    {
        let mut file = File::open("testfiles/test_write_file_simple").unwrap();
        use std::io::Read;
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        assert_eq!(text, "[{1}] [{2}]\n");
    }
}

#[test]
fn test_write_file_complex() {
    // write to file
    {
        let mut file = File::create("testfiles/test_write_file_complex").unwrap();
        file.set_len(0).unwrap(); // clear file
        pwrite!(&mut file, "test1 {} test2 {} test3\n", 1, 2).unwrap();
        pwrite!(&mut file, "test4 {} test5 {} test6", 3, 4).unwrap();
    }

    // read from file
    {
        let mut file = File::open("testfiles/test_write_file_complex").unwrap();
        use std::io::Read;
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        assert_eq!(text, "test1 {1} test2 {2} test3\ntest4 {3} test5 {4} test6");
    }
}

#[test]
fn test_scan_file() {
    // write to file
    {
        let mut file = File::create("testfiles/test_scan_file").unwrap();
        file.set_len(0).unwrap(); // clear file
        pwrite!(&mut file, "test1 {} test1", 1).unwrap();
        pwrite!(&mut file, "test2 {} test2", 2).unwrap();
    }

    // read from file
    {
        let file = File::open("testfiles/test_scan_file").unwrap();
        let mut a: i32 = 0;
        let mut b: i32 = 0;

        use std::io::Read;
        let it: &mut dyn Iterator<Item = u8> = &mut file.bytes().map(std::result::Result::unwrap);

        pscan!(it => "test1 {} test1", a)
            .map_err(|err| panic!("{}", err))
            .unwrap();
        pscan!(it => "test2 {} test2", b)
            .map_err(|err| panic!("{}", err))
            .unwrap();

        assert_eq!(a, 1);
        assert_eq!(b, 2);
    }
}

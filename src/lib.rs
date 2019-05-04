#![allow(dead_code)]
mod divsufsort;

pub fn apply_bwt(data: &mut Vec<u8>) -> i32 {
    let mut sa = Vec::with_capacity(data.len());
    sa.resize(data.len(), 0);
    let pidx: i32;
    unsafe {
        pidx = divsufsort::divbwt(data.as_ptr(), data.as_mut_ptr(), sa.as_mut_ptr(), data.len() as i32);
    }
    pidx
}

pub fn reverse_bwt(data: &mut Vec<u8>, pidx: i32) {
    let mut sa = Vec::with_capacity(data.len());
    sa.resize(data.len(), 0);
    unsafe {
        divsufsort::inverse_bw_transform(data.as_ptr(), data.as_mut_ptr(), sa.as_mut_ptr(), data.len() as i32, pidx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    #[ignore]
    fn test_with_bwt_cli() {
        let path = std::env::current_dir().unwrap();
        let test = path.join("data/test.s31");
        let mut f = File::open(test).unwrap();
        let mut data : Vec<u8> = Vec::new();
        f.read_to_end(&mut data).unwrap();
        let expected_d = data.clone();
        let test = path.join("data/test.s31.bwt");
        let mut g = File::open(test).unwrap();
        let mut expected :Vec<u8> = Vec::new();
        g.read_to_end(&mut expected).unwrap();

        let pidx = apply_bwt(&mut data);
        assert_eq!(data, expected.iter().skip(8).map(|&x| x).collect::<Vec<u8>>());

        reverse_bwt(&mut data, pidx);
        assert_eq!(data, expected_d);
    }

    #[test]
    fn test_bwt() {
        let mut data : Vec<u8> = vec![154,67,101,83,2,2,2,2,2,2,89,99,3,168,234,24,12,34];
        let expected = data.clone();
        let pidx = apply_bwt(&mut data);
        reverse_bwt(&mut data, pidx);
        assert_eq!(data, expected);
    }
}

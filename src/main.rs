
mod divsufsort;
use std::fs::File;
use std::io::prelude::*;

fn apply_bwt(data: &mut Vec<u8>) -> i32 {
    let mut sa = Vec::with_capacity(data.len());
    sa.resize(data.len(), 0);
    let pidx: i32;
    unsafe {
        // e = divsufsort::divsufsort(data.as_ptr(), sa.as_mut_ptr(), data.len() as i32);
        pidx = divsufsort::divbwt(data.as_ptr(), data.as_mut_ptr(), sa.as_mut_ptr(), data.len() as i32);
    }
    pidx
}

fn reverse_bwt(data: &mut Vec<u8>, pidx: i32) {
    let mut sa = Vec::with_capacity(data.len());
    sa.resize(data.len(), 0);
    unsafe {
        divsufsort::inverse_bw_transform(data.as_ptr(), data.as_mut_ptr(), sa.as_mut_ptr(), data.len() as i32, pidx);
    }
}


fn main() {
    let mut data : Vec<u8> = vec![154,67,101,83,2,2,2,2,2,2,89,99,3,168,234,24,12,34];
    println!("               data {:?}", data);
    let mut buffer = File::create("/tmp/test.raw").unwrap();
    buffer.write_all(&data[..]).unwrap();
    // let mut f = File::open("/tmp/test.s31").unwrap();
    // let mut data : Vec<u8> = Vec::new();
    // f.read_to_end(&mut data).unwrap();
    // let mut g = File::open("/tmp/test.s31.bwt").unwrap();
    // let mut expected :Vec<u8> = Vec::new();
    // g.read_to_end(&mut expected).unwrap();

    let pidx = apply_bwt(&mut data);
    println!("                bwt {:?} (pidx: {})", data, pidx);
    reverse_bwt(&mut data, pidx);
    println!("     data (decoded) {:?}", data);

    // let mut sa = Vec::with_capacity(data.len());
    // sa.resize(data.len(), 0);
    // let mut ix = 0 as i32;

    // unsafe {
    //     // divsufsort::divsufsort(data.as_ptr(), sa.as_mut_ptr(), data.len() as i32);
    //     d = divsufsort::inverse_bw_transform(data.as_ptr(), data.as_mut_ptr(), sa.as_mut_ptr(), data.len() as i32, ix);
    // }

    // println!("Hello, world! {} {}", e, d);
    // println!("               data {:?}", data);
    // println!("                bwt {:?}", u);
    // println!("suffixarray indices {:?}", sa);
    // assert_eq!(expected, u);
}

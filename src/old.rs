use crate::error::Error;

pub fn memw(src: &[u8], dest: &mut [u8], addr: usize) -> Result<(), Error> {
    if addr + src.len() > dest.len() {
        return Err(Error::InvalidOpCode);
    }
    dest[addr..addr + src.len()].copy_from_slice(src);
    Ok(())
}

pub fn memr32(src: &[u8], addr: usize) -> Result<[u8; 4], Error> {
    if addr + 4 > src.len() {
        return Err(Error::InvalidOpCode);
    }
    let mut r = [0u8; 4];
    r[0..4].copy_from_slice(&src[addr..addr + 4]);
    Ok(r)
}

pub fn memr16(src: &[u8], addr: usize) -> Result<[u8; 2], Error> {
    if addr + 4 > src.len() {
        return Err(Error::InvalidOpCode);
    }
    let mut r = [0u8; 2];
    r[0..2].copy_from_slice(&src[addr..addr + 2]);
    Ok(r)
}

pub fn memr8(src: &[u8], addr: usize) -> Result<u8, Error> {
    if addr + 4 > src.len() {
        return Err(Error::InvalidOpCode);
    }
    Ok(src[addr])
}

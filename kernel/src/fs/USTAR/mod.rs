

// Stolen from the r3 kernel, credits to Narasimha Prasanna <3
#[inline]
fn oct_to_usize(buffer: &[u8]) -> usize {
    let mut multiplier = 1;
    let mut number = 0;
    let last_index = buffer.len() - 1;

    for idx in 0..(last_index + 1) {
        let byte = buffer[last_index - idx];
        if byte as char >= '0' && byte as char <= '7' {
            number += ((byte - 48) as usize) * multiplier;
            multiplier *= 8;
        }
    }

    number
}
#[test]
fn test()
{
    
}
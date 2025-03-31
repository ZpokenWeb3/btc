pub fn reverse(bytes: &[u8]) -> Vec<u8> {
    let mut reversed = bytes.to_vec();
    reversed.reverse();
    reversed
}

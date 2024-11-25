pub fn get_program(source: Vec<u8>) -> Vec<u32> {
    (0..source.len() / 4)
        .map(|i| {
            let start = i * 4;
            let end = (i + 1) * 4;
            source[start..end]
                .iter()
                .rev()
                .enumerate()
                .map(|(x, &c)| {
                    let n = c as u32;
                    /* loooool Svintoo. En u8 är 4 bitar stor va???? :^) */
                    n << x * 8
                })
                .sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_program() {
        let source: Vec<u8> = vec![0xca, 0xfe, 0xba, 0xbe, 0xde, 0xad, 0xbe, 0xef];
        let expected: Vec<u32> = vec![0xcafebabe, 0xdeadbeef];
        let got = get_program(source);
        assert_eq!(expected, got);
    }
}

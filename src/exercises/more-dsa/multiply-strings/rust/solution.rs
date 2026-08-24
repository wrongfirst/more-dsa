impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let n1: Vec<u8> = num1.bytes().rev().map(|b| b - b'0').collect();
        let n2: Vec<u8> = num2.bytes().rev().map(|b| b - b'0').collect();

        let mut res = vec![0u8; n1.len() + n2.len()];

        for i1 in 0..n1.len() {
            for i2 in 0..n2.len() {
                let digit = n1[i1] as u16 * n2[i2] as u16;
                res[i1 + i2] += digit as u8;
                res[i1 + i2 + 1] += res[i1 + i2] / 10;
                res[i1 + i2] %= 10;
            }
        }

        // Remove leading zeros
        while res.len() > 1 && res.last() == Some(&0) {
            res.pop();
        }

        res.iter().rev().map(|&d| (d + b'0') as char).collect()
    }
}

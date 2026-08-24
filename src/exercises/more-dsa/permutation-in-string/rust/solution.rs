impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1_bytes: Vec<u8> = s1.bytes().collect();
        let s2_bytes: Vec<u8> = s2.bytes().collect();

        let mut s1_count = [0i32; 26];
        let mut s2_count = [0i32; 26];

        for i in 0..s1_bytes.len() {
            s1_count[(s1_bytes[i] - b'a') as usize] += 1;
            s2_count[(s2_bytes[i] - b'a') as usize] += 1;
        }

        let mut matches = 0;
        for i in 0..26 {
            if s1_count[i] == s2_count[i] {
                matches += 1;
            }
        }

        let mut l = 0;
        for r in s1_bytes.len()..s2_bytes.len() {
            if matches == 26 {
                return true;
            }

            let index = (s2_bytes[r] - b'a') as usize;
            s2_count[index] += 1;
            if s1_count[index] == s2_count[index] {
                matches += 1;
            } else if s1_count[index] + 1 == s2_count[index] {
                matches -= 1;
            }

            let index = (s2_bytes[l] - b'a') as usize;
            s2_count[index] -= 1;
            if s1_count[index] == s2_count[index] {
                matches += 1;
            } else if s1_count[index] - 1 == s2_count[index] {
                matches -= 1;
            }
            l += 1;
        }

        matches == 26
    }
}

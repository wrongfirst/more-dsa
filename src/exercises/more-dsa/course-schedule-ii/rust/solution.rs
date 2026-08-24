use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prereq: HashMap<i32, Vec<i32>> = HashMap::new();

        for c in 0..num_courses {
            prereq.insert(c, Vec::new());
        }

        for p in &prerequisites {
            let crs = p[0];
            let pre = p[1];
            prereq.entry(crs).or_default().push(pre);
        }

        let mut output: Vec<i32> = Vec::new();
        let mut visit: HashSet<i32> = HashSet::new();
        let mut cycle: HashSet<i32> = HashSet::new();

        fn dfs(
            crs: i32,
            prereq: &HashMap<i32, Vec<i32>>,
            visit: &mut HashSet<i32>,
            cycle: &mut HashSet<i32>,
            output: &mut Vec<i32>,
        ) -> bool {
            if cycle.contains(&crs) {
                return false;
            }
            if visit.contains(&crs) {
                return true;
            }

            cycle.insert(crs);
            if let Some(pres) = prereq.get(&crs) {
                for &pre in pres {
                    if !dfs(pre, prereq, visit, cycle, output) {
                        return false;
                    }
                }
            }
            cycle.remove(&crs);
            visit.insert(crs);
            output.push(crs);
            true
        }

        for c in 0..num_courses {
            if !dfs(c, &prereq, &mut visit, &mut cycle, &mut output) {
                return Vec::new();
            }
        }

        output
    }
}

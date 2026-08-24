impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut par: Vec<usize> = (0..=n).collect();
        let mut rank: Vec<usize> = vec![1; n + 1];

        fn find(par: &mut Vec<usize>, n: usize) -> usize {
            let mut p = par[n];
            while p != par[p] {
                par[p] = par[par[p]];
                p = par[p];
            }
            p
        }

        fn union(par: &mut Vec<usize>, rank: &mut Vec<usize>, n1: usize, n2: usize) -> bool {
            let p1 = find(par, n1);
            let p2 = find(par, n2);

            if p1 == p2 {
                return false;
            }

            if rank[p1] > rank[p2] {
                par[p2] = p1;
                rank[p1] += rank[p2];
            } else {
                par[p1] = p2;
                rank[p2] += rank[p1];
            }
            true
        }

        for edge in &edges {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            if !union(&mut par, &mut rank, n1, n2) {
                return vec![edge[0], edge[1]];
            }
        }

        vec![]
    }
}

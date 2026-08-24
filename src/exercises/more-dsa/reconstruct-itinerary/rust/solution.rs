use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();

        for ticket in &tickets {
            adj.entry(ticket[0].clone()).or_insert_with(Vec::new);
        }

        for ticket in &tickets {
            adj.get_mut(&ticket[0]).unwrap().push(ticket[1].clone());
        }

        for destinations in adj.values_mut() {
            destinations.sort();
        }

        let mut res: Vec<String> = Vec::new();
        let mut stack: Vec<String> = vec!["JFK".to_string()];

        while let Some(src) = stack.last() {
            if adj.get(src).map_or(true, |d| d.is_empty()) {
                let node = stack.pop().unwrap();
                res.push(node);
            } else {
                let src_clone = src.clone();
                let dest = adj.get_mut(&src_clone).unwrap().remove(0);
                stack.push(dest);
            }
        }

        res.reverse();

        if res.len() != tickets.len() + 1 {
            return Vec::new();
        }

        res
    }
}

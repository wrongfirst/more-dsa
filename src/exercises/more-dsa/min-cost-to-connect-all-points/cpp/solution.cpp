class Solution {
public:
    int minCostConnectPoints(vector<vector<int>>& points) {
        int N = points.size();
        unordered_map<int, vector<pair<int, int>>> adj;
        for (int i = 0; i < N; i++) {
            int x1 = points[i][0];
            int y1 = points[i][1];
            for (int j = i + 1; j < N; j++) {
                int x2 = points[j][0];
                int y2 = points[j][1];
                int dist = abs(x1 - x2) + abs(y1 - y2);
                adj[i].push_back({dist, j});
                adj[j].push_back({dist, i});
            }
        }

        int res = 0;
        unordered_set<int> visit;
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> minH;
        minH.push({0, 0});
        while (visit.size() < N) {
            auto curr = minH.top();
            minH.pop();
            int cost = curr.first;
            int i = curr.second;
            if (visit.count(i)) {
                continue;
            }
            res += cost;
            visit.insert(i);
            for (const auto& nei : adj[i]) {
                int neiCost = nei.first;
                int neiIndex = nei.second;
                if (!visit.count(neiIndex)) {
                    minH.push({neiCost, neiIndex});
                }
            }
        }
        return res;
    }
};

class Solution {
public:
    int maxAreaOfIsland(vector<vector<int>>& grid) {
        int ROWS = grid.size();
        int COLS = grid[0].size();
        unordered_set<string> visit;

        int area = 0;
        for (int r = 0; r < ROWS; r++) {
            for (int c = 0; c < COLS; c++) {
                if (grid[r][c] == 1 && visit.count(to_string(r) + "," + to_string(c)) == 0) {
                    area = max(area, Dfs(grid, r, c, visit));
                }
            }
        }
        return area;
    }

    int Dfs(vector<vector<int>>& grid, int r, int c, unordered_set<string>& visit) {
        if (r < 0 || r == grid.size() || c < 0 || c == grid[0].size() || grid[r][c] == 0 || visit.count(to_string(r) + "," + to_string(c))) {
            return 0;
        }
        visit.insert(to_string(r) + "," + to_string(c));
        return 1 + Dfs(grid, r + 1, c, visit) + Dfs(grid, r - 1, c, visit) + Dfs(grid, r, c + 1, visit) + Dfs(grid, r, c - 1, visit);
    }
};

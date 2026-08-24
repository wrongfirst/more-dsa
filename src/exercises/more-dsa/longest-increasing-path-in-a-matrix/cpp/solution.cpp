class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        if (matrix.empty()) {
            return 0;
        }
        int rows = matrix.size();
        int cols = matrix[0].size();

        int maxDist = 0;
        vector<vector<int>> memo(rows, vector<int>(cols, 0));

        vector<vector<int>> dirs = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

        for (int i = 0; i < rows; ++i) {
            for (int j = 0; j < cols; ++j) {
                maxDist = max(maxDist, dfs(matrix, i, j, memo, dirs));
            }
        }
        return maxDist;
    }

    int dfs(vector<vector<int>>& matrix, int r, int c, vector<vector<int>>& memo, vector<vector<int>>& dirs) {
        if (memo[r][c] != 0) {
            return memo[r][c];
        }

        int res = 1;
        for (const auto& dir : dirs) {
            int newRow = r + dir[0];
            int newCol = c + dir[1];
            if (newRow >= 0 && newRow < matrix.size() && newCol >= 0 && newCol < matrix[0].size()
                && matrix[newRow][newCol] > matrix[r][c]) {
                res = max(res, 1 + dfs(matrix, newRow, newCol, memo, dirs));
            }
        }
        memo[r][c] = res;
        return res;
    }
};

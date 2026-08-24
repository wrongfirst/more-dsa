class CountSquares {
private:
    // Mapping from a combined key of coordinates to their count
    unordered_map<long, int> ptsCount;
    // All points added so far (to iterate over during count)
    vector<vector<int>> pts;

    // Combines two integers into a long key
    long getKey(int x, int y) {
        return (static_cast<long>(x) << 32) | static_cast<long>(y);
    }

public:
    CountSquares() {
    }

    void add(vector<int> point) {
        long key = getKey(point[0], point[1]);
        ptsCount[key]++; // Increment the count for this point
        pts.push_back(point); // Store the point for later iteration
    }
    
    int count(vector<int> point) {
        int res = 0;
        int px = point[0], py = point[1];

        // Iterate over all points to find potential square corners
        for (const auto& pt : pts) {
            int x = pt[0], y = pt[1];
            // Skip if it doesn't form a square side
            if (abs(py - y) != abs(px - x) || x == px || y == py) continue;

            // Check for other two corners of the square
            res += ptsCount[getKey(x, py)] * ptsCount[getKey(px, y)];
        }
        return res;
    }
};

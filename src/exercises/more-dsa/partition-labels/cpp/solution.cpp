class Solution {
public:
    vector<int> partitionLabels(string S) {
        unordered_map<char, int> count;
        vector<int> res;
        int i = 0, length = S.length();
        for (int j = 0; j < length; j++) {
            char c = S[j];
            count[c] = j;
        }

        int curLen = 0;
        int goal = 0;
        while (i < length) {
            char c = S[i];
            goal = max(goal, count[c]);
            curLen++;

            if (goal == i) {
                res.push_back(curLen);
                curLen = 0;
            }
            i++;
        }
        return res;
    }
};

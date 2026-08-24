class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        unordered_map<char, int> counts;
        for (char task : tasks) {
            counts[task]++;
        }

        priority_queue<int> maxHeap;
        for (auto& entry : counts) {
            maxHeap.push(entry.second);
        }

        int time = 0;
        queue<pair<int, int>> queue;  // pairs of {cnt, idleTime}
        while (!maxHeap.empty() || !queue.empty()) {
            if (!queue.empty() && time >= queue.front().second) {
                maxHeap.push(queue.front().first);
                queue.pop();
            }
            if (!maxHeap.empty()) {
                int cnt = maxHeap.top() - 1;
                maxHeap.pop();
                if (cnt > 0) {
                    queue.push({cnt, time + n + 1});
                }
            }
            time++;
        }
        return time;
    }
};

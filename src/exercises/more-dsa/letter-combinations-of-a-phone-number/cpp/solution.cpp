class Solution {
public:
    vector<string> letterCombinations(string digits) {
        vector<string> res;
        if (digits.empty()) {
            return res;
        }

        unordered_map<char, string> digitToChar = {
            {'2', "abc"},
            {'3', "def"},
            {'4', "ghi"},
            {'5', "jkl"},
            {'6', "mno"},
            {'7', "qprs"},
            {'8', "tuv"},
            {'9', "wxyz"}
        };

        backtrack(0, digits, "", res, digitToChar);
        return res;
    }

private:
    void backtrack(int i, const string& digits, string curStr, vector<string>& res,
                   const unordered_map<char, string>& digitToChar) {
        if (curStr.length() == digits.length()) {
            res.push_back(curStr);
            return;
        }

        for (char c : digitToChar.at(digits[i])) {
            backtrack(i + 1, digits, curStr + c, res, digitToChar);
        }
    }
};

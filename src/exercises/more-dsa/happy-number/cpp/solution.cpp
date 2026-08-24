class Solution {
public:
    bool isHappy(int n) {
        int slow = n, fast = sumSquareDigits(n);

        while (slow != fast) {
            fast = sumSquareDigits(fast);
            fast = sumSquareDigits(fast);
            slow = sumSquareDigits(slow);
        }

        return fast == 1;
    }

private:
    int sumSquareDigits(int n) {
        int output = 0;
        while (n != 0) {
            output += (n % 10) * (n % 10);
            n /= 10;
        }
        return output;
    }
};

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func isBalanced(root *TreeNode) bool {
    balanced, _ := dfs(root)
    return balanced
}

func dfs(root *TreeNode) (bool, int) {
    if root == nil {
        return true, 0
    }

    leftBalanced, leftHeight := dfs(root.Left)
    rightBalanced, rightHeight := dfs(root.Right)

    balanced := leftBalanced && rightBalanced && abs(leftHeight-rightHeight) <= 1
    return balanced, 1 + max(leftHeight, rightHeight)
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}

func abs(a int) int {
    if a < 0 {
        return -a
    }
    return a
}

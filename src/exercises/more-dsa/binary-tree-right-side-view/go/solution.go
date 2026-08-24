/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func rightSideView(root *TreeNode) []int {
    var res []int
    queue := []*TreeNode{root}

    for len(queue) > 0 {
        rightSide := (*TreeNode)(nil)
        qLen := len(queue)

        for i := 0; i < qLen; i++ {
            node := queue[0]
            queue = queue[1:]
            if node != nil {
                rightSide = node
                queue = append(queue, node.Left)
                queue = append(queue, node.Right)
            }
        }

        if rightSide != nil {
            res = append(res, rightSide.Val)
        }
    }

    return res
}

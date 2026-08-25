# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

def reverseKGroup(head: Optional[ListNode], k: int) -> Optional[ListNode]:
    dummy = ListNode(0, head)
    groupPrev: Optional[ListNode] = dummy

    while groupPrev:
        kth = getKth(groupPrev, k)
        if not kth:
            break
        groupNext = kth.next

        prev, curr = kth.next, groupPrev.next
        while curr and curr != groupNext:
            tmp = curr.next
            curr.next = prev
            prev = curr
            curr = tmp

        tmp = groupPrev.next
        groupPrev.next = kth
        groupPrev = tmp
    return dummy.next

def getKth(curr: Optional[ListNode], k: int) -> Optional[ListNode]:
    while curr and k > 0:
        curr = curr.next
        k -= 1
    return curr

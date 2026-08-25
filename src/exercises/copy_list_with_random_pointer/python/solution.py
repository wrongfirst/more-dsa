"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

def copyRandomList(head: Optional[Node]) -> Optional[Node]:
    oldToCopy: dict[Optional[Node], Optional[Node]] = {None: None}

    cur = head
    while cur:
        copy = Node(cur.val)
        oldToCopy[cur] = copy
        cur = cur.next
    cur = head
    while cur:
        copy = oldToCopy[cur]
        if copy:
            copy.next = oldToCopy[cur.next]
            copy.random = oldToCopy[cur.random]
        cur = cur.next
    return oldToCopy[head]

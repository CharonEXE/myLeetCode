/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public bool HasCycle(ListNode head) {
        ListNode hare = head;
        ListNode tortoise = head;

        while (hare != null && hare.next != null)
        {
            hare = hare.next.next;
            tortoise = tortoise.next;
            if (hare == tortoise)
                return true;
        }
        return false;
    }
}
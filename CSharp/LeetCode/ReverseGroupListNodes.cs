using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LeetCode
{
    public class ReverseNodesKGroup
    {
        public ListNode ReverseKGroup(ListNode head, int k) {

            ListNode groupHead = head;
            ListNode previousGroupHead = null;
            ListNode nextGroupHead = null;
            ListNode result = null;
            Stack<ListNode> stack = new Stack<ListNode>(k);
            while (groupHead != null)
            {
                ListNode current = groupHead;
                // Count 'k' nodes, push them onto the stack
                int i = 0;
                for (i = 0; i < k && current != null; i++)
                {
                    stack.Push(current);
                    current = current.next;
                }
                // if there aren't 'k' nodes left in the list, we're done.
                // Return
                if (i < k)
                {
                    return result ?? groupHead;
                }
                // This is the furthest we go in this loop. Save a reference to
                // the start of the next group
                nextGroupHead = current;
                
                // start reversing the list, using the stack
                current = stack.Pop();
                // if there is no previous group, this is the first group we've reversed,
                // so save a reference to the new start of this list - it will
                // be the final return value
                if (previousGroupHead == null)
                {
                    result = current;
                    previousGroupHead = groupHead;
                }
                else
                {
                    previousGroupHead.next = current;
                }

                // reverse the group
                while (stack.Count > 0)
                {
                    ListNode previous = stack.Pop();
                    current.next = previous;
                    current = previous;
                }
                current.next = groupHead;

                // create the link to the next group
                groupHead.next = nextGroupHead;
                // assign the next group as the current group, for the next loop
                previousGroupHead = groupHead;
                groupHead = nextGroupHead;
            }

            return result;
        }
    }
}

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LeetCode
{
    public class SwapAlternateListNodesSolution
    {
        public ListNode SwapPairs(ListNode head) {
            if (head == null || head.next == null)
            {
                return head;
            }
            ListNode alternate = head.next;
            ListNode tail = SwapPairs(alternate.next);
            alternate.next = head;
            head.next = tail;
            return alternate;
        }
    }
}

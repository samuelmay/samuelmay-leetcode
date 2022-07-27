using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LeetCode
{
    public class MergeListsSolution
    {
        public ListNode MergeKLists(ListNode[] lists) {
            if (lists.Length == 0)
            {
                return null;
            }
            else if (lists.Length == 1)
            {
                return lists[0];
            }
            else if (lists.Length == 2)
            {
                return this.MergeTwoLists(lists[0], lists[1]);
            }
            else
            {
                int half = lists.Length / 2 + 1;
                int otherHalf = lists.Length - half;
                ListNode[] left = new ListNode[half];
                ListNode[] right = new ListNode[otherHalf];
                Array.Copy(lists, left, half);
                Array.Copy(lists, half, right, 0, otherHalf);

                ListNode leftMerged = this.MergeKLists(left);
                ListNode rightMerged = this.MergeKLists(right);

                return this.MergeTwoLists(leftMerged, rightMerged);

            }
        
        }
        private ListNode MergeTwoLists(ListNode list1, ListNode list2) {
            ListNode resultHead;
            ListNode tail;

            if (list1 == null && list2 == null)
            {
                return null;
            }
            else if (list1 == null)
            {
                return list2;
            }
            else if (list2 == null)
            {
                return list1;
            }

            if (list1.val <= list2.val)
            {
                resultHead = list1;
                list1 = list1.next;
            } else {
                resultHead = list2;
                list2 = list2.next;
            }
            tail = resultHead;

            while (list1 != null && list2 != null)
            {
                if (list1.val <= list2.val)
                {
                    tail.next = list1;
                    list1 = list1.next;
                }
                else
                {
                    tail.next = list2;
                    list2 = list2.next;
                }
                tail = tail.next;
            }

            while (list1 != null)
            {
                tail.next = list1;
                list1 = list1.next;
                tail = tail.next;
            }
            while (list2 != null)
            {
                tail.next = list2;
                list2 = list2.next;
                tail = tail.next;
            }

            tail.next = null;
            return resultHead;
        }
    }
}

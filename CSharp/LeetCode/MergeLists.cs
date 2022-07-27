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
            return this.MergeKListsRecursive(new ArraySegment<ListNode>(lists));
        }

        private ListNode MergeKListsRecursive(ArraySegment<ListNode> lists) {
            if (lists.Count == 0)
            {
                return null;
            }
            else if (lists.Count == 1)
            {
                return lists[0];
            }
            else if (lists.Count == 2)
            {
                return this.MergeTwoLists(lists[0], lists[1]);
            }
            else
            {
                int half = lists.Count / 2 + 1;
                int otherHalf = lists.Count - half;
                var left = new ArraySegment<ListNode>(lists.Array,lists.Offset,half);
                var right = new ArraySegment<ListNode>(lists.Array,lists.Offset + half, otherHalf);

                ListNode leftMerged = this.MergeKListsRecursive(left);
                ListNode rightMerged = this.MergeKListsRecursive(right);

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

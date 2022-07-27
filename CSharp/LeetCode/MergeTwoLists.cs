using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LeetCode
{
    public class MergeTwoListsSolution
    {
        /**
         * Definition for singly-linked list.
         * public class ListNode {
         *     public int val;
         *     public ListNode next;
         *     public ListNode(int val=0, ListNode next=null) {
         *         this.val = val;
         *         this.next = next;
         *     }
         * }
         */
        public ListNode? MergeTwoLists(ListNode? list1, ListNode? list2) {
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

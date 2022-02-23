using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LeetCode
{
    public class AddTwoNumbersSolution
    {
 
        public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
        {
            return AddTwoNumbersRecursive(l1, l2, 0);
        
        }
        
        private ListNode AddTwoNumbersRecursive(ListNode l1, ListNode l2, int carry)
        {
            int sum = l1.val + l2.val + carry;
            int newVal = sum % 10;
            int newCarry = sum / 10;

            ListNode? newNext;
            if (l1.next == null && l2.next == null)
            {
                if (newCarry > 0)
                {
                    newNext = new ListNode(newCarry, null);
                }
                else
                {
                    newNext = null;
                }
            }
            else if (l1.next == null)
            {
                if (newCarry > 0)
                {
                    ListNode carryList = new ListNode(newCarry, null);
                    newNext = AddTwoNumbersRecursive(carryList, l2.next, 0);
                }
                else
                {
                    newNext = l2.next;
                }
            }
            else if (l2.next == null)
            {
                if (newCarry > 0)
                {
                    ListNode carryList = new ListNode(newCarry, null);
                    newNext = AddTwoNumbersRecursive(carryList, l1.next, 0);
                }
                else
                {
                    newNext = l1.next;
                }
            }
            else
            {
                newNext = AddTwoNumbersRecursive(l1.next, l2.next, newCarry);
            }

            return new ListNode(newVal, newNext);        
        }

        // For testing
        public ListNode ConvertToList(int i)
        {
            if (i < 0)
            {
                throw new ArgumentException("Number must be larger than zero");
            }

            if (i < 10)
            {
                return new ListNode(i, null);
            }
            else
            {
                int digit = i % 10;
                int rest = i / 10;
                ListNode list = ConvertToList(rest);
                return new ListNode(digit, list);
            }
        }

        public int ConvertToInt(ListNode list)
        {
            if (list.val < 0)
            {
                throw new ArgumentException("List is invalid, digit is smaller than zero");
            }
            else if (list.val >= 10)
            {
                throw new ArgumentException("List is invalid, digit is larger than 10");
            }

            int rest;
            if (list.next == null)
            {
                rest = 0;
            }
            else
            {
                rest = 10 * ConvertToInt(list.next);
            }
            int number = list.val + rest;

            return number;
        }
    }
    public class ListNode {
        public int val;
        public ListNode? next;
        public ListNode(int val=0, ListNode? next =null) {
            this.val = val;
            this.next = next;
        }
    }
}

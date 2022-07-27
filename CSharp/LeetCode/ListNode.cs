using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace LeetCode
{
    public class ListNode
    {
        public int val;
        public ListNode next;
        public ListNode(int val = 0, ListNode next = null)
        {
            this.val = val;
            this.next = next;
        }

        public override string ToString()
        {
            StringBuilder output = new StringBuilder();
            output.Append('(');
            output.Append(this.val.ToString());
            ListNode node = this.next;
            while (node != null)
            {
                output.AppendFormat(",{0}", node.val);
                node = node.next;
            }
            output.Append(')');
            return output.ToString();
        }

        public static ListNode FromArray(int[] array)
        {
            if (array.Length == 0)
            {
                return null;
            }
            ListNode result = new ListNode(array[0],null);
            ListNode tail = result;
            for (int i = 1; i < array.Length; i++)
            {
                tail.next = new ListNode(array[i],null);
                tail = tail.next;
            }
            return result;
        }
    }
}

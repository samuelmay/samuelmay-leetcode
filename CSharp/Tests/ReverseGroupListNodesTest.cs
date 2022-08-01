using NUnit.Framework;
using LeetCode;

namespace Tests
{
    public class ReverseGroupListNodesTest
    {
        private ReverseNodesKGroup solution;
        public ReverseGroupListNodesTest()
        {
            solution = new ReverseNodesKGroup();
        }

        [Test]
        public void LeetcodeExample1()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1, 2, 3, 4, 5 });
            ListNode result = solution.ReverseKGroup(list1,2);
            string output = result.ToString();
            Assert.AreEqual("(2,1,4,3,5)",output);

        }
        [Test]
        public void LeetcodeExample2()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1, 2, 3, 4, 5 });
            ListNode result = solution.ReverseKGroup(list1,3);
            string output = result.ToString();
            Assert.AreEqual("(3,2,1,4,5)",output);

        }
        [Test]
        public void DivisibleLengthExample()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1,2,3,4,5,6,7,8,9 });
            ListNode result = solution.ReverseKGroup(list1,3);
            string output = result.ToString();
            Assert.AreEqual("(3,2,1,6,5,4,9,8,7)",output);
        }

        [Test]
        public void EntireLengthExample()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1,2,3,4,5,6 });
            ListNode result = solution.ReverseKGroup(list1,6);
            string output = result.ToString();
            Assert.AreEqual("(6,5,4,3,2,1)",output);
        }
        [Test]
        public void NullInput()
        {
            ListNode? result = solution.ReverseKGroup(null,5);
            Assert.IsNull(result);

            ListNode oneElement = ListNode.FromArray(new int[] { 1 } );
            result = solution.ReverseKGroup(oneElement,1);
            Assert.AreEqual(1, result.val);
            Assert.IsNull(result.next);

            result = solution.ReverseKGroup(oneElement,77);
            Assert.AreEqual(1, result.val);
            Assert.IsNull(result.next);

        }
        
    }
}

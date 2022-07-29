using NUnit.Framework;
using LeetCode;

namespace Tests
{
    public class SwapListNodesTests
    {
        private SwapAlternateListNodesSolution solution;
        public SwapListNodesTests()
        {
            solution = new SwapAlternateListNodesSolution();
        }

        [Test]
        public void LeetcodeExample()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1, 2, 3, 4 });
            ListNode result = solution.SwapPairs(list1);
            string output = result.ToString();
            Assert.AreEqual(output, "(2,1,4,3)");

        }

        [Test]
        public void OddLengthExample()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 5,6,7,8,9 });
            ListNode result = solution.SwapPairs(list1);
            string output = result.ToString();
            Assert.AreEqual(output, "(6,5,8,7,9)");
        }

        [Test]
        public void NullInput()
        {
            ListNode? result = solution.SwapPairs(null);
            Assert.IsNull(result);

            ListNode oneElement = ListNode.FromArray(new int[] { 1 } );
            result = solution.SwapPairs(oneElement);
            Assert.AreEqual(result.val, 1);
            Assert.IsNull(result.next);
        }
        
    }
}

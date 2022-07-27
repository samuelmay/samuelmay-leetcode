using NUnit.Framework;
using LeetCode;

namespace Tests
{
    public class MergeListsTests
    {
        private MergeTwoListsSolution solution;
        public MergeListsTests()
        {
            solution = new MergeTwoListsSolution();
        }

        [Test]
        public void ListCreateAndPrint()
        {
            int[] array1 = { 1, 2, 44 };
            ListNode? test = ListNode.FromArray(array1);
            Assert.IsNotNull(test);
            string output = test.ToString();
            Assert.AreEqual(output, "(1,2,44)");

            int[] array2 = { };
            Assert.IsNull(ListNode.FromArray(array2));

            int[] array3 = { 55 };
            test = ListNode.FromArray(array3);
            Assert.IsNotNull(test);
            output = test.ToString();
            Assert.AreEqual(output, "(55)");

        }

        [Test]
        public void BasicMerge()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1, 3, 5, 7});
            Assert.IsNotNull(list1);
            ListNode list2 = ListNode.FromArray(new int[] { 2, 4, 6, 8});
            Assert.IsNotNull(list2);
            ListNode result = solution.MergeTwoLists(list1, list2);
            Assert.AreEqual(result.val, 1);
            string output = result.ToString();
            Assert.AreEqual(output, "(1,2,3,4,5,6,7,8)");
        }

        [Test]
        public void LeetcodeExample()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1, 2, 4});
            Assert.IsNotNull(list1);
            ListNode list2 = ListNode.FromArray(new int[] { 1, 3, 4});
            Assert.IsNotNull(list2);
            ListNode result = solution.MergeTwoLists(list1, list2);
            string output = result.ToString();
            Assert.AreEqual(output, "(1,1,2,3,4,4)");

        }

        [Test]
        public void UnevenLength()
        {
            ListNode list1 = ListNode.FromArray(new int[] { 1, 1, 100, 100, 100 });
            Assert.IsNotNull(list1);
            ListNode list2 = ListNode.FromArray(new int[] { 50 });
            Assert.IsNotNull(list2);
            ListNode result = solution.MergeTwoLists(list1, list2);
            string output = result.ToString();
            Assert.AreEqual(output, "(1,1,50,100,100,100)");

        }
        [Test]
        public void NullInput()
        {
            ListNode? result = solution.MergeTwoLists(null, null);
            Assert.IsNull(result);

            ListNode oneElement = ListNode.FromArray(new int[] { 1 } );
            result = solution.MergeTwoLists(oneElement, null);
            Assert.AreEqual(result.val, 1);
            Assert.IsNull(result.next);

            result = solution.MergeTwoLists(null, oneElement);
            Assert.AreEqual(result.val, 1);
            Assert.IsNull(result.next);

        }
        
    }
}

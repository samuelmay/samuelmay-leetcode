using NUnit.Framework;
using LeetCode;
using System.Linq;

namespace Tests
{
    public class AddTwoNumbersTest
    {
        private AddTwoNumbersSolution solution;
        public AddTwoNumbersTest()
        {
            solution = new AddTwoNumbersSolution();
        }

        [Test]
        public void Test1()
        {
            ListNode a = solution.ConvertToList(123456);
            int b = solution.ConvertToInt(a);
            Assert.AreEqual(123456, b);
            
            a = solution.ConvertToList(1000);
            b = solution.ConvertToInt(a);
            Assert.AreEqual(1000, b);

            a = solution.ConvertToList(1050);
            b = solution.ConvertToInt(a);
            Assert.AreEqual(1050, b);

            a = solution.ConvertToList(9999);
            b = solution.ConvertToInt(a);
            Assert.AreEqual(9999, b);
        }
        [Test]
        public void Test2()
        {
            ListNode a = solution.ConvertToList(1000);
            ListNode b = solution.ConvertToList(1);
            ListNode c = solution.AddTwoNumbers(a, b);
            int cInt = solution.ConvertToInt(c);

            Assert.AreEqual(1001, cInt);
        }
        [Test]
        public void Test3()
        {
            ListNode a = solution.ConvertToList(342);
            ListNode b = solution.ConvertToList(465);
            ListNode c = solution.AddTwoNumbers(a, b);
            int cInt = solution.ConvertToInt(c);

            Assert.AreEqual(807, cInt);
        }
        [Test]
        public void Test4()
        {
            ListNode a = solution.ConvertToList(0);
            ListNode b = solution.ConvertToList(0);
            ListNode c = solution.AddTwoNumbers(a, b);
            int cInt = solution.ConvertToInt(c);

            Assert.AreEqual(0, cInt);
        }
        [Test]
        public void Test5()
        {
            ListNode a = solution.ConvertToList(9999999);
            ListNode b = solution.ConvertToList(9999);
            ListNode c = solution.AddTwoNumbers(a, b);
            int cInt = solution.ConvertToInt(c);

            Assert.AreEqual(10009998, cInt);
        }
    }
}

using NUnit.Framework;
using LeetCode;
using System.Linq;

namespace Tests
{
    public class TwoSumTests
    {
        private TwoSumSolver solution;
        public TwoSumTests()
        {
            solution = new TwoSumSolver();
        }

        [SetUp]
        public void Setup()
        {
        }

        [Test]
        public void Test1()
        {
            int[] example = { 2,7,11,15};
            int target = 9;

            int[] result = solution.TwoSum(example, target);

            int[] expectedResult = { 0, 1 };

            Assert.IsTrue(result.SequenceEqual(expectedResult));
        }
        
        [Test]
        public void Test2()
        {
            int[] example = {3,4,2};
            int target = 6;

            int[] result = solution.TwoSum(example, target);

            int[] expectedResult = { 1, 2 };

            Assert.IsTrue(result.SequenceEqual(expectedResult));
        }

        [Test]
        public void Test3()
        {
            int[] example = { 3, 3 };
            int target = 6;

            int[] result = solution.TwoSum(example, target);

            int[] expectedResult = { 0, 1 };

            Assert.IsTrue(result.SequenceEqual(expectedResult));
        }

        [Test]
        public void Test4()
        {
            int[] example = { 0,4,3,0};
            int target = 0;

            int[] result = solution.TwoSum(example, target);

            int[] expectedResult = { 0, 3 };

            Assert.IsTrue(result.SequenceEqual(expectedResult));

        }
        
        [Test]
        public void Test5()
        {
            int[] example = {-3,4,3,90};
            int target = 0;

            int[] result = solution.TwoSum(example, target);

            int[] expectedResult = { 0, 2 };

            Assert.IsTrue(result.SequenceEqual(expectedResult));

        }
    }
}
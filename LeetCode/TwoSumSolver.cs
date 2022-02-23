namespace LeetCode
{
    public class TwoSumSolver
    {

        public int[] TwoSum(int[] nums, int target) {

            // sort array
            int[] sortedNums = new int[nums.Length];
            nums.CopyTo(sortedNums,0);
            Array.Sort(sortedNums);

            // There might be some optimization still to be done here.
            // Both the inner and outer loops could probably
            // be terminated early based on some other tests.

            // iterate backwards from highest number
            for (int i = (sortedNums.Length - 1); i > 0; i--)
            {
                // in inner loop, iterate forwards from lowest number, calcuate sums until target is equaled
                // or exceeded
                for (int j = 0; j < i; j++)
                {
                    int attempt = sortedNums[j] + sortedNums[i];
                    if (attempt == target)
                    {
                        return getResult(sortedNums[j], sortedNums[i], nums);
                    }
                }
            }
            
            return new int[] { -1,-1};
        }

        private int[] getResult(int firstElement, int secondElement, int[] nums)
        {
            int[] result = new int[2];
            // lookup indices and return
            int indexA = indexOf(nums,firstElement,0);
            int indexB = indexOf(nums,secondElement,0);
            // Special case to handle duplicates
            if (indexA == indexB)
            {
                // try again, but starting after the first
                // index we found, which will skip the duplicate
                indexB = indexOf(nums,secondElement,indexB+1);
            }

            // It's unsure if the result needs to be
            // in sorted order, but we'll do it anyway 
            if (indexA < indexB)
            {
                result[0] = indexA;
                result[1] = indexB;
            }
            else
            {
                result[0] = indexB;
                result[1] = indexA;
            }

            return result;

        }

        private int indexOf(int[] array, int element, int startAt)
        {
            if (startAt >= array.Length || startAt < 0)
            {
                return -1;
            }
            for (int i = startAt; i < array.Length; i++)
            {
                if (array[i] == element)
                {
                    return i;
                }
            }
            return -1;
        }
    }
}
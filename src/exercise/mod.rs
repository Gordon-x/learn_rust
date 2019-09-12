///
/// ## 两数只和
///
/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
///
/// - 示例:
///
///```text
///给定 nums = [2, 7, 11, 15], target = 9
///
///因为 nums[0] + nums[1] = 2 + 7 = 9
///所以返回 [0, 1]
///```
///> 来源：力扣（LeetCode）
///> 链接：https://leetcode-cn.com/problems/two-sum
///> 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub mod two_sum;

///
/// ## 无重复字符的最长子串
///
/// 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
///
///- 示例 1:
///
///```text
///输入: "abcabcbb"
///输出: 3
///解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
/// ```
///
///- 示例 2:
///
/// ```text
///输入: "bbbbb"
///输出: 1
///解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
/// ```
///
///- 示例 3:
///
/// ```text
///输入: "pwwkew"
///输出: 3
///解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
///     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
/// ```
///
///> 来源：力扣（LeetCode）
///> 链接：https://leetcode-cn.com/problems/longest-substring-without-repeating-characters
///> 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub mod length_of_longest_substring;

///
/// ## 寻找两个有序数组的中位数
///
/// 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
///
///请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
///
///你可以假设 nums1 和 nums2 不会同时为空。
///
///- 示例 1:
///
/// ```text
///nums1 = [1, 3]
///nums2 = [2]
///
///则中位数是 2.0
/// ```
///
///- 示例 2:
///
/// ```text
///nums1 = [1, 2]
///nums2 = [3, 4]
///
///则中位数是 (2 + 3)/2 = 2.5
/// ```
///
///> 来源：力扣（LeetCode）
///> 链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
///> 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub mod find_median;

///
/// ## 最长回文子串
///
/// 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
///
/// - 示例 1：
///
/// ```text
/// 输入: "babad"
/// 输出: "bab"
/// 注意: "aba" 也是一个有效答案。
/// ```
///
/// - 示例 2：
///
/// ```text
/// 输入: "cbbd"
/// 输出: "bb"
/// ```
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/longest-palindromic-substring
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub mod longest_palindrome;

///
/// ## Z 字形变换
///
/// 将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
///
/// 比如输入字符串为 "`LEETCODEISHIRING`" 行数为 3 时，排列如下：
///
/// ```text
/// L   C   I   R
/// E T O E S I I G
/// E   D   H   N
/// ```
///
/// 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："`LCIRETOESIIGEDHN`"。
///
/// 请你实现这个将字符串进行指定行数变换的函数：
///
/// ```text
/// string convert(string s, int numRows);
/// ```
///
/// - 示例 1:
///
/// ```text
/// 输入: s = "LEETCODEISHIRING", numRows = 3
/// 输出: "LCIRETOESIIGEDHN"
/// ```
///
/// - 示例 2:
///
/// ```text
/// 输入: s = "LEETCODEISHIRING", numRows = 4
/// 输出: "LDREOEIIECIHNTSG"
/// 解释:
///
/// L     D     R
/// E   O E   I I
/// E C   I H   N
/// T     S     G
/// ```
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/zigzag-conversion
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub mod convert;
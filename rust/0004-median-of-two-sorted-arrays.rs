impl Solution {
    // Time O(log(m+n)) - Space O(log(m+n))
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // If one of the arrays is empty, return the median from the non-empty one.
        if nums1.is_empty() || nums2.is_empty() {
            return Self::median_of_sorted_array(if nums1.is_empty() { &nums2 } else { &nums1 })
                as f64;
        }
        let (a, b) = (&nums1, &nums2);
        let (m, n, l) = (a.len(), b.len(), a.len() + b.len());
        let (la, lb, ra, rb) = (0, 0, m - 1, n - 1);
        if l % 2 == 0 {
            (Self::kth_smallest(a, b, la, ra, lb, rb, l / 2)
                + Self::kth_smallest(a, b, la, ra, lb, rb, l / 2 - 1))
                / 2.0
        } else {
            Self::kth_smallest(a, b, la, ra, lb, rb, l / 2)
        }
    }

    /**
     * Return the k-th smallest element given two sorted arrays.
     */
    pub fn kth_smallest(
        a: &Vec<i32>,
        b: &Vec<i32>,
        la: usize,
        ra: usize,
        lb: usize,
        rb: usize,
        k: usize,
    ) -> f64 {
        if la > ra {
            return b[k - la] as f64;
        }
        if lb > rb {
            return a[k - lb] as f64;
        }
        let (ma, mb) = ((la + ra) / 2, (lb + rb) / 2);
        if ma + mb < k {
            if a[ma] > b[mb] {
                return Self::kth_smallest(a, b, la, ra, mb + 1, rb, k);
            }
            return Self::kth_smallest(a, b, ma + 1, ra, lb, rb, k);
        }
        if a[ma] > b[mb] {
            // Prevent subtracting below 0.
            if ma == 0 {
                return b[k - la] as f64;
            }
            return Self::kth_smallest(a, b, la, ma - 1, lb, rb, k);
        }
        // Prevent subtracting below 0
        if mb == 0 {
            return a[k - lb] as f64;
        }
        return Self::kth_smallest(a, b, la, ra, lb, mb - 1, k);
    }

    /**
     * Return the median of one sorted array.
     */
    pub fn median_of_sorted_array(a: &Vec<i32>) -> f64 {
        let mid = a.len() / 2;
        if a.len() % 2 == 0 {
            (a[mid] + a[mid - 1]) as f64 / 2.0
        } else {
            a[mid] as f64
        }
    }
}

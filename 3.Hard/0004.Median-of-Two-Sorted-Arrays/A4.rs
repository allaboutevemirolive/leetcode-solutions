impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let mut i1 = 0;
        let mut i2 = 0;
        let limit = ((l1 + l2) / 2) + 1;
        let mut sol = vec![0; limit];

        for i in 0..limit {
            let okay1 = i1 < l1;
            let okay2 = i2 < l2;
            if okay1 && okay2 {
                if nums1[i1] < nums2[i2] {
                    sol[i] = nums1[i1];
                    i1 += 1;
                } else {
                    sol[i] = nums2[i2];
                    i2 += 1;
                }
            } else if okay1 {
                sol[i] = nums1[i1];
                i1 += 1;
            } else if okay2 {
                sol[i] = nums2[i2];
                i2 += 1;
            }
        }

        if (l1 + l2) % 2 == 0 {
            return (sol[limit - 2] + sol[limit - 1]) as f64 / 2_f64;
        } else {
            return sol[limit - 1] as f64;
        }
    }
}
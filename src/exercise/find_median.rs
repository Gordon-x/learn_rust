
pub fn find_median_sorted_arrays(v1: Vec<i32>, v2: Vec<i32>) -> f64 {
    let mut stack:Vec<i32> = vec![];
    let total_len = v1.len() + v2.len();
    if total_len == 0 {
        panic!("列表空")
    }

    let mut cur1 = 0;
    let mut cur2 = 0;

    let median_pos:(usize, usize);

    if total_len & 1 == 0 {
        median_pos = (total_len/2 - 1, total_len / 2);
    } else if total_len > 1 {
        let m = (total_len - 1)/2;
        median_pos = (m, m);
    } else {
        median_pos = (total_len - 1, total_len - 1);
    }

    loop {
        let ele1: Option<&i32> = v1.get(cur1);
        let ele2: Option<&i32> = v2.get(cur2);

        if ele1.is_some() && ele2.is_some() {
            if ele1.unwrap() > ele2.unwrap() {
                stack.push(*ele2.unwrap());
                cur2 += 1;
            } else if ele1.unwrap() < ele2.unwrap() {
                stack.push(*ele1.unwrap());
                cur1 += 1;
            } else {
                stack.push(*ele1.unwrap());
                stack.push(*ele2.unwrap());
                cur1 += 1;
                cur2 += 1;
            }
        } else if ele2.is_some() {
            stack.push(*ele2.unwrap());
            cur2 += 1;
        } else if ele1.is_some() {
            stack.push(*ele1.unwrap());
            cur1 += 1;
        }

        if stack.len() - 1 >= median_pos.1 {
            break;
        }
    }

    ((stack[median_pos.0] + stack[median_pos.1]) as f64) / 2.0
}
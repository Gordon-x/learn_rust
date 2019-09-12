
/*pub fn longest_palindrome(s: String) -> String {

}*/


/*fn longest2(s:String) ->String {
    let seq: Vec<char> = s.chars().collect();
    let len = seq.len();
    if len < 1 {return s}
    let (mut idx, mut curr_len, mut curr_start, mut curr_end) = (0, 0, 0, 0);
    while idx < len {
        let (mut i, mut j) = (idx, idx);
        let ch = seq[idx];
        // handle same char
        while i > 0 && seq[i - 1] == ch { i -= 1 };
        while j < len - 1 && seq[j + 1] == ch { j += 1 };
        idx = j + 1;
        while i > 0 && j < len - 1 && seq[i - 1] == seq[j + 1] {
            i -= 1; j +=1;
        }
        let max_len = j - i + 1;
        if max_len > curr_len {
            curr_len = max_len; curr_start = i; curr_end = j;
        }
        if max_len >= len - 1 {
            break;
        }
    }

    s[curr_start..curr_end+1].to_owned()
}*/

fn long_new(s: String) ->String {
    let length = s.len();
    if length < 2 {
        return s;
    }
    let chars:Vec<char> = s.chars().collect();
    if length == 2 {
        if chars[0] == chars[1] {
            return s;
        }
        return s[0..1].to_owned();
    }
    let (mut start, mut end, mut cur_start, mut cur_end) = (0, length - 1, 0, 0);


    while start < end {
        if cur_end - cur_start < end - start && chars[start] == chars[end] {
            let (mut t_start, mut t_end, mut finish) = (start, end, true);

            while t_end >= t_start {
                if chars[t_end] != chars[t_start] {
                    finish = false;
                    break;
                }
                t_end -= 1;
                t_start += 1;
            }
            if finish {
                cur_start = start;
                cur_end = end;
            }
        }

        if end - start <= 1 {
            start += 1;
            end = length - 1;
        } else {
            end -= 1;
        }
    }

    s[cur_start..=cur_end].to_owned()
}

/*
fn longest(s:String) -> String {
    let l = s.len();
    if l <= 1 {
        return s;
    }

    let mut start:usize  = 0;
    let mut end:usize = l - 1;
    let (mut cur_start, mut cur_end) = (0, 0);
    let chars_vec = s.chars().collect::<Vec<char>>();

    if start + 1 == end {
        if chars_vec[start] == chars_vec[end] {
            return s;
        }
        return s[0..1].to_owned();
    }

    while end > start {
        if chars_vec[start] == chars_vec[end] && end - start > cur_end - cur_start  {
            let mut tmp_start = start;
            let mut tmp_end = end;

            while tmp_end >= tmp_start {
                if chars_vec[tmp_start] != chars_vec[tmp_end] {
                    break;
                }

                if tmp_start == tmp_end || tmp_start + 1 == tmp_end {
                    cur_start = start;
                    cur_end = end;
                    break;
                }
                tmp_end -= 1;
                tmp_start += 1;
            }
        }

        if start == end || start + 1 == end {
            start += 1;
            end = l - 1;
        } else {
            end -= 1;
        }
    }
    s[cur_start..=cur_end].to_owned()
}*/
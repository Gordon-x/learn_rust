
pub fn long_new(s: String) ->String {
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
use std::borrow::Cow;

pub fn char_at(index: i64) -> u8 {
    assert!(index >= 0);
    let index = index as u128;

    let mut l = 0;
    let mut r = i64::MAX;

    while l + 1 < r {
        let mid = l + (r - l) / 2;
        let v = fizz_buzz_length(mid);
        if v <= index {
            l = mid;
        } else {
            r = mid;
        }
    }

    let base = fizz_buzz_length(r - 1);
    let line_index = (index - base) as usize;
    let line = line_at(r);

    line[line_index]
}

pub fn line_at(n: i64) -> Cow<'static, [u8]> {
    if n % 15 == 0 {
        Cow::from(b"FizzBuzz\n" as &'static [u8])
    } else if n % 3 == 0 {
        Cow::from(b"Fizz\n" as &'static [u8])
    } else if n % 5 == 0 {
        Cow::from(b"Buzz\n" as &'static [u8])
    } else {
        let s = format!("{}\n", n);
        let v = Vec::from(s.as_bytes());
        Cow::from(v)
    }
}

const NEWLINE_LEN: u128 = 1;

pub fn fizz_buzz_length(n: i64) -> u128 {
    assert!(n >= 0);
    let n = n as u128;

    let zero = 0u128;
    let mut num_len_sum = zero;
    let mut num_count = zero;
    let mut fizz_count = zero;
    let mut buzz_count = zero;
    let mut fizz_buzz_count = zero;
    let max = (i64::MAX / 10 + 1) as u128;
    let mut i = 10u128;
    let mut digits = 1;
    while i <= n {
        let fizz_buzz_count0 = (i - 1) / 15 - fizz_buzz_count;
        let fizz_count0 = (i - 1) / 3 - fizz_buzz_count0 - fizz_count - fizz_buzz_count;
        let buzz_count0 = (i - 1) / 5 - fizz_buzz_count0 - buzz_count - fizz_buzz_count;
        fizz_buzz_count += fizz_buzz_count0;
        fizz_count += fizz_count0;
        buzz_count += buzz_count0;

        let num_count0 = i - fizz_count - buzz_count - fizz_buzz_count - num_count;
        num_count += num_count0;
        num_len_sum += (digits + NEWLINE_LEN) * num_count0;

        assert_eq!(num_count + fizz_count + buzz_count + fizz_buzz_count, i);

        digits += 1;
        if max <= i {
            break;
        }
        i *= 10;
    }

    let fizz_buzz_count0 = n / 15 - fizz_buzz_count;
    let fizz_count0 = n / 3 - fizz_buzz_count0 - fizz_count - fizz_buzz_count;
    let buzz_count0 = n / 5 - fizz_buzz_count0 - buzz_count - fizz_buzz_count;
    fizz_buzz_count += fizz_buzz_count0;
    fizz_count += fizz_count0;
    buzz_count += buzz_count0;

    let num_count0 = n + 1 - fizz_count - buzz_count - fizz_buzz_count - num_count;
    num_count += num_count0;

    num_len_sum += (digits + NEWLINE_LEN) * num_count0;

    assert_eq!(fizz_count + buzz_count + fizz_buzz_count + num_count, n + 1);

    // 1 + NEWLINE_LEN for "0\n"
    num_len_sum
        + fizz_count * (4 + NEWLINE_LEN)
        + buzz_count * (4 + NEWLINE_LEN)
        + fizz_buzz_count * (8 + NEWLINE_LEN)
        - (1 + NEWLINE_LEN)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(fizz_buzz_length(0), 0);
        assert_eq!(fizz_buzz_length(1), 2);
        assert_eq!(fizz_buzz_length(2), 4);
        assert_eq!(fizz_buzz_length(3), 9); // 3
        assert_eq!(fizz_buzz_length(4), 11);
        assert_eq!(fizz_buzz_length(5), 16); // 5
        assert_eq!(fizz_buzz_length(6), 21); // 3
        assert_eq!(fizz_buzz_length(7), 23);
        assert_eq!(fizz_buzz_length(8), 25);
        assert_eq!(fizz_buzz_length(9), 30); // 3
        assert_eq!(fizz_buzz_length(10), 35); // 5
        assert_eq!(fizz_buzz_length(11), 38);
        assert_eq!(fizz_buzz_length(12), 43); // 3
        assert_eq!(fizz_buzz_length(13), 46);
        assert_eq!(fizz_buzz_length(14), 49);
        assert_eq!(fizz_buzz_length(15), 58); // 35
        assert_eq!(fizz_buzz_length(16), 61);
    }
    #[test]
    fn test_length_many() {
        let mut l = 0u128;
        for i in 1..=11111 {
            if i % 3 == 0 && i % 5 == 0 {
                l += 8 + NEWLINE_LEN;
            } else if i % 3 == 0 {
                l += 4 + NEWLINE_LEN;
            } else if i % 5 == 0 {
                l += 4 + NEWLINE_LEN;
            } else {
                l += i.to_string().len() as u128 + NEWLINE_LEN;
            }
            let actual = fizz_buzz_length(i as i64);
            assert_eq!(
                actual, l,
                "fizz_buzz_length({})={}(expected {})",
                i, actual, l
            );
        }
    }
    #[test]
    fn test_char_at() {
        assert_eq!(char_at(0), b'1');
        assert_eq!(char_at(1), b'\n');

        let expected = b"1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16";
        let mut actual = Vec::new();
        for i in 0..expected.len() {
            actual.push(char_at(i as i64));
        }
        assert_eq!(expected, actual.as_slice());
    }
}

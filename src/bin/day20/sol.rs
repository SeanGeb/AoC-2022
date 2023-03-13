use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TaggedI16 {
    val: i16,
    tag: bool,
}

pub fn perform_mix(v: &[i16]) -> Vec<i16> {
    // Tag each item in the list with a flag indicating if it's been processed
    // yet.
    let mut v: Vec<TaggedI16> = v
        .iter()
        .map(|val| TaggedI16 {
            val: *val,
            tag: false,
        })
        .collect();

    // Loop over the indexes in the list, but every time we insert ahead of this
    // position, we must not advance the loop.
    let mut i = 0;
    while let Some(val) = v.get(i) {
        let val = val.to_owned();
        if val.tag {
            i += 1;
            continue;
        }

        let mut val = v.remove(i);
        assert!(!val.tag);
        val.tag = true;
        let mut new_idx: usize = (i as i16 + val.val)
            .rem_euclid(v.len().try_into().unwrap())
            .try_into()
            .unwrap();

        if val.val < 0 && new_idx == 0 {
            new_idx = v.len();
        }

        v.insert(new_idx, val);
    }

    v.into_iter()
        .inspect(|val| assert!(val.tag))
        .map(|val| val.val)
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaggedI64 {
    pub val: i64,
    pub original_idx: u16,
}

pub fn perform_mix_part2(v: &mut Vec<TaggedI64>) {
    for original_idx in 0..v.len() {
        let idx = v
            .iter()
            .position(|v| v.original_idx as usize == original_idx)
            .unwrap();

        let val = v.remove(idx);
        let mut new_idx: usize = (idx as i64 + val.val)
            .rem_euclid(v.len().try_into().unwrap())
            .try_into()
            .unwrap();

        if val.val < 0 && new_idx == 0 {
            new_idx = v.len();
        }

        v.insert(new_idx, val);
    }
}

pub fn score_i16(res: &Vec<i16>) -> [i16; 3] {
    let idx_0 = res.iter().position(|&v| v == 0).unwrap();
    assert_eq!(res[idx_0], 0);
    [
        res[(idx_0 + 1000) % res.len()],
        res[(idx_0 + 2000) % res.len()],
        res[(idx_0 + 3000) % res.len()],
    ]
}

pub fn score_i64(res: &Vec<TaggedI64>) -> [i64; 3] {
    let idx_0 = res.iter().position(|&v| v.val == 0).unwrap();
    assert_eq!(res[idx_0].val, 0);
    [
        res[(idx_0 + 1000) % res.len()].val,
        res[(idx_0 + 2000) % res.len()].val,
        res[(idx_0 + 3000) % res.len()].val,
    ]
}

pub fn mix_and_score(v: &[i16]) -> [i16; 3] {
    let res = perform_mix(v);
    score_i16(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perform_mix_on_example() {
        let ex: Vec<i16> = vec![1, 2, -3, 3, -2, 0, 4];
        let ex = perform_mix(&ex);

        assert_eq!(ex, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn mix_and_score_on_example() {
        let ex: Vec<i16> = vec![1, 2, -3, 3, -2, 0, 4];
        let ex = mix_and_score(&ex);

        assert_eq!(ex, [4, -3, 2]);
    }
}

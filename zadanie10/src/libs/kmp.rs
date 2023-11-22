pub fn seek_patterns<T: PartialEq>(pattern: &[T], alphabet: &[T]) -> Vec<usize> {
    let m = pattern.len();
    let pi = build_pi(pattern);
    let mut q = 0;

    let mut matches: Vec<usize> = Vec::new();

    alphabet.iter().enumerate().for_each(|(i, &ref graphene)| {
        while q > 0 && pattern[q] != *graphene {
            q = pi[q - 1];
        }
        if pattern[q] == *graphene {
            q += 1;
        }
        if q == m {
            matches.push(i + 1 - m);
            q = pi[q - 1];
        }
    });

    matches
}

fn build_pi<T: PartialEq>(pattern: &[T]) -> Vec<usize> {
    let m = pattern.len();
    let mut k = 0;
    let mut pi: Vec<usize> = vec![0; m];

    for q in 1..m {
        while k > 0 && pattern[k] != pattern[q] {
            k = pi[k - 1];
        }
        if pattern[q] == pattern[k] {
            k += 1;
        }
        pi[q] = k;
    }

    pi
}
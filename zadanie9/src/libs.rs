use rand::Rng;

pub trait Wka {
    fn shuffle(&mut self);
}

impl<T> Wka for Vec<T>
where
    T: Clone,
{
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let end = self.len() - 1;
        for i in 0..=(end - 1) {
            let j: usize = rng.gen::<usize>() % (end + 1 - i) + i;
            self.swap(i, j)
        }
    }
}

pub fn gen_vec(n: u32) -> Vec<u32> {
    (1..=n).collect()
}

pub fn static_points(v: &[u32]) -> u32 {
    v.iter().enumerate().fold(0_u32, |acc, (i, value)| {
        if (i + 1) as u32 == *value {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn cycles(v: &[u32]) -> u32 {
    let mut cycles: u32 = 0;
    let mut visited: Vec<bool> = vec![false; v.len()];
    for i in 0..v.len() {
        if !visited[i] {
            cycles += 1;
            visited[i] = true;

            let mut temp: u32 = v[i] - 1;

            while !visited[temp as usize] {
                visited[temp as usize] = true;
                temp = v[temp as usize] - 1;
            }
        }
    }
    cycles
}

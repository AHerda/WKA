use rand::{Rng, thread_rng};

fn main() {
    let mut v = vec![1,2,3,4,5,6,7,8,9];
    v.shuffle();
    println!("{:?}", v);
}

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for Vec<T>
where T: Clone
{
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let end = self.len() - 1;
        for i in 0..=end {
            let j: usize = rng.gen::<usize>() % (end + 1 - i) + i;
            self.swap(i, j)
        }
    }
}
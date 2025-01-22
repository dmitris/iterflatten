use std::iter::Iterator;

pub fn flatten<I, J>(iter: I) -> Flatten<I, J>
where
    I: IntoIterator<Item = J>,
    J: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<I, J>
where
    I: IntoIterator<Item = J>,
    J: IntoIterator,
{
    outer: I::IntoIter,
    inner: Option<J::IntoIter>,
}

impl<I, J> Flatten<I, J>
where
    I: IntoIterator<Item = J>,
    J: IntoIterator,
{
    fn new(v: I) -> Self {
        Flatten {
            outer: v.into_iter(),
            inner: None,
        }
    }
}

impl<I, J> Iterator for Flatten<I, J>
where
    I: IntoIterator<Item = J>,
    J: IntoIterator,
{
    type Item = J::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // check if self.inner is None which happens at the first call
        // if self.inner is None, assign self.outer.next() to self.inner
        if self.inner.is_none() {
            self.inner = Some(self.outer.next()?.into_iter());
        }

        loop {
            match self.inner {
                None => panic!("inner is None, should never happen"),
                Some(ref mut inner) => match inner.next() {
                    Some(e) => return Some(e),
                    None => {
                        self.inner = Some(self.outer.next()?.into_iter());
                        continue;
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut flatit = flatten(&v);
        // println!("{}", flatten.count());
        assert_eq!(Some(&1), flatit.next());
        assert_eq!(Some(&2), flatit.next());
        assert_eq!(Some(&3), flatit.next());
        assert_eq!(Some(&4), flatit.next());
        assert_eq!(None, flatit.next());
        // for el in flatten {
        //     println!("{:?}", el);
        // }
        flatit = flatten(&v);
        let flat: Vec<&i32> = flatit.collect();
        assert_eq!(vec![&1, &2, &3, &4], flat);
    }
}

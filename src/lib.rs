use std::iter::Iterator;

pub fn flatten<'a, T>(v: &'a Vec<Vec<T>>) -> Flatten<'a, T>
where
    T: Copy,
{
    Flatten::new(v)
}

pub struct Flatten<'a, T> {
    outer: std::slice::Iter<'a, Vec<T>>,
    // add field inner with the type being the iterator of the inner vector
    inner: Option<std::slice::Iter<'a, T>>,
    // inner: Option<std::slice::Iter<Vec<i32>>>,
}

impl<'a, T> Flatten<'a, T>
where
    T: Copy,
{
    fn new(v: &'a Vec<Vec<T>>) -> Self {
        Flatten {
            outer: v.iter(),
            inner: None,
        }
    }
}

impl<'a, T> Iterator for Flatten<'a, T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // check if self.inner is None which happens at the first call
        // if self.inner is None, assign self.outer.next() to self.inner
        if self.inner.is_none() {
            self.inner = Some(self.outer.next()?.iter());
        }

        loop {
            match self.inner {
                None => panic!("inner is None, should never happen"),
                Some(ref mut inner) => match inner.next() {
                    Some(e) => return Some(*e),
                    None => {
                        self.inner = Some(self.outer.next()?.iter());
                        continue;
                    }
                },
            }
        }
    }
}

// pub fn flatten<I>(iter: I) -> Flatten<I>
// where
//     I: Iterator,
//     I::Item: IntoIterator,
//     <I as Iterator>::Item: Iterator,
// {
//     Flatten::new(iter)
// }

// pub struct Flatten<I>
// where
//     I: Iterator,
//     I::Item: IntoIterator,
// {
//     outer: I,
//     inner: Option<I::Item>,
// }

// impl<I> Flatten<I>
// where
//     I: Iterator,
//     I::Item: Iterator,
// {
//     fn new(iter: I) -> Self {
//         Flatten {
//             outer: iter,
//             inner: None,
//         }
//     }
// }

// impl<I> Iterator for Flatten<I>
// where
//     I: Iterator,
//     I::Item: Iterator,
// {
//     type Item = <I::Item as Iterator>::Item;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.inner.is_none() {
//             self.inner = self.outer.next();
//             if self.inner.is_none() {
//                 return None;
//             }
//         }
//         None
//         // // check if self.inner is None with if clause
//         // // if self.inner is None, assign self.outer.next() to self.inner

//         // if self.inner.is_none() {
//         //     self.inner = self.outer.next()?;
//         // }
//         // let inner = self.inner.as_mut()?;
//         // match inner.next() {
//         //     Some(e) => Some(e),
//         //     None => {
//         //         self.inner = self.outer.next();
//         //         self.inner?.next()
//         //     }
//         // }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![vec![1_u32, 2, 3, 7], vec![4, 5, 6]];
        // let mut iter = v.iter();

        // assert_eq!(Some(&vec![1, 2, 3]), iter.next());
        // assert_eq!(Some(&vec![4, 5, 6]), iter.next());
        // assert_eq!(None, iter.next());

        let flatten = flatten(&v);
        // println!("{}", flatten.count());
        // assert_eq!(Some(1), flatten.next());
        // assert_eq!(Some(2), flatten.next());

        for el in flatten {
            println!("{:?}", el);
        }
        //     let flat: Vec<i32> = flatten.collect();
    }
}

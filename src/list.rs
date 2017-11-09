/* This file is part of rpds.
 *
 * rpds is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rpds is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with rpds.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::sync::Arc;
use std::fmt::Display;
use std::cmp::Ordering;
use std::hash::{Hasher, Hash};
use std::borrow::Borrow;
use std::iter::FromIterator;

/// A persistent list with structural sharing.  This data structure supports fast get head,
/// get tail, and cons.
///
/// # Complexity
///
/// Let *n* be the number of elements in the list.
///
/// ## Temporal complexity
///
/// | Operation         | Best case | Average | Worst case  |
/// |:----------------- | ---------:| -------:| -----------:|
/// | `new()`           |      Θ(1) |    Θ(1) |        Θ(1) |
/// | `cons()`          |      Θ(1) |    Θ(1) |        Θ(1) |
/// | `tail()`          |      Θ(1) |    Θ(1) |        Θ(1) |
/// | `clone()`         |      Θ(1) |    Θ(1) |        Θ(1) |
/// | `len()`           |      Θ(1) |    Θ(1) |        Θ(1) |
/// | iterator creation |      Θ(1) |    Θ(1) |        Θ(1) |
/// | iterator step     |      Θ(1) |    Θ(1) |        Θ(1) |
/// | iterator full     |      Θ(n) |    Θ(n) |        Θ(n) |
///
/// ## Space complexity
///
/// The space complexity is *Θ(n)*.
#[derive(Debug)]
pub struct List<T> {
    node: Arc<Node<T>>,
    length: usize,
}

#[derive(Debug)]
enum Node<T> {
    Cons(T, Arc<Node<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            node: Arc::new(Node::Nil),
            length: 0,
        }
    }

    pub fn head(&self) -> Option<&T> {
        match *self.node {
            Node::Cons(ref h, _) => Some(h),
            Node::Nil            => None,
        }
    }

    pub fn tail(&self) -> Option<List<T>> {
        match *self.node {
            Node::Cons(_, ref t) => Some(List { node: Arc::clone(t), length: self.length - 1 }),
            Node::Nil            => None,
        }
    }

    pub fn cons(&self, v: T) -> List<T> {
        List {
            node: Arc::new(Node::Cons(v, Arc::clone(&self.node))),
            length: self.length + 1,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<T> Default for List<T> {
    fn default() -> List<T> {
        List::new()
    }
}

impl<T: PartialEq<T>> PartialEq<List<T>> for List<T> {
    fn eq(&self, other: &List<T>) -> bool {
        self.length == other.length && self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for List<T> {}

impl<T: PartialOrd<T>> PartialOrd<List<T>> for List<T>  {
    fn partial_cmp(&self, other: &List<T>) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<T: Ord> Ord for List<T> {
    fn cmp(&self, other: &List<T>) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: Hash> Hash for List<T> {
    fn hash<H: Hasher>(&self, state: &mut H) -> () {
        for e in self {
            e.hash(state);
        }
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> List<T> {
        List {
            node: Arc::clone(&self.node),
            length: self.length,
        }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut first = true;

        fmt.write_str("[")?;

        for v in self.iter() {
            if !first {
                fmt.write_str(", ")?;
            }
            v.fmt(fmt)?;
            first = false;
        }

        fmt.write_str("]")
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> List<T> {
        let iter = into_iter.into_iter();
        let (min_size, max_size_hint) = iter.size_hint();
        let mut vec: Vec<T> = Vec::with_capacity(max_size_hint.unwrap_or(min_size));

        for e in iter {
            vec.push(e);
        }

        let mut list: List<T> = List::new();

        for e in vec.into_iter().rev() {
            list = list.cons(e);
        }

        list
    }
}

#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    next: &'a Node<T>,
    length: usize,
}

impl<'a, T> Iter<'a, T> {
    fn new(list: &List<T>) -> Iter<T> {
        Iter {
            next: list.node.borrow(),
            length: list.len(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match *self.next {
            Node::Cons(ref v, ref t) => {
                self.next = t;
                self.length -= 1;
                Some(v)
            },
            Node::Nil => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

#[cfg(test)]
mod test {
    use super::*;

    mod iter {
        use super::super::*;

        #[test]
        fn test_iter() -> () {
            let limit = 1024;
            let mut list = List::new();
            let mut left = limit;

            for i in 0..limit {
                list = list.cons(i);
            }

            for v in list.iter() {
                left -= 1;
                assert_eq!(*v, left);
            }

            assert!(left == 0);
        }


        #[test]
        fn test_iter_size_hint() -> () {
            let vector = List::new()
                .cons(2)
                .cons(1)
                .cons(0);
            let mut iterator = vector.iter();

            assert_eq!(iterator.size_hint(), (3, Some(3)));

            iterator.next();

            assert_eq!(iterator.size_hint(), (2, Some(2)));

            iterator.next();

            assert_eq!(iterator.size_hint(), (1, Some(1)));

            iterator.next();

            assert_eq!(iterator.size_hint(), (0, Some(0)));
        }

        #[test]
        fn test_into_iterator() -> () {
            let list = List::new()
                .cons(3)
                .cons(2)
                .cons(1)
                .cons(0);
            let mut expected = 0;
            let mut left = 4;

            for n in &list {
                left -= 1;

                assert!(left >= 0);
                assert_eq!(*n, expected);

                expected += 1;
            }

            assert!(left == 0);
        }
    }

    mod compile_time {
        use super::super::*;

        #[test]
        fn test_is_send() -> () {
            let _: Box<Send> = Box::new(List::<i32>::new());
        }

        #[test]
        fn test_is_sync() -> () {
            let _: Box<Sync> = Box::new(List::<i32>::new());
        }
    }

    #[test]
    fn test_new() -> () {
        let empty_list: List<i32> = List::new();

        match *empty_list.node {
            Node::Nil => (),
            _         => panic!("should be nil"),
        };

        assert_eq!(empty_list.len(), 0);
        assert!(empty_list.is_empty());
    }

    #[test]
    fn test_head() -> () {
        let empty_list: List<i32> = List::new();
        let singleton_list = List::new()
            .cons("hello");
        let list = List::new()
            .cons(3)
            .cons(2)
            .cons(1)
            .cons(0);

        assert_eq!(empty_list.head(), None);
        assert_eq!(singleton_list.head(), Some(&"hello"));
        assert_eq!(list.head(), Some(&0));
    }

    #[test]
    fn test_tail() -> () {
        let empty_list: List<i32> = List::new();
        let singleton_list = List::new()
            .cons("hello");
        let list = List::new()
            .cons(3)
            .cons(2)
            .cons(1)
            .cons(0);

        assert!(empty_list.tail().is_none());
        assert_eq!(singleton_list.tail().unwrap().head(), None);
        assert_eq!(list.tail().unwrap().head(), Some(&1));

        assert_eq!(list.len(), 4);
        assert_eq!(list.tail().unwrap().len(), 3);
    }

    #[test]
    fn test_from_iterator() -> () {
        let vec: Vec<u32> = vec![10, 11, 12, 13];
        let list: List<u32> = vec.iter().map(|v| *v).collect();

        assert!(vec.iter().eq(list.iter()));
    }

    #[test]
    fn test_default() -> () {
        let list: List<i32> = List::default();

        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_display() -> () {
        let empty_list: List<i32> = List::new();
        let singleton_list = List::new()
            .cons("hello");
        let list = List::new()
            .cons(3)
            .cons(2)
            .cons(1)
            .cons(0);

        assert_eq!(format!("{}", empty_list), "[]");
        assert_eq!(format!("{}", singleton_list), "[hello]");
        assert_eq!(format!("{}", list), "[0, 1, 2, 3]");
    }

    #[test]
    fn test_eq() -> () {
        let list_1 = List::new()
            .cons("a")
            .cons("a");
        let list_1_prime = List::new()
            .cons("a")
            .cons("a");
        let list_2 = List::new()
            .cons("b")
            .cons("a");

        assert_ne!(list_1, list_2);
        assert_eq!(list_1, list_1);
        assert_eq!(list_1, list_1_prime);
        assert_eq!(list_2, list_2);
    }

    #[test]
    fn test_partial_ord() -> () {
        let list_1 = List::new()
            .cons("a");
        let list_1_prime = List::new()
            .cons("a");
        let list_2 = List::new()
            .cons("b");
        let list_3 = List::new()
            .cons(0.0);
        let list_4 = List::new()
            .cons(::std::f32::NAN);

        assert!(list_1.partial_cmp(&list_1_prime) == Some(Ordering::Equal));
        assert!(list_1.partial_cmp(&list_2) == Some(Ordering::Less));
        assert!(list_2.partial_cmp(&list_1) == Some(Ordering::Greater));
        assert!(list_3.partial_cmp(&list_4) == None);
    }

    #[test]
    fn test_ord() -> () {
        let list_1 = List::new()
            .cons("a");
        let list_1_prime = List::new()
            .cons("a");
        let list_2 = List::new()
            .cons("b");

        assert!(list_1.cmp(&list_1_prime) == Ordering::Equal);
        assert!(list_1.cmp(&list_2) == Ordering::Less);
        assert!(list_2.cmp(&list_1) == Ordering::Greater);
    }

    fn hash<T: Hash>(list: &List<T>) -> u64 {
        let mut hasher = ::std::collections::hash_map::DefaultHasher::new();

        list.hash(&mut hasher);

        hasher.finish()
    }

    #[test]
    fn test_hash() -> () {
        let list_1 = List::new()
            .cons("a");
        let list_1_prime = List::new()
            .cons("a");
        let list_2 = List::new()
            .cons("b")
            .cons("a");

        assert_eq!(hash(&list_1), hash(&list_1));
        assert_eq!(hash(&list_1), hash(&list_1_prime));
        assert_ne!(hash(&list_1), hash(&list_2));
    }

    #[test]
    fn test_clone() -> () {
        let list = List::new()
            .cons("there")
            .cons("hello");
        let clone = list.clone();

        assert!(clone.iter().eq(list.iter()));
        assert_eq!(clone.len(), list.len());
    }
}

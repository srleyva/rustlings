// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

struct Wrapper<T> {
    value: T
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Wrapper<T> {
        Wrapper { value }
    }
}

struct Searcher<T> {
    list: Vec<T>,
    comp_fn: fn(T, T) -> bool
}

impl<T: PartialEq + Copy> Searcher<T> {
    fn new(value: T, comp_fn: fn(T,T) -> bool) -> Searcher<T> {
        Searcher {
            list: vec!(value),
            comp_fn: comp_fn
        }
    }

    fn add(&mut self, value: T) {
        self.list.push(value)
    }

    fn sort(&mut self) {
        for _ in 0..self.list.len() {
            for i in 0..self.list.len()-1 {
                if (self.comp_fn)(self.list[i], self.list[i+1]) {
                    self.list.swap(i+1, i)
                }
            }
        }
    }

    fn find(&self, value: T) -> Option<i32> {
        for (i, thing) in self.list.iter().enumerate() {
            if *thing == value {
                return Some(i as i32);
            }
        }
        None
    }
    
    fn len(&self) -> i32 {
        self.list.len() as i32
    }
}

fn max_sort<T: PartialOrd>(val_1: T, val_2: T) -> bool {
    val_1 > val_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }

    #[test]
    fn store_u32_in_searcher() {
        let mut searcher = Searcher::new(0, max_sort);
        searcher.add(12);
        searcher.add(15);
        assert_eq!(searcher.len(), 3);
    }

    #[test]
    fn search_u32_in_searcher() {
        let mut searcher = Searcher::new(0, max_sort);
        searcher.add(12);
        searcher.add(15);
        assert_eq!(searcher.find(12).unwrap(), 1);
    }


    #[test]
    fn sort_u32_in_searcher() {
        let mut searcher = Searcher::new(0, max_sort);
        searcher.add(15);
        searcher.add(1);
        searcher.add(11);
        searcher.add(9);
        searcher.add(6);
        searcher.sort();
        let expected = vec!(0, 1, 6, 9, 11, 15);
        assert_eq!(searcher.list, expected);
    }
}

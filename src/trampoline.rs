pub enum Trampoline<'a, A> {
    More(Box<FnOnce() -> A + 'a>),
    Done(A),
}

impl<'a, A> Trampoline<'a, A> {
    fn map<'f, F, B>(&'a self, func: F) -> Trampoline<'f, B>
    where
        F: 'f + Fn(&'f A) -> B,
        A: 'f,
    {
        self.flat_map(|a| Trampoline::More(Box::new(move || func(&a))))
    }

    fn flat_map<'f, F, B>(&'a self, func: F) -> Trampoline<'f, B>
    where
        F: FnOnce(&'f A) -> Trampoline<'f, B>,
        A: 'f,
    {
        unimplemented!()
    }

    fn value(self) -> A {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::VecDeque;
    fn fold_right<'b, A, B, Func>(
        fa: VecDeque<A>,
        acc: Trampoline<'b, B>,
        func: &Func,
    ) -> Trampoline<'b, B>
    where
        Func: Fn(Trampoline<'b, B>, A) -> Trampoline<'b, B>,
    {
        let mut tail = fa;
        if let Some(head) = tail.pop_front() {
            func(fold_right(tail, acc, func), head)
        } else {
            acc
        }
    }

    #[test]
    fn test_fold_right() {
        let mut vc = VecDeque::new();
        for i in 0..10000 {
            vc.push_back(i)
        }

        let result = fold_right(vc, Trampoline::Done(0), &|acc, i| {
            acc.map(move |acc| acc + i)
        });
        assert_eq!(result.value(), 123);
    }
}

macro_rules! iterator_map {
    ($Iter:ty {
        type Item = $Item:ty;
        |$this:ident, $item:ident| $closure:expr
    }) => {
        impl<'a> Iterator for $Iter {
            type Item = $Item;
            fn next(&mut self) -> Option<Self::Item> {
                let $this = self;
                let $item = $this.inner.next()?;
                Some($closure)
            }
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let $this = self;
                let $item = $this.inner.nth(n)?;
                Some($closure)
            }
            fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
            where
                F: FnMut(B, Self::Item) -> R,
                R: std::ops::Try<Output = B>,
            {
                let $this = self;
                $this.inner.try_fold(init, |init, $item| f(init, $closure))
            }
            fn fold<B, F>(mut self, init: B, mut f: F) -> B
            where F: FnMut(B, Self::Item) -> B {
                self.try_fold(init, |init, x| Ok::<_, !>(f(init, x))).unwrap()
            }
            fn last(mut self) -> Option<Self::Item> {
                self.next_back()
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len();
                (len, Some(len))
            }
        }
        impl<'a> ExactSizeIterator for $Iter {
            fn len(&self) -> usize {
                self.inner.len()
            }
        }
        impl<'a> std::iter::FusedIterator for $Iter {}

        impl<'a> DoubleEndedIterator for $Iter {
            fn next_back(&mut self) -> Option<Self::Item> {
                let $this = self;
                let $item = $this.inner.next_back()?;
                Some($closure)
            }
            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                let $this = self;
                let $item = $this.inner.nth_back(n)?;
                Some($closure)
            }
            fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R
            where
                F: FnMut(B, Self::Item) -> R,
                R: std::ops::Try<Output = B>,
            {
                let $this = self;
                $this.inner.try_rfold(init, |init, $item| f(init, $closure))
            }
            fn rfold<B, F>(mut self, init: B, mut f: F) -> B
            where F: FnMut(B, Self::Item) -> B {
                self.try_rfold(init, |init, x| Ok::<_, !>(f(init, x))).unwrap()
            }
        }
    };
}

pub(crate) use iterator_map;

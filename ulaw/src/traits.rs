

    pub trait Expanded {
        type Compressed: Compressed<Expanded = Self>;

        #[must_use]
        #[inline]
        fn compress<C: Compander<Self, Self::Compressed>>(self) -> Self::Compressed
        where
            Self: Sized,
        {
            C::compress(self)
        }
    }

    pub trait Compressed {
        type Expanded: Expanded<Compressed = Self>;

        #[must_use]
        #[inline]
        fn expand<C: Compander<Self::Expanded, Self>>(self) -> Self::Expanded
        where
            Self: Sized,
        {
            C::expand(self)
        }
    }

    pub trait Compander<E, C>
    where
        E: Expanded<Compressed = C>,
        C: Compressed<Expanded = E>,
    {
        #[must_use]
        fn compress(linear: E) -> C;

        #[must_use]
        fn expand(log: C) -> E;
    }

    pub trait ExpandedBuf {
        type CompressedBuf: CompressedBuf<ExpandedBuf = Self> + ?Sized;

        #[inline]
        fn compress_buf<C: BufCompander<Self, Self::CompressedBuf>>(
            &self,
            log: &mut Self::CompressedBuf,
        ) -> usize {
            C::compress_buf(self, log)
        }
    }

    pub trait CompressedBuf {
        type ExpandedBuf: ExpandedBuf<CompressedBuf = Self> + ?Sized;

        #[inline]
        fn expand_buf<C: BufCompander<Self::ExpandedBuf, Self>>(
            &self,
            linear: &mut Self::ExpandedBuf,
        ) -> usize {
            C::expand_buf(self, linear)
        }
    }

    pub trait BufCompander<E, C>
    where
        E: ExpandedBuf<CompressedBuf = C> + ?Sized,
        C: CompressedBuf<ExpandedBuf = E> + ?Sized,
    {
        fn compress_buf(linear_buf: &E, log_buf: &mut C) -> usize;

        fn expand_buf(log_buf: &C, linear_buf: &mut E) -> usize;
    }

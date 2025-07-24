pub use rand::Rng;
use rand::distr::{Alphanumeric, Distribution, SampleString, StandardUniform};

pub trait Strand: Sized {
    fn strand() -> impl Distribution<Self>;
}

struct StringDistr;

impl Distribution<String> for StringDistr {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        Alphanumeric.sample_string(rng, 8)
    }
}

impl Strand for String {
    fn strand() -> impl Distribution<Self> {
        return StringDistr;
    }
}

impl Strand for u8 {
    fn strand() -> impl Distribution<Self> {
        StandardUniform
    }
}

#[cfg(test)]
mod test {
    #![allow(dead_code)]

    use super::Strand;
    use rand::Rng;
    use rand::distr::Distribution;

    #[derive(Debug)]
    pub struct Foo {
        pub bar: String,
        pub baz: u8,
    }

    /// Generated distr trait
    pub trait FooDistribution: Distribution<Foo> {
        /// Return a new `FooDistribution` using the provided distribution for the `bar` field.
        fn with_bar(self, bar: impl Distribution<String>) -> impl FooDistribution;
        /// Return a new `FooDistribution` using the provided distribution for the `baz` field.
        fn with_baz(self, baz: impl Distribution<u8>) -> impl FooDistribution;
    }

    /// Generated distr struct
    struct FooDistr<A, B> {
        bar: A,
        baz: B,
    }

    impl<A, B> Distribution<Foo> for FooDistr<A, B>
    where
        A: Distribution<String>,
        B: Distribution<u8>,
    {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Foo {
            Foo {
                bar: self.bar.sample(rng),
                baz: self.baz.sample(rng),
            }
        }
    }

    impl<A, B> FooDistribution for FooDistr<A, B>
    where
        A: Distribution<String>,
        B: Distribution<u8>,
    {
        fn with_bar(self, bar: impl Distribution<String>) -> impl FooDistribution {
            FooDistr { bar, baz: self.baz }
        }

        fn with_baz(self, baz: impl Distribution<u8>) -> impl FooDistribution {
            FooDistr { bar: self.bar, baz }
        }
    }

    impl Strand for Foo {
        #[allow(refining_impl_trait)]
        fn strand() -> impl FooDistribution {
            FooDistr {
                bar: String::strand(),
                baz: u8::strand(),
            }
        }
    }

    #[test]
    fn foo_strand() {
        let strand = Foo::strand();
        let foo = strand.sample(&mut rand::rng());
        println!("{:?}", foo);
    }
}

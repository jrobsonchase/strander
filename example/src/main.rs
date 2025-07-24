use strander::Strand;
use strander::rand;
use strander::rand::distr::Alphabetic;
use strander::rand::distr::Distribution;

#[derive(Debug, Strand)]
struct Foo {
    bar: String,
    baz: u8,
}

fn main() {
    let foogen = Foo::strand().with_baz(Alphabetic);
    println!("Hello, {:?}!", foogen.sample(&mut rand::rng()));
}

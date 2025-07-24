use strander::rand::distr::Alphanumeric;
use strander::Strand;
use strander::rand;
use strander::rand::distr::Alphabetic;
use strander::rand::distr::Distribution;

#[derive(Debug, Strand)]
pub struct Foo {
    bar: String,
    #[strand = "Alphabetic"]
    baz: u8,
}

fn main() {
    let foogen = Foo::strand().with_baz(Alphanumeric);
    println!("Hello, {:?}!", foogen.sample(&mut rand::rng()));
}

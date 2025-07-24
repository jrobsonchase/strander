use bar::Spam;
use strander::rand::distr::Alphanumeric;
use strander::rand::distr::Distribution;
use strander::strand_remote;

#[strand_remote]
pub struct Spam {
    and: String,
    #[strand = "Alphanumeric"]
    eggs: u8,
}

fn main() {
    let spamgen = SpamDistr::new();

    let spam = spamgen.sample(&mut strander::rand::rng());

    println!("{:?}", spam);
}

use url::percent_encoding::percent_decode;
use failure::Error;
use structopt::StructOpt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    Ok(println!("{}", decode(&opt.input)?))
}

fn decode(input: &str) -> Result<String> {
    let decoded = percent_decode(input.as_bytes()).decode_utf8()?;
    Ok(decoded.to_string())
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn decode_space_ok() {
        let want = "foo bar";
        let input = "foo%20bar";
        let got = decode(input).unwrap();
        assert_eq!(want, got)
    }

    #[test]
    fn decode_ascii_ok() {
        let want = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let input = r##"%20%21%22%23%24%25%26%27%28%29%2A%2B%2C%2D%2E%2F0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let got = decode(input).unwrap();
        assert_eq!(want, got);
    }

//    #[test]
//    #[should_panic]
//    fn decode_invalid_utf8_ng() {
//        let input = "%93%FA%96%7B%8C%EA%0D%0A";
//        decode(input);
//    }
}

use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Parser, Debug)]
struct Ruruid {
    #[clap(short = 'n', default_value_t = 1)]
    num: u32,
}
fn main() {
    let args = Ruruid::try_parse().expect("failed to parse args");
    let mut results = vec![];
    for _ in 0..args.num {
        results.push(uuid::Uuid::new_v4().to_string());
    }
    let mut clipboard = ClipboardContext::new().expect("failed generate clipboard context");
    clipboard
        .set_contents(results.iter().fold("".to_string(), |x, y| {
            if x.len() > 0 {
                format!("{}\n{}", x, y)
            } else {
                y.clone()
            }
        }))
        .expect("failed set text for clipboard");
    println!(
        "{}",
        clipboard
            .get_contents()
            .expect("failed to get clipboard contents")
    );
}

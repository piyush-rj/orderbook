use std::io::{self, BufRead, Write};
use engine::{Engine, EngineInput, EngineOutput};

fn main() {
    let mut engine = Engine::new();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    eprintln!("[engine] ready");

    for line_result in stdin.lock().lines() {
        let line = match line_result {
            Ok(l) if l.trim().is_empty() => continue,
            Ok(l) => l,
            Err(_) => break,
        };

        eprintln!("[in ] {}", line);

        let input = match EngineInput::parse(&line) {
            Ok(i) => i,
            Err(err) => {
                eprintln!("[err] parse failed: {}", err);
                continue;
            }
        };

        let output = match input {
            EngineInput::NewOrder { side, price, quantity, order_type } => {
                eprintln!(
                    "[req] place_order side={:?} price={} qty={} type={:?}",
                    side, price, quantity, order_type
                );
                EngineOutput::PlaceResult(
                    engine.place_order(side, price, quantity, order_type)
                )
            }
            EngineInput::CancelOrder { order_id } => {
                eprintln!("[req] cancel_order id={}", order_id);
                EngineOutput::CancelResult(
                    engine.cancel_order(order_id)
                )
            }
        };

        match output.to_json() {
            Ok(json) => {
                eprintln!("[out] {}", json);
                writeln!(out, "{}", json).ok();
            }
            Err(e) => eprintln!("[err] serialize failed: {}", e),
        }
    }

    eprintln!("[engine] shutting down");
}

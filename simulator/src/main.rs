use processor::Processor;

mod instructions;
mod processor;

fn main() {
    let mut processor = Processor::new();
    processor.begin_cicle();
}

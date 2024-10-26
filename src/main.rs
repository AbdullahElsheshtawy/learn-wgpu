use lib::run;

mod lib;

fn main() {
    pollster::block_on(run());
}

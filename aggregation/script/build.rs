use sp1_helper::build_program_with_args;

fn main() {
    build_program_with_args("../program", Default::default());
    build_program_with_args("../../fibonacci/program", Default::default());
}

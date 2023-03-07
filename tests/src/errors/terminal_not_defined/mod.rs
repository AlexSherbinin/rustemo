use rustemo_tools::{local_file, output_cmp};

#[test]
fn terminal_not_defined() {
    let result = rustemo_tools::with_settings()
        .process_grammar(local_file!(file!(), "terminal_not_defined.rustemo"));
    output_cmp!(
        "src/errors/terminal_not_defined/terminal_not_defined.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn terminal_not_defined_sugar() {
    let result = rustemo_tools::with_settings().process_grammar(local_file!(
        file!(),
        "terminal_not_defined_sugar.rustemo"
    ));
    output_cmp!(
        "src/errors/terminal_not_defined/terminal_not_defined_sugar.err",
        result.unwrap_err().to_string()
    )
}

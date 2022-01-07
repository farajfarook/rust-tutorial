mod string_example;
mod vector_example;

fn main() {
    vector_example::create();
    vector_example::update();
    vector_example::drop();
    vector_example::read();
    vector_example::iterate();
    vector_example::multi_type();

    string_example::create();
    string_example::update();
    string_example::slicing();
    string_example::iterating();
}

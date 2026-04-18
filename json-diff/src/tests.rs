use std::convert::Infallible;

use serde_json::json;
use test_case::test_case;

use super::*;

#[test_case(
    json!(null), json!(null)
    => vec![Chunk::Null, Chunk::NewLine];
    "nulls"
)]
#[test_case(
    json!(true), json!(true)
    => vec![Chunk::Bool(true), Chunk::NewLine];
    "bools: equal"
)]
#[test_case(
    json!(true), json!(false)
    => vec![
        Chunk::left(Chunk::Bool(true)), Chunk::NewLine,
        Chunk::right(Chunk::Bool(false)), Chunk::NewLine,
    ];
    "bools: different"
)]
#[test_case(
    json!(7), json!(7)
    => vec![Chunk::Number(7.into()), Chunk::NewLine];
    "numbers: equal"
)]
#[test_case(
    json!(3), json!(4)
    => vec![
        Chunk::left(Chunk::Number(3.into())), Chunk::NewLine,
        Chunk::right(Chunk::Number(4.into())), Chunk::NewLine,
    ];
    "numbers: different"
)]
#[test_case(
    json!("a"), json!("a")
    => vec![Chunk::String("a".into()), Chunk::NewLine];
    "strings: equal"
)]
#[test_case(
    json!("a"), json!("b")
    => vec![
        Chunk::left(Chunk::String("a".into())), Chunk::NewLine,
        Chunk::right(Chunk::String("b".into())), Chunk::NewLine,
    ];
    "strings: different"
)]
#[test_case(
    json!([]), json!([])
    => vec![Chunk::ArrayBegin, Chunk::ArrayEnd, Chunk::NewLine];
    "arrays: empty"
)]
#[test_case(
    json!([true, 7, "a"]), json!([true, 7, "a"])
    => vec![
        Chunk::ArrayBegin, Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(0)), Chunk::Bool(true), Chunk::Comma, Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(1)), Chunk::Number(7.into()), Chunk::Comma, Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(2)), Chunk::String("a".into()), Chunk::NewLine,
        Chunk::ArrayEnd, Chunk::NewLine,
    ];
    "arrays: equal, non-empty"
)]
#[test_case(
    json!([true, 3, "a"]), json!([true, 4, "a"])
    => vec![
        Chunk::ArrayBegin, Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(0)), Chunk::Bool(true), Chunk::Comma, Chunk::NewLine,
        Chunk::Indent(1), Chunk::changed(Chunk::Position(Position::Index(1))), Chunk::left(Chunk::Number(3.into())), Chunk::left(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::changed(Chunk::Position(Position::Index(1))), Chunk::right(Chunk::Number(4.into())), Chunk::right(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(2)), Chunk::String("a".into()), Chunk::NewLine,
        Chunk::ArrayEnd, Chunk::NewLine,
    ];
    "arrays: different, same lengths"
)]
#[test_case(
    json!([true, 3, "a", "b", "c"]), json!([true, 4, "a"])
    => vec![
        Chunk::ArrayBegin, Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(0)), Chunk::Bool(true), Chunk::Comma, Chunk::NewLine,
        Chunk::Indent(1), Chunk::changed(Chunk::Position(Position::Index(1))), Chunk::left(Chunk::Number(3.into())), Chunk::left(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::changed(Chunk::Position(Position::Index(1))), Chunk::right(Chunk::Number(4.into())), Chunk::right(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(2)), Chunk::String("a".into()), Chunk::left(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::left(Chunk::Position(Position::Index(3))), Chunk::left(Chunk::String("b".into())), Chunk::left(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::left(Chunk::Position(Position::Index(4))), Chunk::left(Chunk::String("c".into())), Chunk::NewLine,
        Chunk::ArrayEnd, Chunk::NewLine,
    ];
    "arrays: different, left has larger length"
)]
#[test_case(
    json!([true, 3, "a"]), json!([true, 4, "a", 5, false])
    => vec![
        Chunk::ArrayBegin, Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(0)), Chunk::Bool(true), Chunk::Comma, Chunk::NewLine,
        Chunk::Indent(1), Chunk::changed(Chunk::Position(Position::Index(1))), Chunk::left(Chunk::Number(3.into())), Chunk::left(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::changed(Chunk::Position(Position::Index(1))), Chunk::right(Chunk::Number(4.into())), Chunk::right(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::Position(Position::Index(2)), Chunk::String("a".into()), Chunk::right(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::right(Chunk::Position(Position::Index(3))), Chunk::right(Chunk::Number(5.into())), Chunk::right(Chunk::Comma), Chunk::NewLine,
        Chunk::Indent(1), Chunk::right(Chunk::Position(Position::Index(4))), Chunk::right(Chunk::Bool(false)), Chunk::NewLine,
        Chunk::ArrayEnd, Chunk::NewLine,
    ];
    "arrays: different, right has larger length"
)]
fn diff_chunks(left: Value, right: Value) -> Vec<Chunk> {
    struct Writer {
        chunks: Vec<Chunk>,
    }
    impl Write for Writer {
        type Error = Infallible;

        fn write(&mut self, chunk: Chunk) -> Result<(), Self::Error> {
            self.chunks.push(chunk);
            Ok(())
        }
    }
    let mut writer = Writer { chunks: Vec::new() };

    let mut comparator = Comparator::new(&mut writer);

    comparator.compare(left, right);

    writer.chunks
}

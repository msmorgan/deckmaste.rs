use deckmaste_english::syntax::RecoveredText;
use deckmaste_english::syntax::Sentence;
use deckmaste_english::syntax::SentenceBody;

fn bypass_sentence_assembly() -> Sentence {
    Sentence {
        body: SentenceBody::Recovered(RecoveredText::new("raw", 1)),
    }
}

fn main() {}

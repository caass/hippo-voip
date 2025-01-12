mod ulaw;

pub use ulaw::ULaw;

mod sys {
    include!(env!("G711_H_RS"));
}

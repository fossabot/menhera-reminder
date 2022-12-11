#[deprecated(note = "uncomplete definition")]
pub struct DateTime;

pub enum Kind {
    Private,
    Work,
}

pub struct Item {
    datetime: DateTime,
    content: String,
    kind: Kind,
}

pub enum Relation {
    Continuing,
    Broken,
}

pub struct Manager {
    name: String,
    relying: f64,
    relation: Relation,
}

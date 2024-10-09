#[derive(Debug, Clone)]
pub enum Node {
    Table(Table),
    Array(Array),
    Bool(Bool),
    Str(Str),
    Integer(Integer),
    Float(Float),
    DateTime(DateTime),
    Date(Data),
    Time(Time),
    Invalid(Invalid),
}

use physis::excel::Row;

pub trait StructuredSheet<'a> {
    type Row;

    fn read_row(&'a self, row: &'a Row) -> Option<Self::Row>;
}

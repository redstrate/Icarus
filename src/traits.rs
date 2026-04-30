use physis::excel::Row;

pub trait StructuredSheet {
    type Row;

    fn read_row(&self, row: &Row) -> Option<Self::Row>;
}

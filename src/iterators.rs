use crate::StructuredSheet;

use physis::excel::{SheetIterator, Page, Row};

pub struct StructuredSheetIterator<'a, S: StructuredSheet<'a>> {
    pub(crate) sheet: &'a S,
    pub(crate) iterator: SheetIterator<'a>,
}

impl<'a, S: StructuredSheet<'a>> StructuredSheetIterator<'a, S> {
    /// Flattens this iterator, giving you one that only contains rows.
    ///
    /// If this sheet actually has subrows, then it only takes the first one in each row.
    pub fn flatten_subrows(&self) -> StructuredRowIterator<'a, S> {
        StructuredRowIterator {
            sheet: self.sheet,
            iterator: self.iterator.clone(),
        }
    }
}

impl<'a, S: StructuredSheet<'a>> Iterator for StructuredSheetIterator<'a, S> {
    type Item = (u32, Vec<(u16, S::Row)>);

    fn next(&mut self) -> Option<Self::Item> {
        let (row_id, rows) = self.iterator.next()?;
        Some((row_id, rows.into_iter().map(|(subrow_id, row)| (*subrow_id, self.sheet.read_row(row).unwrap())).collect()))
    }
}

/// Iterator over an [Page], but only the rows.
///
/// To create this iterator, use [PageIterator::flatten_subrows].
pub struct StructuredRowIterator<'a, S: StructuredSheet<'a>> {
    sheet: &'a S,
    iterator: SheetIterator<'a>,
}

impl<'a, S: StructuredSheet<'a>> Iterator for StructuredRowIterator<'a, S> {
    type Item = (u32, S::Row);

    fn next(&mut self) -> Option<Self::Item> {
        let (row_id, rows) = self.iterator.next()?;
        let row = &rows.first()?.1;
        Some((row_id, self.sheet.read_row(row).unwrap()))
    }
}


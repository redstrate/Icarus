//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct ClassJobActionUISheet {
    sheet: Sheet,
}
impl ClassJobActionUISheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJobActionUI")?;
        let sheet = resolver.read_excel_sheet(&exh, "ClassJobActionUI", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ClassJobActionUIRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobActionUIRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ClassJobActionUISheet {
    type Row = ClassJobActionUIRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            UpgradeAction: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            BaseAction: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            ComboTreeLayout: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Unknown3: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown4: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown5: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            GroupedCell: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ClassJobActionUISheet {
    type Item = (u32, Vec<(u16, ClassJobActionUIRow)>);
    type IntoIter = StructuredSheetIterator<'a, ClassJobActionUISheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ClassJobActionUISheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClassJobActionUIRow {
    ///""
    pub UpgradeAction: u32,
    ///""
    pub BaseAction: u32,
    ///"Used to position the action within a combo tree diagram. Non-zero digits identify branches within the combo (eg, Hakaze -> Shifu -> Kasha is 100 -> 120 -> 121)"
    pub ComboTreeLayout: u16,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub Unknown5: u8,
    ///"Currently only used for MNK actions; displays a set of actions in a shared rectangular cell instead of a tree."
    pub GroupedCell: bool,
}

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
pub struct CharaCardDesignPresetSheet {
    sheet: Sheet,
}
impl CharaCardDesignPresetSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CharaCardDesignPreset")?;
        let sheet = resolver.read_excel_sheet(&exh, "CharaCardDesignPreset", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CharaCardDesignPresetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<CharaCardDesignPresetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CharaCardDesignPresetSheet {
    type Row = CharaCardDesignPresetRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            BasePlate: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Backing: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            PatternOverlay: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            PortraitFrame: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            PlateFrame: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            Accent: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            SortKey: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            TopBorder: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            BottomBorder: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a CharaCardDesignPresetSheet {
    type Item = (u32, Vec<(u16, CharaCardDesignPresetRow)>);
    type IntoIter = StructuredSheetIterator<'a, CharaCardDesignPresetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CharaCardDesignPresetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CharaCardDesignPresetRow {
    ///""
    pub Name: String,
    ///""
    pub BasePlate: u16,
    ///""
    pub Backing: u16,
    ///""
    pub PatternOverlay: u16,
    ///""
    pub PortraitFrame: u16,
    ///""
    pub PlateFrame: u16,
    ///""
    pub Accent: u16,
    ///""
    pub SortKey: u16,
    ///""
    pub TopBorder: u8,
    ///""
    pub BottomBorder: u8,
}

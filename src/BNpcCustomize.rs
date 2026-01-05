//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct BNpcCustomizeSheet {
    sheet: ExcelSheet,
}
impl BNpcCustomizeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcCustomize")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcCustomize", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<BNpcCustomizeRow> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(BNpcCustomizeRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<BNpcCustomizeRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcCustomizeRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct BNpcCustomizeRow {
    columns: Vec<ColumnData>,
}
impl BNpcCustomizeRow {
    pub fn Race<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Gender<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn BodyType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Height<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Tribe<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Face<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn HairStyle<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn HairHighlight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn SkinColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn EyeHeterochromia<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn HairColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn HairHighlightColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn FacialFeature<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn FacialFeatureColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Eyebrows<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn EyeColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn EyeShape<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Nose<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Jaw<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Mouth<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn LipColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn BustOrTone1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn ExtraFeature1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn ExtraFeature2OrBust<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn FacePaint<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn FacePaintColor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
}

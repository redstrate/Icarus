//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct ClassJobCategorySheet {
    sheet: ExcelSheet,
}
impl ClassJobCategorySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJobCategory")?;
        let sheet = resolver.read_excel_sheet(exh, "ClassJobCategory", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ClassJobCategoryRow> {
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
        Some(ClassJobCategoryRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ClassJobCategoryRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ClassJobCategoryRow> {
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
pub struct ClassJobCategoryRow {
    columns: Vec<ColumnData>,
}
impl ClassJobCategoryRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn ADV<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn GLA<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn PGL<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn MRD<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn LNC<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn ARC<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn CNJ<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn THM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn CRP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn BSM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn ARM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn GSM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn LTW<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn WVR<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn ALC<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn CUL<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn MIN<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn BTN<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn FSH<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn PLD<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn MNK<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn WAR<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn DRG<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn BRD<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn WHM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn BLM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn ACN<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn SMN<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn SCH<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn ROG<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn NIN<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn MCH<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn DRK<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn AST<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn SAM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn RDM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn BLU<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn GNB<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn DNC<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn RPR<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn SGE<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn VPR<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn PCT<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
}

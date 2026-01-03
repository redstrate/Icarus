//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct MountCustomizeSheet {
    sheet: ExcelSheet,
}
impl MountCustomizeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MountCustomize")?;
        let sheet = resolver.read_excel_sheet(exh, "MountCustomize", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<MountCustomizeRow> {
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
        Some(MountCustomizeRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<MountCustomizeRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountCustomizeRow> {
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
pub struct MountCustomizeRow {
    columns: Vec<ColumnData>,
}
impl MountCustomizeRow {
    pub fn HyurMidlanderMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn HyurMidlanderFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn HyurHighlanderMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn HyurHighlanderFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn ElezenMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn ElezenFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn LalaMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn LalaFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn MiqoMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn MiqoFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn RoeMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn RoeFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn AuRaMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn AuRaFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn HrothgarMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn HrothgarFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn VieraMaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn VieraFemaleScale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn HyurMidlanderMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn HyurMidlanderFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn HyurHighlanderMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn HyurHighlanderFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn ElezenMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn ElezenFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn LalaMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn LalaFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn MiqoMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn MiqoFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn RoeMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn RoeFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn AuRaMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn AuRaFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn HrothgarMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn HrothgarFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn VieraMaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn VieraFemaleCameraHeight<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
}

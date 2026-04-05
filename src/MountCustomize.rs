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
pub struct MountCustomizeSheet {
    sheet: Sheet,
}
impl MountCustomizeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MountCustomize")?;
        let sheet = resolver.read_excel_sheet(&exh, "MountCustomize", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MountCustomizeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountCustomizeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MountCustomizeSheet {
    type Row = MountCustomizeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MountCustomizeSheet {
    type Item = (u32, Vec<(u16, MountCustomizeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MountCustomizeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MountCustomizeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MountCustomizeRow<'a> {
    row: &'a Row,
}
impl<'a> MountCustomizeRow<'a> {
    pub fn HyurMidlanderMaleScale(&'a self) -> u16 {
        self.row.columns[1].into_u16().copied().unwrap()
    }
    pub fn HyurMidlanderFemaleScale(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn HyurHighlanderMaleScale(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn HyurHighlanderFemaleScale(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn ElezenMaleScale(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn ElezenFemaleScale(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn LalaMaleScale(&'a self) -> u16 {
        self.row.columns[7].into_u16().copied().unwrap()
    }
    pub fn LalaFemaleScale(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn MiqoMaleScale(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn MiqoFemaleScale(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn RoeMaleScale(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn RoeFemaleScale(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn AuRaMaleScale(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn AuRaFemaleScale(&'a self) -> u16 {
        self.row.columns[14].into_u16().copied().unwrap()
    }
    pub fn HrothgarMaleScale(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn HrothgarFemaleScale(&'a self) -> u16 {
        self.row.columns[16].into_u16().copied().unwrap()
    }
    pub fn VieraMaleScale(&'a self) -> u16 {
        self.row.columns[17].into_u16().copied().unwrap()
    }
    pub fn VieraFemaleScale(&'a self) -> u16 {
        self.row.columns[18].into_u16().copied().unwrap()
    }
    pub fn HyurMidlanderMaleCameraHeight(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn HyurMidlanderFemaleCameraHeight(&'a self) -> u16 {
        self.row.columns[20].into_u16().copied().unwrap()
    }
    pub fn HyurHighlanderMaleCameraHeight(&'a self) -> u16 {
        self.row.columns[21].into_u16().copied().unwrap()
    }
    pub fn HyurHighlanderFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[22].into_u8().copied().unwrap()
    }
    pub fn ElezenMaleCameraHeight(&'a self) -> u8 {
        self.row.columns[23].into_u8().copied().unwrap()
    }
    pub fn ElezenFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn LalaMaleCameraHeight(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn LalaFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[26].into_u8().copied().unwrap()
    }
    pub fn MiqoMaleCameraHeight(&'a self) -> u8 {
        self.row.columns[27].into_u8().copied().unwrap()
    }
    pub fn MiqoFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[28].into_u8().copied().unwrap()
    }
    pub fn RoeMaleCameraHeight(&'a self) -> u8 {
        self.row.columns[29].into_u8().copied().unwrap()
    }
    pub fn RoeFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[30].into_u8().copied().unwrap()
    }
    pub fn AuRaMaleCameraHeight(&'a self) -> u8 {
        self.row.columns[31].into_u8().copied().unwrap()
    }
    pub fn AuRaFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[32].into_u8().copied().unwrap()
    }
    pub fn HrothgarMaleCameraHeight(&'a self) -> u8 {
        self.row.columns[33].into_u8().copied().unwrap()
    }
    pub fn HrothgarFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[34].into_u8().copied().unwrap()
    }
    pub fn VieraMaleCameraHeight(&'a self) -> u8 {
        self.row.columns[35].into_u8().copied().unwrap()
    }
    pub fn VieraFemaleCameraHeight(&'a self) -> u8 {
        self.row.columns[36].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[37].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[38].into_u8().copied().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> u8 {
        self.row.columns[39].into_u8().copied().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> u8 {
        self.row.columns[40].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[41].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u8 {
        self.row.columns[42].into_u8().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
}

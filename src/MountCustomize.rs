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
impl StructuredSheet for MountCustomizeSheet {
    type Row = MountCustomizeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            HyurMidlanderMaleScale: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            HyurMidlanderFemaleScale: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            HyurHighlanderMaleScale: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            HyurHighlanderFemaleScale: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            ElezenMaleScale: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            ElezenFemaleScale: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            LalaMaleScale: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            LalaFemaleScale: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            MiqoMaleScale: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            MiqoFemaleScale: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            RoeMaleScale: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            RoeFemaleScale: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            AuRaMaleScale: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            AuRaFemaleScale: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            HrothgarMaleScale: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            HrothgarFemaleScale: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            VieraMaleScale: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            VieraFemaleScale: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            HyurMidlanderMaleCameraHeight: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            HyurMidlanderFemaleCameraHeight: row
                .columns[20]
                .into_u16()
                .copied()
                .expect("Expected column 20 to be a uint16!"),
            HyurHighlanderMaleCameraHeight: row
                .columns[21]
                .into_u16()
                .copied()
                .expect("Expected column 21 to be a uint16!"),
            HyurHighlanderFemaleCameraHeight: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            ElezenMaleCameraHeight: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            ElezenFemaleCameraHeight: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            LalaMaleCameraHeight: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            LalaFemaleCameraHeight: row
                .columns[26]
                .into_u8()
                .copied()
                .expect("Expected column 26 to be a uint8!"),
            MiqoMaleCameraHeight: row
                .columns[27]
                .into_u8()
                .copied()
                .expect("Expected column 27 to be a uint8!"),
            MiqoFemaleCameraHeight: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            RoeMaleCameraHeight: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            RoeFemaleCameraHeight: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            AuRaMaleCameraHeight: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            AuRaFemaleCameraHeight: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            HrothgarMaleCameraHeight: row
                .columns[33]
                .into_u8()
                .copied()
                .expect("Expected column 33 to be a uint8!"),
            HrothgarFemaleCameraHeight: row
                .columns[34]
                .into_u8()
                .copied()
                .expect("Expected column 34 to be a uint8!"),
            VieraMaleCameraHeight: row
                .columns[35]
                .into_u8()
                .copied()
                .expect("Expected column 35 to be a uint8!"),
            VieraFemaleCameraHeight: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            Unknown0: row
                .columns[37]
                .into_u8()
                .copied()
                .expect("Expected column 37 to be a uint8!"),
            Unknown1: row
                .columns[38]
                .into_u8()
                .copied()
                .expect("Expected column 38 to be a uint8!"),
            Unknown_70_1: row
                .columns[39]
                .into_u8()
                .copied()
                .expect("Expected column 39 to be a uint8!"),
            Unknown_70_2: row
                .columns[40]
                .into_u8()
                .copied()
                .expect("Expected column 40 to be a uint8!"),
            Unknown2: row
                .columns[41]
                .into_u8()
                .copied()
                .expect("Expected column 41 to be a uint8!"),
            Unknown3: row
                .columns[42]
                .into_u8()
                .copied()
                .expect("Expected column 42 to be a uint8!"),
            Unknown4: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a MountCustomizeSheet {
    type Item = (u32, Vec<(u16, MountCustomizeRow)>);
    type IntoIter = StructuredSheetIterator<'a, MountCustomizeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MountCustomizeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MountCustomizeRow {
    ///""
    pub HyurMidlanderMaleScale: u16,
    ///""
    pub HyurMidlanderFemaleScale: u16,
    ///""
    pub HyurHighlanderMaleScale: u16,
    ///""
    pub HyurHighlanderFemaleScale: u16,
    ///""
    pub ElezenMaleScale: u16,
    ///""
    pub ElezenFemaleScale: u16,
    ///""
    pub LalaMaleScale: u16,
    ///""
    pub LalaFemaleScale: u16,
    ///""
    pub MiqoMaleScale: u16,
    ///""
    pub MiqoFemaleScale: u16,
    ///""
    pub RoeMaleScale: u16,
    ///""
    pub RoeFemaleScale: u16,
    ///""
    pub AuRaMaleScale: u16,
    ///""
    pub AuRaFemaleScale: u16,
    ///""
    pub HrothgarMaleScale: u16,
    ///""
    pub HrothgarFemaleScale: u16,
    ///""
    pub VieraMaleScale: u16,
    ///""
    pub VieraFemaleScale: u16,
    ///""
    pub HyurMidlanderMaleCameraHeight: u16,
    ///""
    pub HyurMidlanderFemaleCameraHeight: u16,
    ///""
    pub HyurHighlanderMaleCameraHeight: u16,
    ///""
    pub HyurHighlanderFemaleCameraHeight: u8,
    ///""
    pub ElezenMaleCameraHeight: u8,
    ///""
    pub ElezenFemaleCameraHeight: u8,
    ///""
    pub LalaMaleCameraHeight: u8,
    ///""
    pub LalaFemaleCameraHeight: u8,
    ///""
    pub MiqoMaleCameraHeight: u8,
    ///""
    pub MiqoFemaleCameraHeight: u8,
    ///""
    pub RoeMaleCameraHeight: u8,
    ///""
    pub RoeFemaleCameraHeight: u8,
    ///""
    pub AuRaMaleCameraHeight: u8,
    ///""
    pub AuRaFemaleCameraHeight: u8,
    ///""
    pub HrothgarMaleCameraHeight: u8,
    ///""
    pub HrothgarFemaleCameraHeight: u8,
    ///""
    pub VieraMaleCameraHeight: u8,
    ///""
    pub VieraFemaleCameraHeight: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Unknown_70_1: u8,
    ///""
    pub Unknown_70_2: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown4: bool,
}

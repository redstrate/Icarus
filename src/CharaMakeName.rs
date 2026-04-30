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
pub struct CharaMakeNameSheet {
    sheet: Sheet,
}
impl CharaMakeNameSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CharaMakeName")?;
        let sheet = resolver.read_excel_sheet(&exh, "CharaMakeName", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CharaMakeNameRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CharaMakeNameRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CharaMakeNameSheet {
    type Row = CharaMakeNameRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            HyurMidlanderMale: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            HyurMidlanderFemale: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            HyurMidlanderLastName: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            HyurHighlanderMale: row
                .columns[3]
                .into_string()
                .cloned()
                .expect("Expected column 3 to be a string!"),
            HyurHighlanderFemale: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            HyurHighlanderLastName: row
                .columns[5]
                .into_string()
                .cloned()
                .expect("Expected column 5 to be a string!"),
            ElezenMale: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            ElezenFemale: row
                .columns[7]
                .into_string()
                .cloned()
                .expect("Expected column 7 to be a string!"),
            ElezenWildwoodLastName: row
                .columns[8]
                .into_string()
                .cloned()
                .expect("Expected column 8 to be a string!"),
            ElezenDuskwightLastName: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            MiqoteSunMale: row
                .columns[10]
                .into_string()
                .cloned()
                .expect("Expected column 10 to be a string!"),
            MiqoteSunFemale: row
                .columns[11]
                .into_string()
                .cloned()
                .expect("Expected column 11 to be a string!"),
            MiqoteSunMaleLastName: row
                .columns[12]
                .into_string()
                .cloned()
                .expect("Expected column 12 to be a string!"),
            MiqoteSunFemaleLastName: row
                .columns[13]
                .into_string()
                .cloned()
                .expect("Expected column 13 to be a string!"),
            MiqoteMoonMale: row
                .columns[14]
                .into_string()
                .cloned()
                .expect("Expected column 14 to be a string!"),
            MiqoteMoonFemale: row
                .columns[15]
                .into_string()
                .cloned()
                .expect("Expected column 15 to be a string!"),
            MiqoteMoonLastname: row
                .columns[16]
                .into_string()
                .cloned()
                .expect("Expected column 16 to be a string!"),
            LalafellPlainsfolkFirstNameStart: row
                .columns[17]
                .into_string()
                .cloned()
                .expect("Expected column 17 to be a string!"),
            LalafellPlainsfolkLastNameStart: row
                .columns[18]
                .into_string()
                .cloned()
                .expect("Expected column 18 to be a string!"),
            LalafellPlainsfolkEndOfNames: row
                .columns[19]
                .into_string()
                .cloned()
                .expect("Expected column 19 to be a string!"),
            LalafellDunesfolkMale: row
                .columns[20]
                .into_string()
                .cloned()
                .expect("Expected column 20 to be a string!"),
            LalafellDunesfolkMaleLastName: row
                .columns[21]
                .into_string()
                .cloned()
                .expect("Expected column 21 to be a string!"),
            LalafellDunesfolkFemale: row
                .columns[22]
                .into_string()
                .cloned()
                .expect("Expected column 22 to be a string!"),
            LalafellDunesfolkFemaleLastName: row
                .columns[23]
                .into_string()
                .cloned()
                .expect("Expected column 23 to be a string!"),
            RoegadynSeaWolfMale: row
                .columns[24]
                .into_string()
                .cloned()
                .expect("Expected column 24 to be a string!"),
            RoegadynSeaWolfMaleLastName: row
                .columns[25]
                .into_string()
                .cloned()
                .expect("Expected column 25 to be a string!"),
            RoegadynSeaWolfFemale: row
                .columns[26]
                .into_string()
                .cloned()
                .expect("Expected column 26 to be a string!"),
            RoegadynSeaWolfFemaleLastName: row
                .columns[27]
                .into_string()
                .cloned()
                .expect("Expected column 27 to be a string!"),
            RoegadynHellsguardFirstName: row
                .columns[28]
                .into_string()
                .cloned()
                .expect("Expected column 28 to be a string!"),
            RoegadynHellsguardMaleLastName: row
                .columns[29]
                .into_string()
                .cloned()
                .expect("Expected column 29 to be a string!"),
            RoegadynHellsguardFemaleLastName: row
                .columns[30]
                .into_string()
                .cloned()
                .expect("Expected column 30 to be a string!"),
            AuRaRaenMale: row
                .columns[31]
                .into_string()
                .cloned()
                .expect("Expected column 31 to be a string!"),
            AuRaRaenFemale: row
                .columns[32]
                .into_string()
                .cloned()
                .expect("Expected column 32 to be a string!"),
            AuRaRaenLastName: row
                .columns[33]
                .into_string()
                .cloned()
                .expect("Expected column 33 to be a string!"),
            AuRaXaelaMale: row
                .columns[34]
                .into_string()
                .cloned()
                .expect("Expected column 34 to be a string!"),
            AuRaXaelaFemale: row
                .columns[35]
                .into_string()
                .cloned()
                .expect("Expected column 35 to be a string!"),
            AuRaXaelaLastName: row
                .columns[36]
                .into_string()
                .cloned()
                .expect("Expected column 36 to be a string!"),
            HrothgarHellionsFirstName: row
                .columns[37]
                .into_string()
                .cloned()
                .expect("Expected column 37 to be a string!"),
            HrothgarHellionsLastName: row
                .columns[38]
                .into_string()
                .cloned()
                .expect("Expected column 38 to be a string!"),
            HrothgarLostFirstName: row
                .columns[39]
                .into_string()
                .cloned()
                .expect("Expected column 39 to be a string!"),
            HrothgarLostLastName: row
                .columns[40]
                .into_string()
                .cloned()
                .expect("Expected column 40 to be a string!"),
            Unknown0: row
                .columns[41]
                .into_string()
                .cloned()
                .expect("Expected column 41 to be a string!"),
            Unknown1: row
                .columns[42]
                .into_string()
                .cloned()
                .expect("Expected column 42 to be a string!"),
            Unknown2: row
                .columns[43]
                .into_string()
                .cloned()
                .expect("Expected column 43 to be a string!"),
            VieraFirstName: row
                .columns[44]
                .into_string()
                .cloned()
                .expect("Expected column 44 to be a string!"),
            VieraRavaLastName: row
                .columns[45]
                .into_string()
                .cloned()
                .expect("Expected column 45 to be a string!"),
            VieraVeenaLastName: row
                .columns[46]
                .into_string()
                .cloned()
                .expect("Expected column 46 to be a string!"),
            Unknown_70_1: row
                .columns[47]
                .into_string()
                .cloned()
                .expect("Expected column 47 to be a string!"),
            Unknown_70_2: row
                .columns[48]
                .into_string()
                .cloned()
                .expect("Expected column 48 to be a string!"),
            Unknown_70_3: row
                .columns[49]
                .into_string()
                .cloned()
                .expect("Expected column 49 to be a string!"),
        })
    }
}
impl<'a> IntoIterator for &'a CharaMakeNameSheet {
    type Item = (u32, Vec<(u16, CharaMakeNameRow)>);
    type IntoIter = StructuredSheetIterator<'a, CharaMakeNameSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CharaMakeNameSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CharaMakeNameRow {
    ///""
    pub HyurMidlanderMale: String,
    ///""
    pub HyurMidlanderFemale: String,
    ///""
    pub HyurMidlanderLastName: String,
    ///""
    pub HyurHighlanderMale: String,
    ///""
    pub HyurHighlanderFemale: String,
    ///""
    pub HyurHighlanderLastName: String,
    ///""
    pub ElezenMale: String,
    ///""
    pub ElezenFemale: String,
    ///""
    pub ElezenWildwoodLastName: String,
    ///""
    pub ElezenDuskwightLastName: String,
    ///""
    pub MiqoteSunMale: String,
    ///""
    pub MiqoteSunFemale: String,
    ///""
    pub MiqoteSunMaleLastName: String,
    ///""
    pub MiqoteSunFemaleLastName: String,
    ///""
    pub MiqoteMoonMale: String,
    ///""
    pub MiqoteMoonFemale: String,
    ///""
    pub MiqoteMoonLastname: String,
    ///""
    pub LalafellPlainsfolkFirstNameStart: String,
    ///""
    pub LalafellPlainsfolkLastNameStart: String,
    ///""
    pub LalafellPlainsfolkEndOfNames: String,
    ///""
    pub LalafellDunesfolkMale: String,
    ///""
    pub LalafellDunesfolkMaleLastName: String,
    ///""
    pub LalafellDunesfolkFemale: String,
    ///""
    pub LalafellDunesfolkFemaleLastName: String,
    ///""
    pub RoegadynSeaWolfMale: String,
    ///""
    pub RoegadynSeaWolfMaleLastName: String,
    ///""
    pub RoegadynSeaWolfFemale: String,
    ///""
    pub RoegadynSeaWolfFemaleLastName: String,
    ///""
    pub RoegadynHellsguardFirstName: String,
    ///""
    pub RoegadynHellsguardMaleLastName: String,
    ///""
    pub RoegadynHellsguardFemaleLastName: String,
    ///""
    pub AuRaRaenMale: String,
    ///""
    pub AuRaRaenFemale: String,
    ///""
    pub AuRaRaenLastName: String,
    ///""
    pub AuRaXaelaMale: String,
    ///""
    pub AuRaXaelaFemale: String,
    ///""
    pub AuRaXaelaLastName: String,
    ///""
    pub HrothgarHellionsFirstName: String,
    ///""
    pub HrothgarHellionsLastName: String,
    ///""
    pub HrothgarLostFirstName: String,
    ///""
    pub HrothgarLostLastName: String,
    ///""
    pub Unknown0: String,
    ///""
    pub Unknown1: String,
    ///""
    pub Unknown2: String,
    ///""
    pub VieraFirstName: String,
    ///""
    pub VieraRavaLastName: String,
    ///""
    pub VieraVeenaLastName: String,
    ///""
    pub Unknown_70_1: String,
    ///""
    pub Unknown_70_2: String,
    ///""
    pub Unknown_70_3: String,
}

//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct CharaMakeNameSheet {
    sheet: ExcelSheet,
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
    fn read_row(&self, row: &ExcelSingleRow) -> Option<CharaMakeNameRow> {
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
        Some(CharaMakeNameRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<CharaMakeNameRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CharaMakeNameRow> {
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
pub struct CharaMakeNameRow {
    columns: Vec<ColumnData>,
}
impl CharaMakeNameRow {
    pub fn HyurMidlanderMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn HyurMidlanderFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn HyurMidlanderLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn HyurHighlanderMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn HyurHighlanderFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn HyurHighlanderLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn ElezenMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn ElezenFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn ElezenWildwoodLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn ElezenDuskwightLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn MiqoteSunMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn MiqoteSunFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn MiqoteSunMaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn MiqoteSunFemaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn MiqoteMoonMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn MiqoteMoonFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn MiqoteMoonLastname<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn LalafellPlainsfolkFirstNameStart<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn LalafellPlainsfolkLastNameStart<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn LalafellPlainsfolkEndOfNames<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn LalafellDunesfolkMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn LalafellDunesfolkMaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn LalafellDunesfolkFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn LalafellDunesfolkFemaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn RoegadynSeaWolfMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn RoegadynSeaWolfMaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn RoegadynSeaWolfFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn RoegadynSeaWolfFemaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn RoegadynHellsguardFirstName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn RoegadynHellsguardMaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn RoegadynHellsguardFemaleLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn AuRaRaenMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn AuRaRaenFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn AuRaRaenLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn AuRaXaelaMale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn AuRaXaelaFemale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn AuRaXaelaLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn HrothgarHellionsFirstName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn HrothgarHellionsLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn HrothgarLostFirstName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn HrothgarLostLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn VieraFirstName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn VieraRavaLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn VieraVeenaLastName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn Unknown_70_3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
}

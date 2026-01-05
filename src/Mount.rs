//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct MountSheet {
    sheet: ExcelSheet,
}
impl MountSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Mount")?;
        let sheet = resolver.read_excel_sheet(&exh, "Mount", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<MountRow> {
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
        Some(MountRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<MountRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountRow> {
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
pub struct MountRow {
    columns: Vec<ColumnData>,
}
impl MountRow {
    pub fn Singular<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Plural<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Adjective<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn PossessivePronoun<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn StartsWithVowel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn Pronoun<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Article<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn ModelChara<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn EquipHead<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn EquipBody<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn EquipLeg<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn EquipFoot<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn MoveControl<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn RideBGM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn UIPriority<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn MountAction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn Order<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn FlyingCondition<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn IsFlying<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn MountCustomize<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn ExitMoveDist<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn ExitMoveSpeed<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn RadiusRate<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn BaseMotionSpeed_Run<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn BaseMotionSpeed_Walk<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn ExtraSeats<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn IsEmote<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn IsAirborne<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn ExHotbarEnableConfig<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn UseEP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn IsImmobile<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
    pub fn HideHeadgear<'a>(&'a self) -> &'a ColumnData {
        &self.columns[51]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a ColumnData {
        &self.columns[52]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a ColumnData {
        &self.columns[53]
    }
}

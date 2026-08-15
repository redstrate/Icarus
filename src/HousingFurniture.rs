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
pub struct HousingFurnitureSheet {
    sheet: Sheet,
}
impl HousingFurnitureSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 196608u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HousingFurniture")?;
        let sheet = resolver.read_excel_sheet(&exh, "HousingFurniture", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HousingFurnitureRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HousingFurnitureRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for HousingFurnitureSheet {
    type Row = HousingFurnitureRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            UsageParameter: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            CustomTalk: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Item: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            ModelKey: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            HousingItemCategory: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            UsageType: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            PlaceLimitType: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            AquariumTier: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Placement: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unplacement: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            UnplacementStorage: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            DestroyOnRemoval: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            Unknown4: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            Unknown5: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            Unknown6: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a HousingFurnitureSheet {
    type Item = (u32, Vec<(u16, HousingFurnitureRow)>);
    type IntoIter = StructuredSheetIterator<'a, HousingFurnitureSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HousingFurnitureSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HousingFurnitureRow {
    ///""
    pub UsageParameter: u32,
    ///""
    pub CustomTalk: u32,
    ///""
    pub Item: u32,
    ///""
    pub ModelKey: u16,
    ///""
    pub HousingItemCategory: u8,
    ///"0 = Static furnishing\n /// 3 = Vendor Permit\n /// 4 = Crafting Workbenche\n /// 7 = Food\n /// 8 = Aetherial Wheel\n /// 9 = Triple Triad Board\n /// 10 = Orchestrion\n /// 11 = Flowerpot\n /// 12 = Vase\n /// 13 = Aquarium\n /// 14 = Message Book\n /// 15 = Mannequin\n /// "
    pub UsageType: u8,
    ///"Any furnishings with the same type are counted towards the limit. For example, all vendor permits are grouped together under the same key."
    pub PlaceLimitType: u8,
    ///""
    pub AquariumTier: u8,
    ///""
    pub Placement: u8,
    ///""
    pub Unplacement: u8,
    ///""
    pub UnplacementStorage: u8,
    ///""
    pub DestroyOnRemoval: bool,
    ///""
    pub Unknown4: bool,
    ///""
    pub Unknown5: bool,
    ///""
    pub Unknown6: bool,
}

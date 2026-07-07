use super::csv_date::CsvDate;
use crate::{AcquisitionSample, AgriculturalSample, BrandedFood, CollectUpsertedRowsError, CompactDecimal, CsvRowError, DateRkyv, FnddsDerivation, FnddsIngredientFdcReference, FnddsIngredientNutrientValue, Food, FoodAttribute, FoodAttributeType, FoodCalorieConversionFactor, FoodCategory, FoodCategoryKey, FoodCategoryValue, FoodComponent, FoodDataType, FoodNutrient, FoodNutrientConversionFactor, FoodNutrientDerivation, FoodNutrientSource, FoodPortion, FoodProteinConversionFactor, FoodUpdateLogEntry, FoundationFood, InputFood, LabMethod, LabMethodCode, LabMethodNutrient, MarketAcquisition, MeasureUnit, Microbe, Nutrient, RetentionFactor, RetentionFactorKey, SampleFood, SrLegacyFood, SubSampleFood, SubSampleResult, SurveyFnddsFood, TryCsvRowsError, WweiaFoodCategory, WweiaFoodCategoryKey, YesNoBool, collect_upserted_rows, try_csv_rows};
use errgonomic::handle;
use rkyv::rancor::Fallible;
use rkyv::with::{ArchiveWith, DeserializeWith, Identity, MapKV, SerializeWith};
use rkyv::{Archive, Archived, Deserialize as RkyvDeserialize, Place, Resolver, Serialize as RkyvSerialize};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::path::Path;
use thiserror::Error;
use time::Date;

pub type AcquisitionSampleKey = (u32, u32);
pub type AcquisitionSampleRow = (AcquisitionSampleKey, AcquisitionSample);
type AcquisitionSampleCsvRow = (u32, u32);

pub type AgriculturalSampleRow = (u32, AgriculturalSample);
type AgriculturalSampleCsvRow = (u32, CsvDate, Box<str>, Box<str>, Box<str>);

pub type BrandedFoodRow = (u32, BrandedFood);

pub type FnddsDerivationRow = (Box<str>, FnddsDerivation);
type FnddsDerivationCsvRow = (Box<str>, Box<str>);

pub type FnddsIngredientNutrientValueKey = (u32, u16, FnddsIngredientFdcReference, Date, Date);
pub type FnddsIngredientNutrientValueRow = (FnddsIngredientNutrientValueKey, FnddsIngredientNutrientValue);
type FnddsIngredientNutrientValueCsvRow = (u32, Box<str>, u16, CompactDecimal, Box<str>, FnddsIngredientFdcReference, Option<Box<str>>, Option<u16>, u16, CsvDate, CsvDate);

pub struct FnddsIngredientNutrientValueKeyRkyv;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[rkyv(derive(Eq, PartialEq, Hash))]
pub struct FnddsIngredientNutrientValueKeyParts {
    ingredient_code: u32,
    nutrient_code: u16,
    fdc_reference: FnddsIngredientFdcReference,
    #[rkyv(with = DateRkyv)]
    start_date: Date,
    #[rkyv(with = DateRkyv)]
    end_date: Date,
}

impl FnddsIngredientNutrientValueKeyParts {
    fn from_key(&(ingredient_code, nutrient_code, fdc_reference, start_date, end_date): &FnddsIngredientNutrientValueKey) -> Self {
        Self {
            ingredient_code,
            nutrient_code,
            fdc_reference,
            start_date,
            end_date,
        }
    }

    fn into_key(self) -> FnddsIngredientNutrientValueKey {
        (self.ingredient_code, self.nutrient_code, self.fdc_reference, self.start_date, self.end_date)
    }
}

impl ArchiveWith<FnddsIngredientNutrientValueKey> for FnddsIngredientNutrientValueKeyRkyv {
    type Archived = Archived<FnddsIngredientNutrientValueKeyParts>;
    type Resolver = Resolver<FnddsIngredientNutrientValueKeyParts>;

    fn resolve_with(field: &FnddsIngredientNutrientValueKey, resolver: Self::Resolver, out: Place<Self::Archived>) {
        let parts = FnddsIngredientNutrientValueKeyParts::from_key(field);
        parts.resolve(resolver, out);
    }
}

impl<S> SerializeWith<FnddsIngredientNutrientValueKey, S> for FnddsIngredientNutrientValueKeyRkyv
where
    S: Fallible + ?Sized,
    FnddsIngredientNutrientValueKeyParts: RkyvSerialize<S>,
{
    fn serialize_with(field: &FnddsIngredientNutrientValueKey, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let parts = FnddsIngredientNutrientValueKeyParts::from_key(field);
        parts.serialize(serializer)
    }
}

impl<D> DeserializeWith<Archived<FnddsIngredientNutrientValueKeyParts>, FnddsIngredientNutrientValueKey, D> for FnddsIngredientNutrientValueKeyRkyv
where
    D: Fallible + ?Sized,
    Archived<FnddsIngredientNutrientValueKeyParts>: RkyvDeserialize<FnddsIngredientNutrientValueKeyParts, D>,
{
    // Project style requires explicit error branches instead of the try operator.
    #[allow(clippy::question_mark)]
    fn deserialize_with(field: &Archived<FnddsIngredientNutrientValueKeyParts>, deserializer: &mut D) -> Result<FnddsIngredientNutrientValueKey, D::Error> {
        let parts = match field.deserialize(deserializer) {
            Ok(parts) => parts,
            Err(error) => return Err(error),
        };
        Ok(parts.into_key())
    }
}

pub type FoodRow = (u32, Food);
type FoodCsvRow = (u32, FoodDataType, Option<Box<str>>, Option<Box<str>>, CsvDate);

pub type FoodAttributeRow = (u32, FoodAttribute);
type FoodAttributeCsvRow = (u32, u32, Option<u16>, Option<u16>, Option<Box<str>>, Option<Box<str>>);

pub type FoodAttributeTypeRow = (u16, FoodAttributeType);
type FoodAttributeTypeCsvRow = (u16, Box<str>, Box<str>);

pub type FoodCalorieConversionFactorRow = (u32, FoodCalorieConversionFactor);
type FoodCalorieConversionFactorCsvRow = (u32, Option<CompactDecimal>, Option<CompactDecimal>, Option<CompactDecimal>);

pub type FoodCategoryRow = (FoodCategoryKey, FoodCategory);
type FoodCategoryCsvRow = (FoodCategoryKey, u16, Box<str>);

pub type FoodComponentRow = (u32, FoodComponent);
type FoodComponentCsvRow = (u32, u32, Box<str>, Option<CompactDecimal>, YesNoBool, CompactDecimal, u16, Option<u16>);

pub type FoodNutrientRow = (u32, FoodNutrient);
type FoodNutrientCsvRow = (u32, u32, u16, CompactDecimal, Option<u16>, Option<u16>, Option<CompactDecimal>, Option<CompactDecimal>, Option<CompactDecimal>, Option<CompactDecimal>, Option<Box<str>>, Option<u16>, Option<CompactDecimal>);

pub type FoodNutrientConversionFactorRow = (u32, FoodNutrientConversionFactor);
type FoodNutrientConversionFactorCsvRow = (u32, u32);

pub type FoodNutrientDerivationRow = (u16, FoodNutrientDerivation);
type FoodNutrientDerivationCsvRow = (u16, Box<str>, Box<str>);

pub type FoodNutrientSourceRow = (u16, FoodNutrientSource);
type FoodNutrientSourceCsvRow = (u16, u16, Box<str>);

pub type FoodPortionRow = (u32, FoodPortion);
type FoodPortionCsvRow = (u32, Option<u32>, Option<u16>, Option<CompactDecimal>, Option<u16>, Option<Box<str>>, Option<Box<str>>, CompactDecimal, Option<u16>, Option<Box<str>>, Option<u16>);

pub type FoodProteinConversionFactorRow = (u32, FoodProteinConversionFactor);
type FoodProteinConversionFactorCsvRow = (u32, CompactDecimal);

pub type FoodUpdateLogEntryRow = (u32, FoodUpdateLogEntry);
type FoodUpdateLogEntryCsvRow = (u32, Box<str>, CsvDate);

pub type FoundationFoodRow = (u32, FoundationFood);
type FoundationFoodCsvRow = (u32, u32, Option<Box<str>>);

pub type InputFoodRow = (u32, InputFood);
type InputFoodCsvRow = (u32, u32, Option<u32>, u16, CompactDecimal, u32, Box<str>, Option<Box<str>>, u32, Box<str>, CompactDecimal, u16);

pub type LabMethodRow = (u16, LabMethod);
type LabMethodCsvRow = (u16, Box<str>, Box<str>);

pub type LabMethodCodeKey = (u16, Option<Box<str>>);
pub type LabMethodCodeRow = (LabMethodCodeKey, LabMethodCode);
type LabMethodCodeCsvRow = (u16, Option<Box<str>>);

pub type LabMethodNutrientKey = (u16, u16);
pub type LabMethodNutrientRow = (LabMethodNutrientKey, LabMethodNutrient);
type LabMethodNutrientCsvRow = (u16, u16);

pub type MarketAcquisitionRow = (u32, MarketAcquisition);
type MarketAcquisitionCsvRow = (u32, Option<Box<str>>, Option<CsvDate>, Option<Box<str>>, Option<Box<str>>, Option<CsvDate>, Option<Box<str>>, Option<Box<str>>, Option<CsvDate>, Option<Box<str>>, Option<Box<str>>, Option<Box<str>>, Option<Box<str>>, Box<str>);

pub type MeasureUnitRow = (u16, MeasureUnit);
type MeasureUnitCsvRow = (u16, Box<str>);

pub type MicrobeRow = (u16, Microbe);
type MicrobeCsvRow = (u16, u32, Box<str>, Box<str>, u32, Option<u32>, Option<Box<str>>);

pub type NutrientRow = (u16, Nutrient);
type NutrientCsvRow = (u16, Box<str>, Box<str>, Option<Box<str>>, Option<CompactDecimal>);

pub type RetentionFactorRow = (RetentionFactorKey, RetentionFactor);
type RetentionFactorCsvRow = (u16, RetentionFactorKey, u16, Box<str>);

pub type SampleFoodRow = (u32, SampleFood);
type SampleFoodCsvRow = (u32,);

pub type SrLegacyFoodRow = (u32, SrLegacyFood);
type SrLegacyFoodCsvRow = (u32, u32);

pub type SubSampleFoodRow = (u32, SubSampleFood);
type SubSampleFoodCsvRow = (u32, u32);

pub type SubSampleResultRow = (u32, SubSampleResult);
type SubSampleResultCsvRow = (u32, Option<CompactDecimal>, u16, Box<str>);

pub type SurveyFnddsFoodRow = (u32, SurveyFnddsFood);
type SurveyFnddsFoodCsvRow = (u32, u32, u16, CsvDate, CsvDate);

pub type WweiaFoodCategoryRow = (WweiaFoodCategoryKey, WweiaFoodCategory);
type WweiaFoodCategoryCsvRow = (WweiaFoodCategoryKey, Box<str>);

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Default)]
pub struct Database {
    pub acquisition_samples: FxHashMap<AcquisitionSampleKey, AcquisitionSample>,
    pub agricultural_samples: FxHashMap<u32, AgriculturalSample>,
    pub branded_food: FxHashMap<u32, BrandedFood>,
    pub fndds_derivation: FxHashMap<Box<str>, FnddsDerivation>,
    #[rkyv(with = MapKV<FnddsIngredientNutrientValueKeyRkyv, Identity>)]
    pub fndds_ingredient_nutrient_value: FxHashMap<FnddsIngredientNutrientValueKey, FnddsIngredientNutrientValue>,
    pub food: FxHashMap<u32, Food>,
    pub food_attribute: FxHashMap<u32, FoodAttribute>,
    pub food_attribute_type: FxHashMap<u16, FoodAttributeType>,
    pub food_calorie_conversion_factor: FxHashMap<u32, FoodCalorieConversionFactor>,
    pub food_category: FxHashMap<FoodCategoryKey, FoodCategory>,
    pub food_component: FxHashMap<u32, FoodComponent>,
    pub food_nutrient: FxHashMap<u32, FoodNutrient>,
    pub food_nutrient_conversion_factor: FxHashMap<u32, FoodNutrientConversionFactor>,
    pub food_nutrient_derivation: FxHashMap<u16, FoodNutrientDerivation>,
    pub food_nutrient_source: FxHashMap<u16, FoodNutrientSource>,
    pub food_portion: FxHashMap<u32, FoodPortion>,
    pub food_protein_conversion_factor: FxHashMap<u32, FoodProteinConversionFactor>,
    pub food_update_log_entry: FxHashMap<u32, FoodUpdateLogEntry>,
    pub foundation_food: FxHashMap<u32, FoundationFood>,
    pub input_food: FxHashMap<u32, InputFood>,
    pub lab_method: FxHashMap<u16, LabMethod>,
    pub lab_method_code: FxHashMap<LabMethodCodeKey, LabMethodCode>,
    pub lab_method_nutrient: FxHashMap<LabMethodNutrientKey, LabMethodNutrient>,
    pub market_acquisition: FxHashMap<u32, MarketAcquisition>,
    pub measure_unit: FxHashMap<u16, MeasureUnit>,
    pub microbe: FxHashMap<u16, Microbe>,
    pub nutrient: FxHashMap<u16, Nutrient>,
    pub retention_factor: FxHashMap<RetentionFactorKey, RetentionFactor>,
    pub sample_food: FxHashMap<u32, SampleFood>,
    pub sr_legacy_food: FxHashMap<u32, SrLegacyFood>,
    pub sub_sample_food: FxHashMap<u32, SubSampleFood>,
    pub sub_sample_result: FxHashMap<u32, SubSampleResult>,
    pub survey_fndds_food: FxHashMap<u32, SurveyFnddsFood>,
    pub wweia_food_category: FxHashMap<WweiaFoodCategoryKey, WweiaFoodCategory>,
}

macro_rules! define_iter {
    ($fn_name:ident, $file_name:literal, $csv_row:ty, $row:ty, $mapper:ident, $error:ident) => {
        impl Database {
            pub fn $fn_name(dir: &Path) -> Result<impl Iterator<Item = Result<$row, CsvRowError>>, $error> {
                use $error::*;
                let path = dir.join($file_name);
                let rows = handle!(try_csv_rows::<$csv_row>(dir, $file_name), TryCsvRowsFailed);
                Ok(rows.map(move |result| match result {
                    Ok(row) => Ok($mapper(row)),
                    Err(source) => Err(CsvRowError::ReadRowFailed {
                        source,
                        path: path.clone(),
                    }),
                }))
            }
        }

        #[derive(Error, Debug)]
        pub enum $error {
            #[error("failed to read CSV rows")]
            TryCsvRowsFailed { source: TryCsvRowsError },
        }
    };
}

#[derive(Deserialize)]
struct BrandedFoodCsv {
    fdc_id: u32,
    brand_owner: Option<Box<str>>,
    brand_name: Option<Box<str>>,
    subbrand_name: Option<Box<str>>,
    gtin_upc: Box<str>,
    ingredients: Option<Box<str>>,
    not_a_significant_source_of: Option<Box<str>>,
    serving_size: Option<CompactDecimal>,
    serving_size_unit: Option<Box<str>>,
    household_serving_fulltext: Option<Box<str>>,
    branded_food_category: Option<Box<str>>,
    data_source: Box<str>,
    package_weight: Option<Box<str>>,
    modified_date: Option<CsvDate>,
    available_date: CsvDate,
    market_country: Box<str>,
    discontinued_date: Option<CsvDate>,
    preparation_state_code: Option<Box<str>>,
    trade_channel: Option<Box<str>>,
    short_description: Option<Box<str>>,
    material_code: Option<u32>,
}

fn acquisition_sample_from_csv_row((fdc_id_of_sample_food, fdc_id_of_acquisition_food): AcquisitionSampleCsvRow) -> AcquisitionSampleRow {
    ((fdc_id_of_sample_food, fdc_id_of_acquisition_food), AcquisitionSample {})
}

fn agricultural_sample_from_csv_row((fdc_id, acquisition_date, market_class, treatment, state): AgriculturalSampleCsvRow) -> AgriculturalSampleRow {
    (
        fdc_id,
        AgriculturalSample {
            acquisition_date: Date::from(acquisition_date),
            market_class,
            treatment,
            state,
        },
    )
}

fn branded_food_from_csv_row(row: BrandedFoodCsv) -> BrandedFoodRow {
    (
        row.fdc_id,
        BrandedFood {
            brand_owner: row.brand_owner,
            brand_name: row.brand_name,
            subbrand_name: row.subbrand_name,
            gtin_upc: row.gtin_upc,
            ingredients: row.ingredients,
            not_a_significant_source_of: row.not_a_significant_source_of,
            serving_size: row.serving_size,
            serving_size_unit: row.serving_size_unit,
            household_serving_fulltext: row.household_serving_fulltext,
            branded_food_category: row.branded_food_category,
            data_source: row.data_source,
            package_weight: row.package_weight,
            modified_date: row.modified_date.map(Date::from),
            available_date: Date::from(row.available_date),
            market_country: row.market_country,
            discontinued_date: row.discontinued_date.map(Date::from),
            preparation_state_code: row.preparation_state_code,
            trade_channel: row.trade_channel,
            short_description: row.short_description,
            material_code: row.material_code,
        },
    )
}

fn fndds_derivation_from_csv_row((derivation_code, derivation_description): FnddsDerivationCsvRow) -> FnddsDerivationRow {
    (
        derivation_code,
        FnddsDerivation {
            derivation_description,
        },
    )
}

fn fndds_ingredient_nutrient_value_from_csv_row((ingredient_code, ingredient_description, nutrient_code, nutrient_value, nutrient_value_source, fdc_reference, derivation_code, sr_add_mod_year, foundation_year_acquired, start_date, end_date): FnddsIngredientNutrientValueCsvRow) -> FnddsIngredientNutrientValueRow {
    (
        (ingredient_code, nutrient_code, fdc_reference, Date::from(start_date), Date::from(end_date)),
        FnddsIngredientNutrientValue {
            ingredient_description,
            nutrient_value,
            nutrient_value_source,
            derivation_code,
            sr_add_mod_year,
            foundation_year_acquired,
        },
    )
}

fn food_from_csv_row((fdc_id, data_type, description, food_category_source, publication_date): FoodCsvRow) -> FoodRow {
    let (food_category_id, food_category) = food_category_fields(&data_type, food_category_source);
    (
        fdc_id,
        Food {
            data_type,
            description,
            food_category_id,
            food_category,
            publication_date: Date::from(publication_date),
        },
    )
}

fn food_category_fields(data_type: &FoodDataType, food_category_source: Option<Box<str>>) -> (Option<FoodCategoryKey>, Option<FoodCategoryValue>) {
    use FoodCategoryValue::*;
    use FoodDataType::*;
    match food_category_source {
        None => (None, None),
        Some(value) => match data_type {
            BrandedFood => (None, Some(BrandedFoodCategory(value))),
            SurveyFnddsFood => match parse_wweia_food_category_key(value) {
                Ok(value) => (None, Some(WweiaFoodCategory(value))),
                Err(value) => (None, Some(Unrecognized(value))),
            },
            Unknown(_) => (None, Some(Unrecognized(value))),
            AgriculturalAcquisition | ExperimentalFood | FoundationFood | MarketAcquisition | SampleFood | SrLegacyFood | SubSampleFood => match parse_food_category_key(value) {
                Ok(value) => (Some(value), None),
                Err(value) => (None, Some(Unrecognized(value))),
            },
        },
    }
}

fn parse_food_category_key(value: Box<str>) -> Result<FoodCategoryKey, Box<str>> {
    match value.parse::<FoodCategoryKey>() {
        Ok(value) => Ok(value),
        Err(_) => Err(value),
    }
}

fn parse_wweia_food_category_key(value: Box<str>) -> Result<WweiaFoodCategoryKey, Box<str>> {
    match value.parse::<WweiaFoodCategoryKey>() {
        Ok(value) => Ok(value),
        Err(_) => Err(value),
    }
}

fn food_attribute_from_csv_row((id, fdc_id, seq_num, food_attribute_type_id, name, value): FoodAttributeCsvRow) -> FoodAttributeRow {
    (
        id,
        FoodAttribute {
            fdc_id,
            seq_num,
            food_attribute_type_id,
            name,
            value,
        },
    )
}

fn food_attribute_type_from_csv_row((id, name, description): FoodAttributeTypeCsvRow) -> FoodAttributeTypeRow {
    (
        id,
        FoodAttributeType {
            name,
            description,
        },
    )
}

fn food_calorie_conversion_factor_from_csv_row((food_nutrient_conversion_factor_id, protein_value, fat_value, carbohydrate_value): FoodCalorieConversionFactorCsvRow) -> FoodCalorieConversionFactorRow {
    (
        food_nutrient_conversion_factor_id,
        FoodCalorieConversionFactor {
            protein_value,
            fat_value,
            carbohydrate_value,
        },
    )
}

fn food_category_from_csv_row((id, code, description): FoodCategoryCsvRow) -> FoodCategoryRow {
    (
        id,
        FoodCategory {
            code,
            description,
        },
    )
}

fn food_component_from_csv_row((id, fdc_id, name, pct_weight, is_refuse, gram_weight, data_points, min_year_acquired): FoodComponentCsvRow) -> FoodComponentRow {
    (
        id,
        FoodComponent {
            fdc_id,
            name,
            pct_weight,
            is_refuse: bool::from(is_refuse),
            gram_weight,
            data_points,
            min_year_acquired,
        },
    )
}

fn food_nutrient_from_csv_row((id, fdc_id, nutrient_id, amount, data_points, derivation_id, min, max, median, loq, footnote, min_year_acquired, percent_daily_value): FoodNutrientCsvRow) -> FoodNutrientRow {
    (
        id,
        FoodNutrient {
            fdc_id,
            nutrient_id,
            amount,
            data_points,
            derivation_id,
            min,
            max,
            median,
            loq,
            footnote,
            min_year_acquired,
            percent_daily_value,
        },
    )
}

fn food_nutrient_conversion_factor_from_csv_row((id, fdc_id): FoodNutrientConversionFactorCsvRow) -> FoodNutrientConversionFactorRow {
    (
        id,
        FoodNutrientConversionFactor {
            fdc_id,
        },
    )
}

fn food_nutrient_derivation_from_csv_row((id, code, description): FoodNutrientDerivationCsvRow) -> FoodNutrientDerivationRow {
    (
        id,
        FoodNutrientDerivation {
            code,
            description,
        },
    )
}

fn food_nutrient_source_from_csv_row((id, code, description): FoodNutrientSourceCsvRow) -> FoodNutrientSourceRow {
    (
        id,
        FoodNutrientSource {
            code,
            description,
        },
    )
}

fn food_portion_from_csv_row((id, fdc_id, seq_num, amount, measure_unit_id, portion_description, modifier, gram_weight, data_points, footnote, min_year_acquired): FoodPortionCsvRow) -> FoodPortionRow {
    (
        id,
        FoodPortion {
            fdc_id,
            seq_num,
            amount,
            measure_unit_id,
            portion_description,
            modifier,
            gram_weight,
            data_points,
            footnote,
            min_year_acquired,
        },
    )
}

fn food_protein_conversion_factor_from_csv_row((food_nutrient_conversion_factor_id, value): FoodProteinConversionFactorCsvRow) -> FoodProteinConversionFactorRow {
    (
        food_nutrient_conversion_factor_id,
        FoodProteinConversionFactor {
            value,
        },
    )
}

fn food_update_log_entry_from_csv_row((id, description, last_updated): FoodUpdateLogEntryCsvRow) -> FoodUpdateLogEntryRow {
    (
        id,
        FoodUpdateLogEntry {
            description,
            last_updated: Date::from(last_updated),
        },
    )
}

fn foundation_food_from_csv_row((fdc_id, ndb_number, footnote): FoundationFoodCsvRow) -> FoundationFoodRow {
    (
        fdc_id,
        FoundationFood {
            ndb_number,
            footnote,
        },
    )
}

fn input_food_from_csv_row((id, fdc_id, fdc_id_of_input_food, seq_num, amount, sr_code, sr_description, unit, portion_code, portion_description, gram_weight, retention_code): InputFoodCsvRow) -> InputFoodRow {
    (
        id,
        InputFood {
            fdc_id,
            fdc_id_of_input_food,
            seq_num,
            amount,
            sr_code,
            sr_description,
            unit,
            portion_code,
            portion_description,
            gram_weight,
            retention_code: retention_factor_key(retention_code),
        },
    )
}

fn retention_factor_key(retention_code: RetentionFactorKey) -> Option<RetentionFactorKey> {
    if retention_code == 0 { None } else { Some(retention_code) }
}

fn lab_method_from_csv_row((id, description, technique): LabMethodCsvRow) -> LabMethodRow {
    (
        id,
        LabMethod {
            description,
            technique,
        },
    )
}

fn lab_method_code_from_csv_row((lab_method_id, code): LabMethodCodeCsvRow) -> LabMethodCodeRow {
    ((lab_method_id, code), LabMethodCode {})
}

fn lab_method_nutrient_from_csv_row((lab_method_id, nutrient_id): LabMethodNutrientCsvRow) -> LabMethodNutrientRow {
    ((lab_method_id, nutrient_id), LabMethodNutrient {})
}

fn market_acquisition_from_csv_row((fdc_id, brand_description, expiration_date, label_weight, location, acquisition_date, sales_type, sample_lot_nbr, sell_by_date, store_city, store_name, store_state, upc_code, acquisition_number): MarketAcquisitionCsvRow) -> MarketAcquisitionRow {
    (
        fdc_id,
        MarketAcquisition {
            brand_description,
            expiration_date: expiration_date.map(Date::from),
            label_weight,
            location,
            acquisition_date: acquisition_date.map(Date::from),
            sales_type,
            sample_lot_nbr,
            sell_by_date: sell_by_date.map(Date::from),
            store_city,
            store_name,
            store_state,
            upc_code,
            acquisition_number,
        },
    )
}

fn measure_unit_from_csv_row((id, name): MeasureUnitCsvRow) -> MeasureUnitRow {
    (
        id,
        MeasureUnit {
            name,
        },
    )
}

fn microbe_from_csv_row((id, fdc_id, method, microbe_code, min_value, max_value, uom): MicrobeCsvRow) -> MicrobeRow {
    (
        id,
        Microbe {
            fdc_id,
            method,
            microbe_code,
            min_value,
            max_value,
            uom,
        },
    )
}

fn nutrient_from_csv_row((id, name, unit_name, nutrient_nbr, rank): NutrientCsvRow) -> NutrientRow {
    (
        id,
        Nutrient {
            name,
            unit_name,
            nutrient_nbr,
            rank,
        },
    )
}

/// PRUNING: drops `retention_factor.n.gid` because FDC retention factors are referenced by `n.code`.
fn retention_factor_from_csv_row((_gid, code, food_group_id, description): RetentionFactorCsvRow) -> RetentionFactorRow {
    (
        code,
        RetentionFactor {
            food_group_id,
            description,
        },
    )
}

fn sample_food_from_csv_row((fdc_id,): SampleFoodCsvRow) -> SampleFoodRow {
    (fdc_id, SampleFood {})
}

fn sr_legacy_food_from_csv_row((fdc_id, ndb_number): SrLegacyFoodCsvRow) -> SrLegacyFoodRow {
    (
        fdc_id,
        SrLegacyFood {
            ndb_number,
        },
    )
}

fn sub_sample_food_from_csv_row((fdc_id, fdc_id_of_sample_food): SubSampleFoodCsvRow) -> SubSampleFoodRow {
    (
        fdc_id,
        SubSampleFood {
            fdc_id_of_sample_food,
        },
    )
}

fn sub_sample_result_from_csv_row((food_nutrient_id, adjusted_amount, lab_method_id, nutrient_name): SubSampleResultCsvRow) -> SubSampleResultRow {
    (
        food_nutrient_id,
        SubSampleResult {
            adjusted_amount,
            lab_method_id,
            nutrient_name,
        },
    )
}

fn survey_fndds_food_from_csv_row((fdc_id, food_code, wweia_category_code, start_date, end_date): SurveyFnddsFoodCsvRow) -> SurveyFnddsFoodRow {
    (
        fdc_id,
        SurveyFnddsFood {
            food_code,
            wweia_category_code,
            start_date: Date::from(start_date),
            end_date: Date::from(end_date),
        },
    )
}

fn wweia_food_category_from_csv_row((wweia_food_category, wweia_food_category_description): WweiaFoodCategoryCsvRow) -> WweiaFoodCategoryRow {
    (
        wweia_food_category,
        WweiaFoodCategory {
            wweia_food_category_description,
        },
    )
}

define_iter!(iter_acquisition_samples, "acquisition_samples.csv", AcquisitionSampleCsvRow, AcquisitionSampleRow, acquisition_sample_from_csv_row, IterAcquisitionSamplesError);
define_iter!(iter_agricultural_samples, "agricultural_samples.csv", AgriculturalSampleCsvRow, AgriculturalSampleRow, agricultural_sample_from_csv_row, IterAgriculturalSamplesError);
define_iter!(iter_branded_food, "branded_food.csv", BrandedFoodCsv, BrandedFoodRow, branded_food_from_csv_row, IterBrandedFoodError);
define_iter!(iter_fndds_derivation, "fndds_derivation.csv", FnddsDerivationCsvRow, FnddsDerivationRow, fndds_derivation_from_csv_row, IterFnddsDerivationError);
define_iter!(iter_fndds_ingredient_nutrient_value, "fndds_ingredient_nutrient_value.csv", FnddsIngredientNutrientValueCsvRow, FnddsIngredientNutrientValueRow, fndds_ingredient_nutrient_value_from_csv_row, IterFnddsIngredientNutrientValueError);
define_iter!(iter_food, "food.csv", FoodCsvRow, FoodRow, food_from_csv_row, IterFoodError);
define_iter!(iter_food_attribute, "food_attribute.csv", FoodAttributeCsvRow, FoodAttributeRow, food_attribute_from_csv_row, IterFoodAttributeError);
define_iter!(iter_food_attribute_type, "food_attribute_type.csv", FoodAttributeTypeCsvRow, FoodAttributeTypeRow, food_attribute_type_from_csv_row, IterFoodAttributeTypeError);
define_iter!(iter_food_calorie_conversion_factor, "food_calorie_conversion_factor.csv", FoodCalorieConversionFactorCsvRow, FoodCalorieConversionFactorRow, food_calorie_conversion_factor_from_csv_row, IterFoodCalorieConversionFactorError);
define_iter!(iter_food_category, "food_category.csv", FoodCategoryCsvRow, FoodCategoryRow, food_category_from_csv_row, IterFoodCategoryError);
define_iter!(iter_food_component, "food_component.csv", FoodComponentCsvRow, FoodComponentRow, food_component_from_csv_row, IterFoodComponentError);
define_iter!(iter_food_nutrient, "food_nutrient.csv", FoodNutrientCsvRow, FoodNutrientRow, food_nutrient_from_csv_row, IterFoodNutrientError);
define_iter!(iter_food_nutrient_conversion_factor, "food_nutrient_conversion_factor.csv", FoodNutrientConversionFactorCsvRow, FoodNutrientConversionFactorRow, food_nutrient_conversion_factor_from_csv_row, IterFoodNutrientConversionFactorError);
define_iter!(iter_food_nutrient_derivation, "food_nutrient_derivation.csv", FoodNutrientDerivationCsvRow, FoodNutrientDerivationRow, food_nutrient_derivation_from_csv_row, IterFoodNutrientDerivationError);
define_iter!(iter_food_nutrient_source, "food_nutrient_source.csv", FoodNutrientSourceCsvRow, FoodNutrientSourceRow, food_nutrient_source_from_csv_row, IterFoodNutrientSourceError);
define_iter!(iter_food_portion, "food_portion.csv", FoodPortionCsvRow, FoodPortionRow, food_portion_from_csv_row, IterFoodPortionError);
define_iter!(iter_food_protein_conversion_factor, "food_protein_conversion_factor.csv", FoodProteinConversionFactorCsvRow, FoodProteinConversionFactorRow, food_protein_conversion_factor_from_csv_row, IterFoodProteinConversionFactorError);
define_iter!(iter_food_update_log_entry, "food_update_log_entry.csv", FoodUpdateLogEntryCsvRow, FoodUpdateLogEntryRow, food_update_log_entry_from_csv_row, IterFoodUpdateLogEntryError);
define_iter!(iter_foundation_food, "foundation_food.csv", FoundationFoodCsvRow, FoundationFoodRow, foundation_food_from_csv_row, IterFoundationFoodError);
define_iter!(iter_input_food, "input_food.csv", InputFoodCsvRow, InputFoodRow, input_food_from_csv_row, IterInputFoodError);
define_iter!(iter_lab_method, "lab_method.csv", LabMethodCsvRow, LabMethodRow, lab_method_from_csv_row, IterLabMethodError);
define_iter!(iter_lab_method_code, "lab_method_code.csv", LabMethodCodeCsvRow, LabMethodCodeRow, lab_method_code_from_csv_row, IterLabMethodCodeError);
define_iter!(iter_lab_method_nutrient, "lab_method_nutrient.csv", LabMethodNutrientCsvRow, LabMethodNutrientRow, lab_method_nutrient_from_csv_row, IterLabMethodNutrientError);
define_iter!(iter_market_acquisition, "market_acquisition.csv", MarketAcquisitionCsvRow, MarketAcquisitionRow, market_acquisition_from_csv_row, IterMarketAcquisitionError);
define_iter!(iter_measure_unit, "measure_unit.csv", MeasureUnitCsvRow, MeasureUnitRow, measure_unit_from_csv_row, IterMeasureUnitError);
define_iter!(iter_microbe, "microbe.csv", MicrobeCsvRow, MicrobeRow, microbe_from_csv_row, IterMicrobeError);
define_iter!(iter_nutrient, "nutrient.csv", NutrientCsvRow, NutrientRow, nutrient_from_csv_row, IterNutrientError);
define_iter!(iter_retention_factor, "retention_factor.csv", RetentionFactorCsvRow, RetentionFactorRow, retention_factor_from_csv_row, IterRetentionFactorError);
define_iter!(iter_sample_food, "sample_food.csv", SampleFoodCsvRow, SampleFoodRow, sample_food_from_csv_row, IterSampleFoodError);
define_iter!(iter_sr_legacy_food, "sr_legacy_food.csv", SrLegacyFoodCsvRow, SrLegacyFoodRow, sr_legacy_food_from_csv_row, IterSrLegacyFoodError);
define_iter!(iter_sub_sample_food, "sub_sample_food.csv", SubSampleFoodCsvRow, SubSampleFoodRow, sub_sample_food_from_csv_row, IterSubSampleFoodError);
define_iter!(iter_sub_sample_result, "sub_sample_result.csv", SubSampleResultCsvRow, SubSampleResultRow, sub_sample_result_from_csv_row, IterSubSampleResultError);
define_iter!(iter_survey_fndds_food, "survey_fndds_food.csv", SurveyFnddsFoodCsvRow, SurveyFnddsFoodRow, survey_fndds_food_from_csv_row, IterSurveyFnddsFoodError);
define_iter!(iter_wweia_food_category, "wweia_food_category.csv", WweiaFoodCategoryCsvRow, WweiaFoodCategoryRow, wweia_food_category_from_csv_row, IterWweiaFoodCategoryError);

impl TryFrom<&Path> for Database {
    type Error = TryFromPathForDatabaseError;

    fn try_from(dir: &Path) -> Result<Self, Self::Error> {
        use TryFromPathForDatabaseError::*;
        let acquisition_samples = handle!(collect_upserted_rows(handle!(Self::iter_acquisition_samples(dir), IterAcquisitionSamplesFailed)), CollectAcquisitionSamplesFailed);
        let agricultural_samples = handle!(collect_upserted_rows(handle!(Self::iter_agricultural_samples(dir), IterAgriculturalSamplesFailed)), CollectAgriculturalSamplesFailed);
        let branded_foods = handle!(collect_upserted_rows(handle!(Self::iter_branded_food(dir), IterBrandedFoodFailed)), CollectBrandedFoodFailed);
        let fndds_derivations = handle!(collect_upserted_rows(handle!(Self::iter_fndds_derivation(dir), IterFnddsDerivationFailed)), CollectFnddsDerivationFailed);
        let fndds_ingredient_nutrient_values = handle!(collect_upserted_rows(handle!(Self::iter_fndds_ingredient_nutrient_value(dir), IterFnddsIngredientNutrientValueFailed)), CollectFnddsIngredientNutrientValueFailed);
        let foods = handle!(collect_upserted_rows(handle!(Self::iter_food(dir), IterFoodFailed)), CollectFoodFailed);
        let food_attributes = handle!(collect_upserted_rows(handle!(Self::iter_food_attribute(dir), IterFoodAttributeFailed)), CollectFoodAttributeFailed);
        let food_attribute_types = handle!(collect_upserted_rows(handle!(Self::iter_food_attribute_type(dir), IterFoodAttributeTypeFailed)), CollectFoodAttributeTypeFailed);
        let food_calorie_conversion_factors = handle!(collect_upserted_rows(handle!(Self::iter_food_calorie_conversion_factor(dir), IterFoodCalorieConversionFactorFailed)), CollectFoodCalorieConversionFactorFailed);
        let food_categories = handle!(collect_upserted_rows(handle!(Self::iter_food_category(dir), IterFoodCategoryFailed)), CollectFoodCategoryFailed);
        let food_components = handle!(collect_upserted_rows(handle!(Self::iter_food_component(dir), IterFoodComponentFailed)), CollectFoodComponentFailed);
        let food_nutrients = handle!(collect_upserted_rows(handle!(Self::iter_food_nutrient(dir), IterFoodNutrientFailed)), CollectFoodNutrientFailed);
        let food_nutrient_conversion_factors = handle!(collect_upserted_rows(handle!(Self::iter_food_nutrient_conversion_factor(dir), IterFoodNutrientConversionFactorFailed)), CollectFoodNutrientConversionFactorFailed);
        let food_nutrient_derivations = handle!(collect_upserted_rows(handle!(Self::iter_food_nutrient_derivation(dir), IterFoodNutrientDerivationFailed)), CollectFoodNutrientDerivationFailed);
        let food_nutrient_sources = handle!(collect_upserted_rows(handle!(Self::iter_food_nutrient_source(dir), IterFoodNutrientSourceFailed)), CollectFoodNutrientSourceFailed);
        let food_portions = handle!(collect_upserted_rows(handle!(Self::iter_food_portion(dir), IterFoodPortionFailed)), CollectFoodPortionFailed);
        let food_protein_conversion_factors = handle!(collect_upserted_rows(handle!(Self::iter_food_protein_conversion_factor(dir), IterFoodProteinConversionFactorFailed)), CollectFoodProteinConversionFactorFailed);
        let food_update_log_entries = handle!(collect_upserted_rows(handle!(Self::iter_food_update_log_entry(dir), IterFoodUpdateLogEntryFailed)), CollectFoodUpdateLogEntryFailed);
        let foundation_foods = handle!(collect_upserted_rows(handle!(Self::iter_foundation_food(dir), IterFoundationFoodFailed)), CollectFoundationFoodFailed);
        let input_foods = handle!(collect_upserted_rows(handle!(Self::iter_input_food(dir), IterInputFoodFailed)), CollectInputFoodFailed);
        let lab_methods = handle!(collect_upserted_rows(handle!(Self::iter_lab_method(dir), IterLabMethodFailed)), CollectLabMethodFailed);
        let lab_method_codes = handle!(collect_upserted_rows(handle!(Self::iter_lab_method_code(dir), IterLabMethodCodeFailed)), CollectLabMethodCodeFailed);
        let lab_method_nutrients = handle!(collect_upserted_rows(handle!(Self::iter_lab_method_nutrient(dir), IterLabMethodNutrientFailed)), CollectLabMethodNutrientFailed);
        let market_acquisitions = handle!(collect_upserted_rows(handle!(Self::iter_market_acquisition(dir), IterMarketAcquisitionFailed)), CollectMarketAcquisitionFailed);
        let measure_units = handle!(collect_upserted_rows(handle!(Self::iter_measure_unit(dir), IterMeasureUnitFailed)), CollectMeasureUnitFailed);
        let microbes = handle!(collect_upserted_rows(handle!(Self::iter_microbe(dir), IterMicrobeFailed)), CollectMicrobeFailed);
        let nutrients = handle!(collect_upserted_rows(handle!(Self::iter_nutrient(dir), IterNutrientFailed)), CollectNutrientFailed);
        let retention_factors = handle!(collect_upserted_rows(handle!(Self::iter_retention_factor(dir), IterRetentionFactorFailed)), CollectRetentionFactorFailed);
        let sample_foods = handle!(collect_upserted_rows(handle!(Self::iter_sample_food(dir), IterSampleFoodFailed)), CollectSampleFoodFailed);
        let sr_legacy_foods = handle!(collect_upserted_rows(handle!(Self::iter_sr_legacy_food(dir), IterSrLegacyFoodFailed)), CollectSrLegacyFoodFailed);
        let sub_sample_foods = handle!(collect_upserted_rows(handle!(Self::iter_sub_sample_food(dir), IterSubSampleFoodFailed)), CollectSubSampleFoodFailed);
        let sub_sample_results = handle!(collect_upserted_rows(handle!(Self::iter_sub_sample_result(dir), IterSubSampleResultFailed)), CollectSubSampleResultFailed);
        let survey_fndds_foods = handle!(collect_upserted_rows(handle!(Self::iter_survey_fndds_food(dir), IterSurveyFnddsFoodFailed)), CollectSurveyFnddsFoodFailed);
        let wweia_food_categories = handle!(collect_upserted_rows(handle!(Self::iter_wweia_food_category(dir), IterWweiaFoodCategoryFailed)), CollectWweiaFoodCategoryFailed);
        Ok(Self {
            acquisition_samples,
            agricultural_samples,
            branded_food: branded_foods,
            fndds_derivation: fndds_derivations,
            fndds_ingredient_nutrient_value: fndds_ingredient_nutrient_values,
            food: foods,
            food_attribute: food_attributes,
            food_attribute_type: food_attribute_types,
            food_calorie_conversion_factor: food_calorie_conversion_factors,
            food_category: food_categories,
            food_component: food_components,
            food_nutrient: food_nutrients,
            food_nutrient_conversion_factor: food_nutrient_conversion_factors,
            food_nutrient_derivation: food_nutrient_derivations,
            food_nutrient_source: food_nutrient_sources,
            food_portion: food_portions,
            food_protein_conversion_factor: food_protein_conversion_factors,
            food_update_log_entry: food_update_log_entries,
            foundation_food: foundation_foods,
            input_food: input_foods,
            lab_method: lab_methods,
            lab_method_code: lab_method_codes,
            lab_method_nutrient: lab_method_nutrients,
            market_acquisition: market_acquisitions,
            measure_unit: measure_units,
            microbe: microbes,
            nutrient: nutrients,
            retention_factor: retention_factors,
            sample_food: sample_foods,
            sr_legacy_food: sr_legacy_foods,
            sub_sample_food: sub_sample_foods,
            sub_sample_result: sub_sample_results,
            survey_fndds_food: survey_fndds_foods,
            wweia_food_category: wweia_food_categories,
        })
    }
}

#[derive(Error, Debug)]
pub enum TryFromPathForDatabaseError {
    #[error("failed to iterate acquisition_samples.csv")]
    IterAcquisitionSamplesFailed { source: IterAcquisitionSamplesError },
    #[error("failed to collect acquisition_samples.csv")]
    CollectAcquisitionSamplesFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate agricultural_samples.csv")]
    IterAgriculturalSamplesFailed { source: IterAgriculturalSamplesError },
    #[error("failed to collect agricultural_samples.csv")]
    CollectAgriculturalSamplesFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate branded_food.csv")]
    IterBrandedFoodFailed { source: IterBrandedFoodError },
    #[error("failed to collect branded_food.csv")]
    CollectBrandedFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate fndds_derivation.csv")]
    IterFnddsDerivationFailed { source: IterFnddsDerivationError },
    #[error("failed to collect fndds_derivation.csv")]
    CollectFnddsDerivationFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate fndds_ingredient_nutrient_value.csv")]
    IterFnddsIngredientNutrientValueFailed { source: IterFnddsIngredientNutrientValueError },
    #[error("failed to collect fndds_ingredient_nutrient_value.csv")]
    CollectFnddsIngredientNutrientValueFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food.csv")]
    IterFoodFailed { source: IterFoodError },
    #[error("failed to collect food.csv")]
    CollectFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_attribute.csv")]
    IterFoodAttributeFailed { source: IterFoodAttributeError },
    #[error("failed to collect food_attribute.csv")]
    CollectFoodAttributeFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_attribute_type.csv")]
    IterFoodAttributeTypeFailed { source: IterFoodAttributeTypeError },
    #[error("failed to collect food_attribute_type.csv")]
    CollectFoodAttributeTypeFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_calorie_conversion_factor.csv")]
    IterFoodCalorieConversionFactorFailed { source: IterFoodCalorieConversionFactorError },
    #[error("failed to collect food_calorie_conversion_factor.csv")]
    CollectFoodCalorieConversionFactorFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_category.csv")]
    IterFoodCategoryFailed { source: IterFoodCategoryError },
    #[error("failed to collect food_category.csv")]
    CollectFoodCategoryFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_component.csv")]
    IterFoodComponentFailed { source: IterFoodComponentError },
    #[error("failed to collect food_component.csv")]
    CollectFoodComponentFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_nutrient.csv")]
    IterFoodNutrientFailed { source: IterFoodNutrientError },
    #[error("failed to collect food_nutrient.csv")]
    CollectFoodNutrientFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_nutrient_conversion_factor.csv")]
    IterFoodNutrientConversionFactorFailed { source: IterFoodNutrientConversionFactorError },
    #[error("failed to collect food_nutrient_conversion_factor.csv")]
    CollectFoodNutrientConversionFactorFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_nutrient_derivation.csv")]
    IterFoodNutrientDerivationFailed { source: IterFoodNutrientDerivationError },
    #[error("failed to collect food_nutrient_derivation.csv")]
    CollectFoodNutrientDerivationFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_nutrient_source.csv")]
    IterFoodNutrientSourceFailed { source: IterFoodNutrientSourceError },
    #[error("failed to collect food_nutrient_source.csv")]
    CollectFoodNutrientSourceFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_portion.csv")]
    IterFoodPortionFailed { source: IterFoodPortionError },
    #[error("failed to collect food_portion.csv")]
    CollectFoodPortionFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_protein_conversion_factor.csv")]
    IterFoodProteinConversionFactorFailed { source: IterFoodProteinConversionFactorError },
    #[error("failed to collect food_protein_conversion_factor.csv")]
    CollectFoodProteinConversionFactorFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate food_update_log_entry.csv")]
    IterFoodUpdateLogEntryFailed { source: IterFoodUpdateLogEntryError },
    #[error("failed to collect food_update_log_entry.csv")]
    CollectFoodUpdateLogEntryFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate foundation_food.csv")]
    IterFoundationFoodFailed { source: IterFoundationFoodError },
    #[error("failed to collect foundation_food.csv")]
    CollectFoundationFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate input_food.csv")]
    IterInputFoodFailed { source: IterInputFoodError },
    #[error("failed to collect input_food.csv")]
    CollectInputFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate lab_method.csv")]
    IterLabMethodFailed { source: IterLabMethodError },
    #[error("failed to collect lab_method.csv")]
    CollectLabMethodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate lab_method_code.csv")]
    IterLabMethodCodeFailed { source: IterLabMethodCodeError },
    #[error("failed to collect lab_method_code.csv")]
    CollectLabMethodCodeFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate lab_method_nutrient.csv")]
    IterLabMethodNutrientFailed { source: IterLabMethodNutrientError },
    #[error("failed to collect lab_method_nutrient.csv")]
    CollectLabMethodNutrientFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate market_acquisition.csv")]
    IterMarketAcquisitionFailed { source: IterMarketAcquisitionError },
    #[error("failed to collect market_acquisition.csv")]
    CollectMarketAcquisitionFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate measure_unit.csv")]
    IterMeasureUnitFailed { source: IterMeasureUnitError },
    #[error("failed to collect measure_unit.csv")]
    CollectMeasureUnitFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate microbe.csv")]
    IterMicrobeFailed { source: IterMicrobeError },
    #[error("failed to collect microbe.csv")]
    CollectMicrobeFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate nutrient.csv")]
    IterNutrientFailed { source: IterNutrientError },
    #[error("failed to collect nutrient.csv")]
    CollectNutrientFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate retention_factor.csv")]
    IterRetentionFactorFailed { source: IterRetentionFactorError },
    #[error("failed to collect retention_factor.csv")]
    CollectRetentionFactorFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate sample_food.csv")]
    IterSampleFoodFailed { source: IterSampleFoodError },
    #[error("failed to collect sample_food.csv")]
    CollectSampleFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate sr_legacy_food.csv")]
    IterSrLegacyFoodFailed { source: IterSrLegacyFoodError },
    #[error("failed to collect sr_legacy_food.csv")]
    CollectSrLegacyFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate sub_sample_food.csv")]
    IterSubSampleFoodFailed { source: IterSubSampleFoodError },
    #[error("failed to collect sub_sample_food.csv")]
    CollectSubSampleFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate sub_sample_result.csv")]
    IterSubSampleResultFailed { source: IterSubSampleResultError },
    #[error("failed to collect sub_sample_result.csv")]
    CollectSubSampleResultFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate survey_fndds_food.csv")]
    IterSurveyFnddsFoodFailed { source: IterSurveyFnddsFoodError },
    #[error("failed to collect survey_fndds_food.csv")]
    CollectSurveyFnddsFoodFailed { source: CollectUpsertedRowsError<CsvRowError> },
    #[error("failed to iterate wweia_food_category.csv")]
    IterWweiaFoodCategoryFailed { source: IterWweiaFoodCategoryError },
    #[error("failed to collect wweia_food_category.csv")]
    CollectWweiaFoodCategoryFailed { source: CollectUpsertedRowsError<CsvRowError> },
}

#[cfg(test)]
mod tests {
    use crate::{CountCsvDataRowsError, CountResultRowsError, CsvRowError, Database, count_csv_data_rows, count_result_rows};
    use errgonomic::handle;
    use std::error::Error;
    use std::path::Path;
    use thiserror::Error;

    macro_rules! check_table {
        ($dir:ident, $file_name:literal, $iter_name:ident) => {
            handle!(assert_table_count($dir, $file_name, Database::$iter_name($dir)), AssertTableCountFailed);
        };
    }

    #[test]
    fn must_import() -> Result<(), MustImportError> {
        use MustImportError::*;
        let dir = Path::new(".cache/FoodData_Central_csv_2026-04-30");
        check_table!(dir, "acquisition_samples.csv", iter_acquisition_samples);
        check_table!(dir, "agricultural_samples.csv", iter_agricultural_samples);
        check_table!(dir, "branded_food.csv", iter_branded_food);
        check_table!(dir, "fndds_derivation.csv", iter_fndds_derivation);
        check_table!(dir, "fndds_ingredient_nutrient_value.csv", iter_fndds_ingredient_nutrient_value);
        check_table!(dir, "food.csv", iter_food);
        check_table!(dir, "food_attribute.csv", iter_food_attribute);
        check_table!(dir, "food_attribute_type.csv", iter_food_attribute_type);
        check_table!(dir, "food_calorie_conversion_factor.csv", iter_food_calorie_conversion_factor);
        check_table!(dir, "food_category.csv", iter_food_category);
        check_table!(dir, "food_component.csv", iter_food_component);
        check_table!(dir, "food_nutrient.csv", iter_food_nutrient);
        check_table!(dir, "food_nutrient_conversion_factor.csv", iter_food_nutrient_conversion_factor);
        check_table!(dir, "food_nutrient_derivation.csv", iter_food_nutrient_derivation);
        check_table!(dir, "food_nutrient_source.csv", iter_food_nutrient_source);
        check_table!(dir, "food_portion.csv", iter_food_portion);
        check_table!(dir, "food_protein_conversion_factor.csv", iter_food_protein_conversion_factor);
        check_table!(dir, "food_update_log_entry.csv", iter_food_update_log_entry);
        check_table!(dir, "foundation_food.csv", iter_foundation_food);
        check_table!(dir, "input_food.csv", iter_input_food);
        check_table!(dir, "lab_method.csv", iter_lab_method);
        check_table!(dir, "lab_method_code.csv", iter_lab_method_code);
        check_table!(dir, "lab_method_nutrient.csv", iter_lab_method_nutrient);
        check_table!(dir, "market_acquisition.csv", iter_market_acquisition);
        check_table!(dir, "measure_unit.csv", iter_measure_unit);
        check_table!(dir, "microbe.csv", iter_microbe);
        check_table!(dir, "nutrient.csv", iter_nutrient);
        check_table!(dir, "retention_factor.csv", iter_retention_factor);
        check_table!(dir, "sample_food.csv", iter_sample_food);
        check_table!(dir, "sr_legacy_food.csv", iter_sr_legacy_food);
        check_table!(dir, "sub_sample_food.csv", iter_sub_sample_food);
        check_table!(dir, "sub_sample_result.csv", iter_sub_sample_result);
        check_table!(dir, "survey_fndds_food.csv", iter_survey_fndds_food);
        check_table!(dir, "wweia_food_category.csv", iter_wweia_food_category);
        Ok(())
    }

    fn assert_table_count<T, I, E>(dir: &Path, file_name: &'static str, iter_result: Result<I, E>) -> Result<(), AssertTableCountError>
    where
        I: IntoIterator<Item = Result<T, CsvRowError>>,
        E: Error + Send + Sync + 'static,
    {
        use AssertTableCountError::*;
        let iter = match iter_result {
            Ok(iter) => iter,
            Err(source) => {
                return Err(IterRowsFailed {
                    source: Box::new(source),
                    file_name: file_name.to_owned(),
                });
            }
        };
        let actual = handle!(
            count_result_rows(iter),
            CountResultRowsFailed,
            file_name: file_name.to_owned()
        );
        let expected = handle!(
            count_csv_data_rows(dir.join(file_name)),
            CountCsvDataRowsFailed,
            file_name: file_name.to_owned()
        );
        if actual != expected {
            return Err(RowCountMismatchInvalid {
                file_name: file_name.to_owned(),
                expected,
                actual,
            });
        }
        Ok(())
    }

    #[derive(Error, Debug)]
    enum MustImportError {
        #[error("failed to assert table count")]
        AssertTableCountFailed { source: AssertTableCountError },
    }

    #[derive(Error, Debug)]
    enum AssertTableCountError {
        #[error("failed to construct iterator for '{file_name}'")]
        IterRowsFailed { source: Box<dyn Error + Send + Sync>, file_name: String },
        #[error("failed to count imported rows for '{file_name}'")]
        CountResultRowsFailed { source: CountResultRowsError<CsvRowError>, file_name: String },
        #[error("failed to count CSV rows for '{file_name}'")]
        CountCsvDataRowsFailed { source: CountCsvDataRowsError, file_name: String },
        #[error("row count mismatch for '{file_name}': expected {expected}, actual {actual}")]
        RowCountMismatchInvalid { file_name: String, expected: u64, actual: u64 },
    }
}

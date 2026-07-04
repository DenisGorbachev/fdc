# FoodData Central Download Field Descriptions removals

These fields are removed because they are not present in current data dir.

## `food`

- `scientific_name`: The scientific name for the food
- `food_key`: A string of characters used to identify both the current and all historical records for a specific food
- `foodClass`: For internal use only

## `food_fat_conversion_factor`

- `food_nutrient_conversion_factor_id`: Id of the related row in the nutrient_conversion_factor table
- `fat_nlea_value`: The multiplication factor to convert from fat NLEA (298) to total fat (204)

## `food_nutrient`

- `standard_error`: Standard error

## `food_nutrient_derivation`

- `source_id`: ID of the nutrient source associated with the derivation

## `food_update_log_entry`

- `fdc_id`: ID of the food in the food table
- `publication_date`: Date when the food was published to FoodData Central

## `input_food`

- `survey_flag`: 2 means SR description does not match SR code; other values are internal FSRG processing codes

## `lab_method_code`

- `id`

## `lab_method_nutrient`

- `id`

## `measure_unit`

- `abbreviation`: abbreviated name of the unit

## `nutrient_incoming_name`

- `id`
- `name`: The name used for the incoming nutrient, e.g. if nutrient is Protein, name might be Prot
- `nutrient_id`: The id of the nutrient, in the nutrient file, related to the incoming name. Optional; see is_ignored for more info.

## `survey_fndds_food`

- `wweia_category_number`: Unique Identification number for WWEIA food category to which this food is assigned

## `wweia_food_category`

- `wweia_food_category_code`: Unique identification code

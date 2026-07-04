# FoodData Central Download Field Descriptions

## `agricultural_acquisition`

Non-processed foods obtained directly from the location where they are produced

- `fdc_id`: ID of the food in the food table
- `acquisition_date`: The date this food was obtained
- `market_class`: The name of the specific kind of this food, e.g. "Pinto" for pinto beans
- `treatment`: Any special condition relevant to the production of this food - typically "drought" or "control"
- `state`: The state in which this food was produced

## `acquisition_sample`

Maps acquisitions to sample foods. Acquisitions may be blended, and one acquisition can be used in multiple sample foods.

- `fdc_id_of_sample_food`: ID of the sample food that uses the acquisitioned food
- `fdc_id_of_acquisition_food`: ID of the acquisitioned food used in the sample food

## `branded_food`

Foods whose nutrient values are typically obtained from food label data provided by food brand owners.

- `fdc_id`: ID of the food in the food table
- `brand_owner`: Brand owner for the food
- `gtin_upc`: GTIN or UPC code. Duplicate codes indicate product updates; use `food.publication_date` to pick the latest update.
- `ingredients`: The list of ingredients, as it appears on the product label
- `serving_size`: The amount of the serving size when expressed as gram or ml
- `serving_size_unit`: The unit used to express the serving size, gram or ml
- `household_serving_fulltext`: Amount and unit of serving size when expressed in household units
- `branded_food_category`: The category of the branded food, assigned by GDSN or Label Insight
- `data_source`: The source of the data for this food. GDSN for GS1 or LI for Label Insight.
- `modified_date`: This date reflects when the product data was last modified by the data provider, i.e. the manufacturer
- `available_date`: This is the date when the product record was available for inclusion in the database.
- `discontinued_date`: This is the date when the product was discontinued.
- `market_country`: The primary country where the product is marketed.

## `food`

Any substance consumed by humans for nutrition, taste and/or aroma

- `fdc_id`: Unique permanent identifier of the food
- `foodClass`: For internal use only
- `data_type`: Type of food data; see Files tab for possible values.
- `description`: Description of the food
- `food_category_id`: Id of the food category the food belongs to
- `publication_date`: Date when the food was published to FoodData Central
- `scientific_name`: The scientific name for the food
- `food_key`: A string of characters used to identify both the current and all historical records for a specific food.

## `food_attribute`

The value for a generic property of a food

- `id`
- `fdc_id`: ID of the food this food attribute pertains to
- `seq_num`: The order the attribute will be displayed on the released food.
- `food_attribute_type_id`: ID of the type of food attribute to which this value is associated for a specific food
- `name`: Name of food attribute
- `value`: The actual value of the attribute

## `food_attribute_type`

The list of supported attributes associated with a food

- `id`
- `name`: Name of the attribute associated with the food - should be displayable to users
- `description`: Description of the attribute

## `food_calorie_conversion_factor`

The multiplication factors to be used when calculating energy from macronutrients for a specific food

- `food_nutrient_conversion_factor_id`: ID of the related row in the nutrient_conversion_factor table
- `protein_value`: The multiplication factor for protein
- `fat_value`: The multiplication factor for fat
- `carbohydrate_value`: The multiplication factor for carbohydrates

## `food_category`

Foods of defined similarity

- `id`
- `code`: Food group code
- `description`: Description of the food group

## `food_component`

A constituent part of a food, e.g. bone is a component of meat

- `id`
- `fdc_id`: ID of the food this food component pertains to
- `name`: The kind of component, e.g. bone
- `pct_weight`: The weight of the component as a percentage of the total weight of the food
- `is_refuse`: Whether the component is refuse, i.e. not edible
- `gram_weight`: The weight of the component in grams
- `data_points`: The number of observations on which the measure is based
- `min_year_acquired`: Minimum purchase year of all acquisitions used to derive the component value

## `food_fat_conversion_factor`

Factor to calculate total lipid fat (204)

- `food_nutrient_conversion_factor_id`: Id of the related row in the nutrient_conversion_factor table
- `fat_nlea_value`: The multiplication factor to convert from fat NLEA (298) to total fat (204)

## `food_nutrient`

A nutrient value for a food

- `id`
- `fdc_id`: ID of the food this food nutrient pertains to
- `nutrient_id`: ID of the nutrient to which the food nutrient pertains
- `amount`: Amount of the nutrient per 100g of food. Specified in unit defined in the nutrient table.
- `data_points`: Number of observations on which the value is based
- `derivation_id`: ID of the food nutrient derivation technique used to derive the value
- `standard_error`: Standard error
- `min`: The minimum amount
- `max`: The maximum amount
- `median`: The median amount
- `footnote`: Notes unusual aspects of the food nutrient, e.g. why a value differs from expectations.
- `min_year_acquired`: Minimum purchase year of all acquisitions used to derive the nutrient value

## `food_nutrient_conversion_factor`

Top level type for all types of nutrient conversion factors. A separate row is stored for each of these 3 types of conversion factor.

- `id`
- `fdc_id`: ID of the food that this food nutrient conversion factor pertains to

## `food_nutrient_derivation`

Procedure indicating how a food nutrient value was obtained

- `id`
- `code`: Code used for the derivation, e.g. A means analytical
- `description`: Description of the derivation
- `source_id`: ID of the nutrient source associated with the derivation

## `food_nutrient_source`

An information source from which we can obtain food nutrient values

- `id`
- `code`: Code used for the source, e.g. 4 means calculated or imputed
- `description`: Description of the source

## `food_portion`

Discrete amount of food

- `id`
- `fdc_id`: ID of the food this food portion pertains to
- `seq_num`: The order the measure will be displayed on the released food.
- `amount`: Number of measure units, e.g. 3 for 3 tsp. For survey (FNDDS) foods, this is embedded in `portion_description` instead.
- `measure_unit_id`: Unit for the measure, e.g. tsp for 3 tsp. Uses '9999' when the food type does not use measure units, including SR legacy and survey (FNDDS) foods.
- `portion_description`: For foundation foods, adds measure specificity, e.g. 1 slice is 1/8 of a 14 inch pizza. For survey (FNDDS) foods, household portion description.
- `modifier`: For foundation foods, measure qualifier such as melted, crushed, or diced. For survey (FNDDS) foods, portion code. For SR legacy foods, measure description including unit and modifier, e.g. waffle round (4" dia).
- `gram_weight`: The weight of the measure in grams
- `data_points`: The number of observations on which the measure is based
- `footnote`: Public notes on unusual measure aspects, e.g. usage caveats or unexpected gram weight.
- `min_year_acquired`: Minimum purchase year of all acquisitions used to derive the measure value

## `food_protein_conversion_factor`

- `food_nutrient_conversion_factor_id`: Id of the related row in the nutrient_conversion_factor table
- `value`: The multiplication factor used to calculate protein from nitrogen

## `food_update_log_entry`

Historical record of an update of food data

- `fdc_id`: ID of the food in the food table
- `description`: Description of the food
- `publication_date`: Date when the food was published to FoodData Central

## `foundation_food`

Foods with nutrient and component values derived mainly by chemical analysis. Includes metadata such as sample counts, acquisition locations/dates, analytical methods, and sometimes cultivar, genotype, or production practices.

- `fdc_id`: ID of the food in the food table
- `NDB_number`: Unique number assigned for the food, different from fdc_id, assigned in SR
- `footnote`: Public notes on unusual aspects of the food.

## `input_food`

Ingredient food for survey (FNDDS) foods, or source food for foundation foods and their source foods.

- `id`
- `fdc_id`: fdc_id of the food that contains the input food
- `fdc_id_of_input_food`: fdc_id of the food that is the input food
- `seq_num`: The order in which to display the input food
- `amount`: The amount of the input food included within this food given in terms of unit
- `sr_code`: SR/NDB code of the ingredient food; used for survey (FNDDS) foods only
- `sr_description`: Description of the SR ingredient food; used for survey (FNDDS) foods only
- `unit`: Unit of measure for the input-food amount; used for survey (FNDDS) foods only
- `portion_code`: Portion-description code used to measure ingredient amount; used for survey (FNDDS) foods only
- `portion_description`: Portion description used to measure ingredient amount; used for survey (FNDDS) foods only
- `gram_weight`: The weight in grams of the input food
- `retention_code`: Processing code for nutrient-impacting treatment; used for survey (FNDDS) foods only
- `survey_flag`: 2 means SR description does not match SR code; other values are internal FSRG processing codes

## `lab_method`

A chemical procedure used to measure the amount of one or more nutrients in a food

- `id`
- `description`: Description of the lab method
- `technique`: General chemical analysis approach used by the lab method

## `lab_method_code`

A short, sometimes lab-specific, sequence of characters used to identify a lab method

- `id`
- `lab_method_id`: ID of the lab method the code refers to
- `code`: Value of the method code

## `lab_method_nutrient`

A nutrient whose amount can be measured by a lab method

- `id`
- `lab_method_id`: ID of the lab method the nutrient is measured by
- `nutrient_id`: ID of the nutrient that can be measured by the lab method

## `market_acquisition`

A food obtained for chemical analysis.

- `fdc_id`: ID of the food in the food table
- `brand_description`: Brand name description of the food
- `expiration_date`: Date the food will expire
- `label_weight`: The weight of the food per the product label
- `location`: The region in which the food was purchased, e.g. CA1
- `acquisition_date`: Date the food was purchased
- `sales_type`: The type of establishment in which the food was acquired, e.g. Retail Store, restaurant, farm, etc.
- `sample_lot_nbr`: The lot number of the food
- `sell_by_date`: Date the food should be sold by
- `store_city`: The city where the food was acquired
- `store_name`: The name of the store the food is purchased from
- `store_state`: The state where the food was acquired
- `upc_code`: UPC code for the food. Only applicable for retail products.

## `measure_unit`

units for measuring quantities of foods

- `id`
- `name`: name of the unit
- `abbreviation`: abbreviated name of the unit

## `nutrient`

The chemical constituent of a food, e.g. calcium, vitamin E, officially recognized as essential to human health

- `id`
- `name`: Name of the nutrient
- `unit_name`: The standard unit of measure for the nutrient, per 100g of food
- `nutrient_nbr`: A unique code identifying a nutrient or food constituent

## `nutrient_incoming_name`

A nutrient name used to identify a nutrient in incoming nutrient data

- `id`
- `name`: The name used for the incoming nutrient, e.g. if nutrient is Protein, name might be Prot
- `nutrient_id`: The id of the nutrient, in the nutrient file, related to the incoming name. Optional; see is_ignored for more info.

## `sample_food`

Representative sample of the food supply, created from one acquired food or a composite of multiple acquired foods.

- `fdc_id`: ID of the food in the food table

## `sr_legacy_food`

Foods from the April 2018 USDA National Nutrient Database for Standard Reference. Nutrient and component values come from chemical analysis and calculation.

- `fdc_id`: ID of the food in the food table
- `NDB_number`: Unique number assigned for final food, starts from the minimum number of 100,000

## `sub_sample_food`

A portion of a sample food used for the purpose of specific chemical analysis.

- `fdc_id`: ID of the food in the food table
- `fdc_id_of_sample_food`: ID of the sample food from which the sub sample originated

## `sub_sample_result`

The result of chemical analysis of a lab on a particular sub sample for a particular nutrient

- `food_nutrient_id`: Unique ID for row, same as the food_nutrient ID
- `adjusted_amount`: Amount after adjusting for unit
- `lab_method_id`: ID of the lab method used to measure the nutrient
- `nutrient_name`: The name of the nutrient as supplied by the lab

## `survey_fndds_food`

Foods measured by the What We Eat In America survey component of NHANES. Nutrient values are usually calculated from Branded and SR Legacy data.

- `fdc_id`: ID of the food in the food table
- `food_code`: A unique ID identifying the food within FNDDS
- `wweia_category_number`: Unique Identification number for WWEIA food category to which this food is assigned
- `start_date`: Start date indicates time period corresponding to WWEIA data
- `end_date`: End date indicates time period corresponding to WWEIA data

## `wweia_food_category`

Food categories for fndds

- `wweia_food_category_code`: Unique identification code
- `wweia_food_category_description`: Description for a WWEIA Category

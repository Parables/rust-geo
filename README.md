# Earth → Countries → States/Regions → Cities/Towns
A robust and memory-efficient Rust application for building a hierarchical geographical structure from GeoNames.org data. This tool processes large flat-file datasets in a single pass, optimizes memory usage, resolves geographical names, and outputs a structured JSON hierarchy.

## Usage
Use a json library capable of parsing large json files such as [simdjson](https://github.com/simdjson/simdjson). However, there is nothing stopping you from using native tools like PHP’s json_decode() as shown below:

### To get the list of countries, you can parse the hierarchy.json file and loop over the children array of the Earth object with the key: `6295630`
```php
<?php

$filePath = './output/hierarchy.json'; // Adjust path as needed
$jsonContent = file_get_contents($filePath);
$hierarchy = json_decode($jsonContent, true); // Decode as associative array
// The Earth's GeoName ID is 6295630
$earthId = "6295630";

foreach ($hierarchy[$earthId]['children'] as $child) {
 echo $child['name'] . "\n";
}
?>
```

```json
  "6295630": {
    "name": "Earth",
    "children": [
      {
        "id": 3041565,
        "name": "Andorra"
      },
      //  ...
      {
        "id": 2300660,
        "name": "Ghana"
      },
      //  ...
      {
        "id": 878675,
        "name": "Zimbabwe"
      }
    ]
  },
```


### Get a list of states/regions in a country
```php
<?php

$filePath = './output/hierarchy.json'; // Adjust path as needed
$jsonContent = file_get_contents($filePath);
$hierarchy = json_decode($jsonContent, true); // Decode as associative array

// Get the regions in Ghana: 2300660
$ghanaId = "6295630";

foreach ($hierarchy[$ghanaId]['children'] as $child) {
 echo $child['name'] . "\n";
}
?>
```

```json
  "2300660": {
    "name": "Ghana",
    "children": [
      {
        "id": 2297169,
        "name": "Northern Region"
      },
      {
        "id": 2294076,
        "name": "Western Region"
      },
      {
        "id": 2294291,
        "name": "Upper East Region"
      },
      {
        "id": 12105069,
        "name": "Bono"
      },
      {
        "id": 2301360,
        "name": "Eastern Region"
      },
      {
        "id": 12105075,
        "name": "Western North"
      },
      {
        "id": 2300569,
        "name": "Greater Accra Region"
      },
      {
        "id": 12105071,
        "name": "North East"
      },
      {
        "id": 2294234,
        "name": "Volta Region"
      },
      {
        "id": 12105074,
        "name": "Savannah"
      },
      {
        "id": 12105072,
        "name": "Ahafo"
      },
      {
        "id": 2294286,
        "name": "Upper West Region"
      },
      {
        "id": 12105073,
        "name": "Bono East"
      },
      {
        "id": 12105070,
        "name": "Oti"
      },
      {
        "id": 2304116,
        "name": "Ashanti Region"
      },
      {
        "id": 2302353,
        "name": "Central Region"
      }
    ]
  },
```



### Get the cities/towns in a state/region
```php
<?php

$filePath = './output/hierarchy.json'; // Adjust path as needed
$jsonContent = file_get_contents($filePath);
$hierarchy = json_decode($jsonContent, true); // Decode as associative array

// Get the cities in Western Region: 2294076
$westernRegionId = "2294076";

foreach ($hierarchy[$westernRegionId]['children'] as $child) {
 echo $child['name'] . "\n";
}
?>
```

```json
  "2294076": {
    "name": "Western Region",
    "children": [
      // ... list of cities and towns in Western Region of Ghana
    ],
  },
```

## ✨ Features
- Hierarchical Structure Generation: Constructs a flat geographical hierarchy (`Earth → Countries → States/Regions → Cities/Towns`) by inferring relationships from administrative codes within the allCountries.txt data.
- Flexible Name Resolution: Resolves geographical names by prioritizing:
  - Hardcoded exceptions (e.g., "United States of America").
  - English preferred/short names from alternateNamesV2.txt.
  - Primary names from allCountries.txt.
- JSON Output: Persists the generated hierarchy and any unparented geographical entities into human-readable JSON files.
- Self-Contained: Downloads and unzips necessary data files automatically, requiring no external database setup or pre-processing.

## 🚀 Getting Started
### Prerequisites
- Rust (version 1.70 or newer is recommended for OnceLock).
- Cargo (Rust's package manager, installed with Rust).

### Installation
- Clone the repository:
```sh
git clone [https://github.com/parables/rust-geo.git](https://github.com/parables/rust-geo.git)
cd rust-geo
```

- Build the project:
```sh
cargo build --release
# The --release flag builds an optimized executable.
```

- Running the Application:
After building, you can run the application from your project's root directory:
```sh
cargo run --release
```

The application will:
- Check for and create the `output/` directory in the project root directory.
- Download `allCountries.zip` and `alternateNamesV2.zip` from GeoNames.org.
- Unzip these files.
- Process the data and build the hierarchy.
- Output `hierarchy.json` (the main geographical tree) and `unparented_cities.json` (entities that couldn't be fully parented within the defined hierarchy) into the `output/` directory in the project root directory.

## 📁 Project Structure
`src/main.rs`: The main application logic, orchestrating file operations, data processing, hierarchy building, and output.
`src/models.rs`: Defines the data structures used, including the optimized GeoName struct and the ExtendedGeoNameEntry for the hierarchy.
`src/parsers.rs`: Contains functions for parsing lines from allCountries.txt and alternateNamesV2.txt into their respective Rust structs.
`Cargo.toml`: Project manifest and dependency management.

## 🌍 Data Source
This project utilizes geographical data from GeoNames.org data. Specifically, it downloads and processes:
`allCountries.zip`: Contains comprehensive geographical data for all countries.
`alternateNamesV2.zip`: Provides alternate names for geographical features, used for name resolution.

## 📦 Output Files
Upon successful execution, the following JSON files will be generated in the `output/` directory :
`hierarchy.json`: The primary output, representing the nested geographical hierarchy with Earth as the root.
`unparented_cities.json`: A list of geographical entities (primarily cities) that could not be assigned a parent within the defined hierarchy rules.

## 📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgements
- [GeoNames.org](https://download.geonames.org/export/dump/) for providing the invaluable geographical data.
- The Rust community for excellent tooling and documentation.
- [Google Gemini](https://gemini.google.com) and [Claude.ai](https://claude.ai/) for porting the [intial work](https://github.com/Parables/geo) from PHP to Rust

## Contributors:
- [Parables Boltnoel](https://github.com/Parables): Author
- Send in a PR to add your name here :-)


# Plant Database — Cohousing Bourgoyen Food Forest

Structured plant data for the communal garden redesign at Nekkerputstraat 176, Gent.
Intended as input for Rust-based analysis and visualization tools.

**Site climate**: Temperate oceanic (Köppen Cfb), USDA zone 8a/8b, 51.05°N.
**Design goal**: Permaculture food forest, northwest and northeast zones of the site.

---

## Schema Overview

```
species          ─┐
                  ├─ vendor_plants ─── vendors
cultivars        ─┘
```

Three core tables:
- **`species`** — botanical and ecological data per species (species-level, not cultivar)
- **`vendor_plants`** — what vendors sell, linking species + optional cultivar to a vendor listing
- **`vendors`** — nursery/supplier contact and metadata

---

## Table: `vendors`

| Field | Type | Description |
|-------|------|-------------|
| `id` | string (slug) | Unique identifier, e.g. `de_wit_nursery` |
| `name` | string | Display name |
| `url` | string | Website |
| `country` | string (ISO 3166-1 alpha-2) | e.g. `BE`, `NL`, `DE` |
| `language` | string (BCP 47) | Primary language of site, e.g. `nl`, `de` |
| `ships_to_be` | bool | Whether they ship to Belgium |
| `notes` | string | Free text |

### Vendor entries

| id | name | url | country | language | ships_to_be | notes |
|----|------|-----|---------|----------|-------------|-------|
| `de_groene_wijk` | De Groene Wijk | https://www.degroene wijk.be | BE | nl | true | Gespecialiseerd in permacultuur en voedselbossen |
| `de_bosrand` | De Bosrand | https://www.debosrand.be | BE | nl | true | Inheemse en eetbare planten |
| `food_forest_nursery` | Food Forest Nursery | https://www.foodforestnursery.co.uk | GB | en | false | UK specialist, may not ship to BE |
| `lathyrus` | Lathyrus | https://www.lathyrus.nl | NL | nl | true | Vaste planten en heesters |
| `raintree_nursery` | Raintree Nursery | https://www.raintreenursery.com | US | en | false | Reference only — US vendor |

---

## Table: `species`

### Field definitions

| Field | Type | Description |
|-------|------|-------------|
| `id` | string (slug) | Unique identifier, e.g. `malus_domestica` |
| `latin_name` | string | Binomial species name |
| `common_name_nl` | string | Dutch common name |
| `common_name_en` | string | English common name |
| `family` | string | Botanical family |
| `food_forest_layer` | enum | `canopy` / `sub_canopy` / `shrub` / `herbaceous` / `groundcover` / `root` / `climber` |
| `mature_height_m_min` | f32 | Minimum mature height in metres |
| `mature_height_m_max` | f32 | Maximum mature height in metres |
| `mature_spread_m_min` | f32 | Minimum canopy spread in metres |
| `mature_spread_m_max` | f32 | Maximum canopy spread in metres |
| `growth_rate` | enum | `slow` / `medium` / `fast` |
| `hardiness_zone_min` | u8 | Minimum USDA hardiness zone (site is zone 8) |
| `light` | enum | `full_sun` / `partial_shade` / `full_shade` |
| `soil_moisture` | enum | `dry` / `medium` / `moist` / `wet` |
| `soil_ph_min` | f32 | Minimum suitable soil pH |
| `soil_ph_max` | f32 | Maximum suitable soil pH |
| `nitrogen_fixer` | bool | Whether plant fixes atmospheric nitrogen |
| `edible_parts` | string[] | e.g. `["fruit", "leaf", "nut"]` |
| `wildlife_value` | string[] | e.g. `["pollinator", "bird_food", "hedgehog_habitat"]` |
| `root_type` | enum | `taproot` / `fibrous` / `spreading` |
| `notes` | string | Free text |

### Species entries

#### Canopy trees (layer: `canopy`)

| id | latin_name | common_name_nl | common_name_en | family | mature_height_m | mature_spread_m | growth_rate | hardiness_zone_min | light | soil_moisture | soil_ph | nitrogen_fixer | edible_parts | wildlife_value | notes |
|----|-----------|----------------|----------------|--------|-----------------|-----------------|-------------|-------------------|-------|--------------|---------|----------------|-------------|----------------|-------|
| `juglans_regia` | *Juglans regia* | Walnoot | Walnut | Juglandaceae | 15–25 | 10–15 | medium | 5 | full_sun | medium | 6.0–7.0 | false | [fruit, leaf_tea] | [bird_food, squirrel] | Allelopathic — keep away from sensitive plants; late to leaf out |
| `castanea_sativa` | *Castanea sativa* | Tamme kastanje | Sweet chestnut | Fagaceae | 20–35 | 10–15 | fast | 5 | full_sun | medium–dry | 4.5–6.5 | false | [nut] | [bird_food, squirrel, insect] | Excellent mast crop; prefers slightly acid soil |
| `quercus_robur` | *Quercus robur* | Zomereik | English oak | Fagaceae | 20–40 | 15–25 | slow | 5 | full_sun | medium–moist | 4.5–7.0 | false | [acorn_flour] | [insect, bird_food, bat] | Keystone species; acorns edible after leaching; very long-lived |

#### Sub-canopy trees (layer: `sub_canopy`)

| id | latin_name | common_name_nl | common_name_en | family | mature_height_m | mature_spread_m | growth_rate | hardiness_zone_min | light | soil_moisture | soil_ph | nitrogen_fixer | edible_parts | wildlife_value | notes |
|----|-----------|----------------|----------------|--------|-----------------|-----------------|-------------|-------------------|-------|--------------|---------|----------------|-------------|----------------|-------|
| `malus_domestica` | *Malus domestica* | Appel | Apple | Rosaceae | 3–8 | 3–8 | medium | 3 | full_sun | medium | 6.0–7.0 | false | [fruit] | [pollinator, bird_food] | Height/spread highly cultivar-dependent; see vendor_plants for cultivar details |
| `pyrus_communis` | *Pyrus communis* | Peer | Pear | Rosaceae | 4–12 | 3–6 | medium | 4 | full_sun | medium | 6.0–7.5 | false | [fruit] | [pollinator, bird_food] | Needs cross-pollination partner |
| `prunus_avium` | *Prunus avium* | Zoete kers | Sweet cherry | Rosaceae | 5–15 | 4–10 | fast | 4 | full_sun | medium | 6.0–7.5 | false | [fruit] | [bird_food, pollinator] | Vigorous; prefers well-drained soil |
| `prunus_domestica` | *Prunus domestica* | Pruim | Plum | Rosaceae | 3–8 | 3–6 | medium | 4 | full_sun | medium | 6.0–7.5 | false | [fruit] | [pollinator, bird_food] | Many cultivars; some self-fertile |
| `morus_alba` | *Morus alba* | Witte moerbei | White mulberry | Moraceae | 5–15 | 5–10 | fast | 5 | full_sun | medium | 5.5–7.0 | false | [fruit, leaf] | [bird_food, silkworm] | Very productive; fruit stains; wind-resistant |
| `morus_nigra` | *Morus nigra* | Zwarte moerbei | Black mulberry | Moraceae | 4–10 | 5–8 | slow | 6 | full_sun | medium | 5.5–7.0 | false | [fruit] | [bird_food] | Superior fruit flavour; slower than M. alba |
| `cydonia_oblonga` | *Cydonia oblonga* | Kweepeer | Quince | Rosaceae | 3–6 | 3–5 | medium | 5 | full_sun | medium–moist | 5.5–7.0 | false | [fruit] | [pollinator] | Self-fertile; fragrant flowers; fruit for jelly/jam |
| `ficus_carica` | *Ficus carica* | Vijg | Fig | Moraceae | 3–8 | 3–8 | medium | 7 | full_sun | medium–dry | 6.0–8.0 | false | [fruit, leaf] | [bird_food] | Zone 8 marginally hardy; wall-trained forms more reliable in Ghent |
| `elaeagnus_umbellata` | *Elaeagnus umbellata* | Herfstolief | Autumn olive | Elaeagnaceae | 3–6 | 3–5 | fast | 3 | full_sun | dry–medium | 5.5–7.5 | true | [fruit] | [pollinator, bird_food] | Nitrogen fixer; very productive; fruit slightly astringent when unripe |
| `amelanchier_lamarckii` | *Amelanchier lamarckii* | Krentenboompje | Juneberry | Rosaceae | 4–8 | 4–6 | medium | 4 | full_sun–partial_shade | medium | 5.5–7.0 | false | [fruit] | [pollinator, bird_food] | Beautiful spring blossom; tasty fruit; tolerates part shade well |

#### Shrubs (layer: `shrub`)

| id | latin_name | common_name_nl | common_name_en | family | mature_height_m | mature_spread_m | growth_rate | hardiness_zone_min | light | soil_moisture | soil_ph | nitrogen_fixer | edible_parts | wildlife_value | notes |
|----|-----------|----------------|----------------|--------|-----------------|-----------------|-------------|-------------------|-------|--------------|---------|----------------|-------------|----------------|-------|
| `ribes_nigrum` | *Ribes nigrum* | Zwarte bes | Blackcurrant | Grossulariaceae | 1.0–1.5 | 1.0–1.5 | medium | 3 | full_sun–partial_shade | moist | 5.5–7.0 | false | [fruit, leaf] | [pollinator, bird_food] | Tolerates partial shade; very productive |
| `ribes_rubrum` | *Ribes rubrum* | Rode bes | Redcurrant | Grossulariaceae | 1.0–1.5 | 1.0–1.5 | medium | 3 | full_sun–partial_shade | moist | 5.5–7.0 | false | [fruit] | [bird_food] | Tolerates more shade than blackcurrant |
| `ribes_uva_crispa` | *Ribes uva-crispa* | Kruisbes | Gooseberry | Grossulariaceae | 0.8–1.2 | 0.8–1.2 | medium | 3 | full_sun–partial_shade | medium | 5.5–7.0 | false | [fruit] | [pollinator, bird_food] | Thorny; fruit excellent |
| `sambucus_nigra` | *Sambucus nigra* | Vlier | Elder | Adoxaceae | 3–6 | 3–5 | fast | 4 | full_sun–partial_shade | moist | 5.5–7.5 | false | [flower, fruit, leaf] | [pollinator, bird_food] | Pioneer species; flowers for cordial/syrup; berries must be cooked |
| `corylus_avellana` | *Corylus avellana* | Hazelaar | Hazel | Betulaceae | 3–6 | 3–5 | medium | 4 | full_sun–partial_shade | medium | 5.5–7.0 | false | [nut] | [squirrel, dormouse, bird_food] | Wind-pollinated; plant 2+ for good nut set |
| `rubus_idaeus` | *Rubus idaeus* | Framboos | Raspberry | Rosaceae | 1.0–2.0 | 0.6–1.0 | fast | 3 | full_sun–partial_shade | moist | 5.5–6.5 | false | [fruit, leaf_tea] | [pollinator, bird_food] | Suckering; everbearing cultivars available |
| `rubus_fruticosus` | *Rubus fruticosus* | Braam | Blackberry | Rosaceae | 1.0–3.0 | 1.0–3.0 | fast | 5 | full_sun–partial_shade | medium–moist | 5.5–7.0 | false | [fruit, leaf_tea] | [pollinator, bird_food, hedgehog_habitat] | Thornless cultivars preferred for garden use |
| `vaccinium_corymbosum` | *Vaccinium corymbosum* | Blauwe bes | Highbush blueberry | Ericaceae | 1.5–2.5 | 1.0–2.0 | slow | 4 | full_sun | medium–moist | 4.5–5.5 | false | [fruit] | [pollinator, bird_food] | Needs acidic soil; plant 2+ for best yield; stunning autumn colour |
| `aronia_melanocarpa` | *Aronia melanocarpa* | Zwarte appelbes | Black chokeberry | Rosaceae | 1.5–3.0 | 1.5–2.5 | medium | 3 | full_sun–partial_shade | medium–moist | 5.0–7.0 | false | [fruit] | [pollinator, bird_food] | Very hardy; high antioxidants; astringent fresh but excellent processed |
| `hippophae_rhamnoides` | *Hippophaë rhamnoides* | Duindoorn | Sea buckthorn | Elaeagnaceae | 2–6 | 2–5 | fast | 3 | full_sun | dry–medium | 6.0–7.5 | true | [fruit, seed_oil] | [bird_food, insect] | Nitrogen fixer; thorny windbreak; plant male + female (ratio ~1:6) |

#### Herbaceous perennials (layer: `herbaceous`)

| id | latin_name | common_name_nl | common_name_en | family | mature_height_m | mature_spread_m | growth_rate | hardiness_zone_min | light | soil_moisture | soil_ph | nitrogen_fixer | edible_parts | wildlife_value | notes |
|----|-----------|----------------|----------------|--------|-----------------|-----------------|-------------|-------------------|-------|--------------|---------|----------------|-------------|----------------|-------|
| `symphytum_officinale` | *Symphytum officinale* | Smeerwortel | Comfrey | Boraginaceae | 0.6–1.2 | 0.6–1.0 | fast | 3 | full_sun–partial_shade | moist | 5.5–7.5 | false | [leaf_mulch, leaf_fertiliser] | [pollinator, bumblebee] | Dynamic accumulator; chop-and-drop mulch plant; Russian comfrey (S. × uplandicum) preferred (sterile) |
| `allium_ursinum` | *Allium ursinum* | Daslook | Wild garlic | Amaryllidaceae | 0.2–0.4 | 0.2–0.3 | medium | 4 | partial_shade–full_shade | moist | 6.0–7.5 | false | [leaf, flower, bulb] | [pollinator, bee] | Spring ephemeral; naturalises well under trees; all parts edible |
| `myrrhis_odorata` | *Myrrhis odorata* | Roomse kervel | Sweet cicely | Apiaceae | 0.6–1.2 | 0.6–0.9 | medium | 3 | partial_shade | moist | 5.5–7.5 | false | [leaf, seed, root] | [pollinator] | Anise flavour; deep taproot; early spring foliage; semi-evergreen |
| `levisticum_officinale` | *Levisticum officinale* | Lavas | Lovage | Apiaceae | 1.0–2.0 | 0.6–1.0 | fast | 4 | full_sun–partial_shade | moist | 6.0–7.5 | false | [leaf, stem, seed, root] | [pollinator] | Vigorous; celery-like flavour; all parts edible |

#### Groundcover (layer: `groundcover`)

| id | latin_name | common_name_nl | common_name_en | family | mature_height_m | mature_spread_m | growth_rate | hardiness_zone_min | light | soil_moisture | soil_ph | nitrogen_fixer | edible_parts | wildlife_value | notes |
|----|-----------|----------------|----------------|--------|-----------------|-----------------|-------------|-------------------|-------|--------------|---------|----------------|-------------|----------------|-------|
| `fragaria_vesca` | *Fragaria vesca* | Bosaardbei | Wild strawberry | Rosaceae | 0.1–0.2 | 0.2–0.4 | fast | 3 | full_sun–partial_shade | medium | 5.5–7.0 | false | [fruit, leaf_tea] | [pollinator, bird_food] | Excellent groundcover under trees; runners fill gaps |
| `trifolium_repens` | *Trifolium repens* | Witte klaver | White clover | Fabaceae | 0.1–0.2 | 0.3–0.5 | fast | 3 | full_sun–partial_shade | medium | 5.5–7.5 | true | [flower_tea, leaf] | [pollinator, bumblebee] | Nitrogen fixer; living mulch; suppresses weeds |
| `mentha_sp` | *Mentha* spp. | Munt | Mint | Lamiaceae | 0.3–0.6 | 0.3–1.0 | fast | 3 | full_sun–partial_shade | moist | 6.0–7.5 | false | [leaf] | [pollinator, bee] | Invasive — use in containers or constrained areas |

#### Climbers (layer: `climber`)

| id | latin_name | common_name_nl | common_name_en | family | mature_height_m | mature_spread_m | growth_rate | hardiness_zone_min | light | soil_moisture | soil_ph | nitrogen_fixer | edible_parts | wildlife_value | notes |
|----|-----------|----------------|----------------|--------|-----------------|-----------------|-------------|-------------------|-------|--------------|---------|----------------|-------------|----------------|-------|
| `vitis_vinifera` | *Vitis vinifera* | Druif | Grape vine | Vitaceae | 5–15 | 2–5 | fast | 6 | full_sun | medium–dry | 6.0–7.5 | false | [fruit, leaf] | [pollinator, bird_food] | Train on pergola or south-facing structure; table or wine grapes |
| `humulus_lupulus` | *Humulus lupulus* | Hop | Hop | Cannabaceae | 4–8 | 1–2 | fast | 3 | full_sun | medium–moist | 6.0–7.5 | false | [young_shoot, cone] | [pollinator] | Annual shoots back to roots each year; vigorous |
| `lonicera_periclymenum` | *Lonicera periclymenum* | Wilde kamperfoelie | Honeysuckle | Caprifoliaceae | 3–6 | 1–3 | medium | 4 | full_sun–partial_shade | medium | 5.5–7.5 | false | [] | [pollinator, bird_food, moth] | Native; very fragrant; berries for birds only (toxic to humans) |

---

## Table: `vendor_plants`

Links a vendor to a species (and optional cultivar), with commercial details.

### Field definitions

| Field | Type | Description |
|-------|------|-------------|
| `id` | string (slug) | Unique identifier, e.g. `fruitbomenhof_prunus_domestica_victoria` |
| `vendor_id` | string | Foreign key → `vendors.id` |
| `species_id` | string | Foreign key → `species.id` |
| `cultivar` | string (optional) | Cultivar name, e.g. `Jonagold`, `Cox Orange` |
| `product_name` | string | Name as listed by the vendor |
| `delivery_method` | enum | `bare_root` / `pot` / `rootball` / `air_pot` / `unknown` |
| `container_size_l` | f32 (optional) | Pot size in litres (for potted plants) |
| `height_at_sale_cm` | f32 (optional) | Plant height at time of listing in cm |
| `trunk_circumference_cm` | f32 (optional) | Trunk girth in cm (for large specimen trees) |
| `age_years` | u8 (optional) | Approximate age of the plant |
| `rootstock` | string (optional) | Rootstock code, e.g. `M9`, `M26`, `MM106`, `St. Julien A` |
| `price_eur` | f32 (optional) | Price in euros |
| `stock_status` | enum | `in_stock` / `limited` / `out_of_stock` / `unknown` |
| `url` | string (optional) | Direct product URL |
| `notes` | string | Free text |

### Vendor plant entries

*(To be populated as vendor catalogues are reviewed.
Add entries here as plants are sourced from specific suppliers.)*

| id | vendor_id | species_id | cultivar | product_name | container_size_l | rootstock | price_eur | stock_status | notes |
|----|-----------|-----------|----------|-------------|-----------------|-----------|-----------|-------------|-------|
| — | — | — | — | — | — | — | — | — | Add entries here |

---

## Flat Files

The canonical data lives in three TOML files under `data/`:

| File | Contents |
|------|----------|
| `data/species.toml` | All species entries (`[[species]]` array) |
| `data/vendors.toml` | All vendor entries (`[[vendor]]` array) |
| `data/vendor_plants.toml` | Vendor product listings (`[[vendor_plant]]` array) |

Edit those files directly. This document is the schema reference; the TOML files are the source of truth.

## Notes for Rust Integration

Recommended approach:

- Deserialize with `serde` + `toml` crate; each file maps to a `Vec<Species>` / `Vec<Vendor>` / `Vec<VendorPlant>`.
- Use `Option<f32>` for nullable numeric fields (`price_eur`, `container_size_l`).
- Enum fields (`light`, `soil_moisture`, `growth_rate`, etc.) map to Rust `enum` types derived with `serde`.
- `edible_parts` and `wildlife_value` are `Vec<String>`.
- Height and spread ranges: `mature_height_m_min` + `mature_height_m_max` as separate `f32` fields.

### Key derived quantities for solar/shadow simulation

For the Phase 5 food forest tree layer in the 3D solar model:

- **`mature_height_m_max`** → vertical extent of tree model
- **`mature_spread_m_max`** → canopy radius = spread / 2
- **`growth_rate`** → parameterise intermediate states (year 5, year 10, year 20)

---

## Change Log

| Date | Change |
|------|--------|
| 2026-04-18 | Initial schema and species list created |

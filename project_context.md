# Cohousing Bourgoyen — Garden Redesign: Project Context

## Goal

Redesign the communal garden as a **permaculture food forest** for the Cohousing Bourgoyen project at Nekkerputstraat 176, 9000 Gent, Belgium. The developer's landscape designer (buro Groen) produced a preliminary design that serves as the geometric baseline; the community wants to replace it with a food forest design.

A supporting task is to build a **dimensionally accurate 3D model** of the grounds, suitable for simulating solar exposure (by time of day and season) to inform plant placement decisions.

---

## Site

| Field | Value |
|-------|-------|
| Address | Nekkerputstraat 176, 9000 Gent |
| Cadastral reference | Gent, 16° afdeling, sectie K, 831/d – 842/z2 |
| Latitude / Longitude | ~51.05°N, 3.72°E |
| Client | Cohousing Projects BVBA, Sint-Markoenstraat 28, 9032 Gent |

---

## Buildings

### Villa Gustaaf (pre-existing mansion)
- Located at the center of the site.
- **Outer dimensions will not change** — it is a protected/heritage structure.
- This is the anchor from which all relative measurements should be made.

### New housing (under construction)
- A **U-shaped row of 8 private homes** is being built around Villa Gustaaf.
- Also includes the **gemeenschappelijke woning** (common house) — likely integrated into the U or adjacent.
- Other structures: fietsenstalling (bicycle storage), serre (greenhouse/conservatory), brievenbussen (mailboxes).

---

## Site Features — From Preliminary Design Plan

Source: `380_SO_231214-01_ontwerpplan groenburo.pdf` (AutoCAD LT 2021, 1:200 scale, dated 14/12/2023, phase: sketch design)

### Elevations (TAW — Belgian altitude datum)
| Feature | TAW elevation |
|---------|--------------|
| Ground level (site average) | ~+9.0 to +9.5 |
| Low point (south end of site) | ~+7.9 |
| Building ridge (nok) | +14.01 / +14.03 |
| Roof slabs / terraces (op platen) | +10.22 to +11.86 |
| Wall tops (op muur) | +9.42 to +12.77 |
| Wadi floor | +8.96 |
| Wadi max water level | +9.16 |

Buildings rise approximately **4.5–5 m above ground level** (ridge at +14.0, ground at +9.0–9.5).

### Water management
The preliminary design includes two water infiltration basins (wadis):

| | WADI 1 | WADI 2 | Total |
|-|--------|--------|-------|
| Max water level | 9.16 TAW | 9.16 TAW | — |
| Buffer volume | 35.9 m³ | 8.2 m³ | 44.1 m³ |
| Infiltration surface | 132 m² | 26 m² | 158 m² |
| Basin depth | 30 cm | 30 cm | — |
| Slope | 1/3 | 1/3 | — |

Also includes a **zwemvijver** (natural swimming pond).

### Private garden areas (terraces per unit)
Small private outdoor spaces adjacent to each unit. Note: the main communal garden is shared.

| House | Area (m²) |
|-------|-----------|
| 1 | 16.35 |
| 2 | 18.35 |
| 3 | 11.00 |
| 4 | 11.05 |
| 5 | 15.00 |
| 6 | 17.30 |
| 7 | 18.80 |
| 8 | 15.20 |

### Parking
10 spaces (P1–P10) for residents.

### Vegetation legend (from preliminary plan)
- Bestaande boom — existing tree
- Nieuwe boom — new tree
- Gazon — lawn
- Gemengde haag — mixed hedge
- Struweelrand met bloei & bessen — shrub border with flowers & berries
- Lage vegetatie — low vegetation
- Oevervegetatie — water edge / riparian vegetation

---

## Oprijlaan (Driveway)

- Located in the **southwest corner** of the site.
- Originally flanked by **two rows of hornbeam trees** (haagbeuk), which were removed by the developer.
- The developer is required by the city to replace them with **beech or oak** trees.
- **This section is outside the community's control** — planting species and location are determined by the developer/city.

---

## Site Boundary & Surroundings

The cohousing project occupies the interior of a **city block**. The PDF drawings include the surrounding plots of neighboring properties. These are separated from the cohousing garden by **brick walls or tall fences** — they are neighboring backyards, not part of the design scope.

For the 3D solar model, the surrounding walls/fences are shadow-casting elements that should be included in the geometry, even though the vegetation behind them is outside scope.

---

## Design Scope

| Zone | Status |
|------|--------|
| **Northwest** | **Primary design focus** — food forest |
| **Northeast** | **Primary design focus** — food forest |
| South | Existing trees and plants present; not being redesigned |
| Southwest (oprijlaan) | Developer-controlled tree row; outside scope |

---

## Constraints & Fixed Elements

1. **Villa Gustaaf** — footprint and height are fixed.
2. **New U-shaped housing row** — footprint is fixed (under construction).
3. **Oprijlaan tree row** — developer-controlled; beech or oak, not our choice.
4. **Building heights** — relevant for shadow analysis; use ridge at +14.0 TAW.
5. **Perimeter walls/fences** — brick walls or tall fences along the property boundary; include as shadow casters in the model.

---

## 3D Model & Solar Simulation Plan

### Objective
Build a Python-based pipeline to:
1. Georeference the AutoCAD plan and extract building footprints.
2. Extrude buildings to 3D using known TAW heights.
3. Calculate solar position for Gent across the year using `pvlib`.
4. Cast shadow rays onto a garden-surface grid to produce seasonal irradiance maps.
5. Add parametric food forest tree models to simulate canopy shading at maturity.

### Key technical facts for the model
- **Coordinate reference**: TAW (Belgian altitude datum). Ground ≈ +9.0 TAW.
- **Drawing scale**: 1:200 — PDF coordinates in points need to be converted to meters.
- **North orientation**: North arrow visible on drawing; must be applied during coordinate alignment.
- **Sun position library**: `pvlib` with `Location(latitude=51.05, longitude=3.72, tz='Europe/Brussels')`.
- **Shadow geometry**: `trimesh` for ray-mesh intersection; `shapely` for 2D footprint tracing.
- **Visualization**: `pyvista` (3D interactive) + `matplotlib` (2D heatmaps).

### PDF geometry
The AutoCAD PDF contains 271,451 vector paths (lines). Coordinate units are PDF points (1 pt = 1/72 inch). At 1:200 scale, the conversion factor from PDF points to real-world meters needs to be calibrated against a known dimension on the drawing (e.g., a labeled wall length or the scale bar).

### Phases
1. **Phase 1** — Georeference & calibrate: establish PDF-point → meter transform using scale bar or known dimension.
2. **Phase 2** — Trace footprints: click-trace building outlines as `shapely` polygons in a Jupyter notebook.
3. **Phase 3** — Extrude to 3D: buildings become `trimesh` meshes using TAW heights above grade.
4. **Phase 4** — Solar simulation: `pvlib` sun angles + ray casting per garden grid cell.
5. **Phase 5** — Food forest layer: add parametric trees (position, mature height, canopy radius) and re-run.

# Provenance and recovery boundary

This repository is a clean modernization and compatibility corpus. The legacy
source trees remain unchanged.

Primary evidence inspected:

- `E:\UTILITY\LIBFUNCT.PRG`
- `E:\Visual Foxpro Projects\FoxPro & VB\Prg\LIBFUNCT.PRG`
- `E:\MDOT\MATCHPRG\CVT2GIS\FINDXY.PRG`
- `E:\MDOT\MATCHPRG\CVT2GIS\FIGUREMI.PRG`
- `E:\MDOT\MATCHPRG\TRACENET\*.PRG`

The two `LIBFUNCT.PRG` copies were byte-identical when this corpus was created.
Their SHA-256 digest was
`D506FB856D4B8E9AF01FE1C20ADF2BD6959B861C555F184702410488FCD785C1`.
Their original 1990s functions supplied names and behavioral intent, while the
implementations in this repository were rewritten from mathematical and graph
algorithm definitions.

No DBF data, personal records, organization-specific identifiers, absolute
legacy data paths, or compiled binaries are copied into this repository.

## Corrected defects

- Compass degrees are converted to radians before trigonometric calls.
- Longitude is calculated with a full spherical destination-point formula,
  rather than multiplying a degree delta by `COS(latitude)`.
- Zero latitude and zero longitude are accepted as real coordinates.
- Great-circle distance uses a clamped haversine calculation.
- Initial bearing uses spherical rather than flat longitude/latitude deltas.
- Link routing uses Dijkstra's global relaxation invariant.
- Small travelling-salesperson instances use exact Held-Karp dynamic
  programming. Larger heuristic tours are never mislabeled as optimal.
- Local road-shape radius uses a three-point circumcircle calculation.

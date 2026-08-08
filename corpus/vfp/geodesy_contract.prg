*! Executable VFP/Copperfin contract checks for the corrected library subset.

SET PROCEDURE TO libfunct_updated.prg ADDITIVE
LOCAL lnRadius, lnDistance, lnLatitude, lnLongitude
lnRadius = 6371008.8
lnDistance = arclength(0, 0, 1, 0, lnRadius)
IF ABS(lnDistance - 111195.0802335) > 0.001
    ERROR "Great-circle distance contract failed"
ENDIF
lnLatitude = nextlat(1000, lnRadius, 0, 90)
lnLongitude = nextlong(1000, lnRadius, 0, 0, 90)
IF ABS(lnLatitude) > 0.000001 OR ABS(lnLongitude - 0.0089932036) > 0.000001
    ERROR "Destination-point contract failed"
ENDIF
IF extrefdeg("SW") <> 225 OR extrefdir(225) <> "SW"
    ERROR "Compass conversion contract failed"
ENDIF
? "GEODESY_CONTRACT_OK"


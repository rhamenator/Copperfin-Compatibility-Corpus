*! Generic corrected compatibility subset recovered from LIBFUNCT.PRG.
*! Bearings are compass degrees clockwise from north. Coordinates are degrees.
*! Distances and radius must use the same unit.

FUNCTION normalizedegrees
LPARAMETERS tnDegrees
LOCAL lnResult
lnResult = MOD(tnDegrees, 360)
IF lnResult < 0
    lnResult = lnResult + 360
ENDIF
RETURN lnResult

FUNCTION normalizelongitude
LPARAMETERS tnLongitude
RETURN normalizedegrees(tnLongitude + 180) - 180

FUNCTION validcoordinate
LPARAMETERS tnLatitude, tnLongitude
RETURN VARTYPE(tnLatitude) = "N" AND VARTYPE(tnLongitude) = "N" AND ;
    tnLatitude >= -90 AND tnLatitude <= 90 AND ;
    tnLongitude >= -180 AND tnLongitude <= 180

FUNCTION nextlat
LPARAMETERS tnDistance, tnRadius, tnLatitude, tnDirection
LOCAL lnLat1, lnBearing, lnAngular, lnLat2
IF tnRadius <= 0 OR tnDistance < 0 OR tnLatitude < -90 OR tnLatitude > 90
    ERROR 11
ENDIF
lnLat1 = DTOR(tnLatitude)
lnBearing = DTOR(normalizedegrees(tnDirection))
lnAngular = tnDistance / tnRadius
lnLat2 = ASIN(MAX(-1, MIN(1, SIN(lnLat1) * COS(lnAngular) + ;
    COS(lnLat1) * SIN(lnAngular) * COS(lnBearing))))
RETURN RTOD(lnLat2)

FUNCTION nextlong
LPARAMETERS tnDistance, tnRadius, tnLatitude, tnLongitude, tnDirection
LOCAL lnLat1, lnLon1, lnLat2, lnBearing, lnAngular, lnLon2
IF tnRadius <= 0 OR tnDistance < 0 OR NOT validcoordinate(tnLatitude, tnLongitude)
    ERROR 11
ENDIF
lnLat1 = DTOR(tnLatitude)
lnLon1 = DTOR(tnLongitude)
lnBearing = DTOR(normalizedegrees(tnDirection))
lnAngular = tnDistance / tnRadius
lnLat2 = ASIN(MAX(-1, MIN(1, SIN(lnLat1) * COS(lnAngular) + ;
    COS(lnLat1) * SIN(lnAngular) * COS(lnBearing))))
lnLon2 = lnLon1 + ATN2(SIN(lnBearing) * SIN(lnAngular) * COS(lnLat1), ;
    COS(lnAngular) - SIN(lnLat1) * SIN(lnLat2))
RETURN normalizelongitude(RTOD(lnLon2))

FUNCTION arclength
LPARAMETERS tnLongitude1, tnLatitude1, tnLongitude2, tnLatitude2, tnRadius
LOCAL lnLat1, lnLat2, lnDeltaLat, lnDeltaLon, lnHaversine, lnAngle
IF tnRadius <= 0 OR NOT validcoordinate(tnLatitude1, tnLongitude1) OR ;
    NOT validcoordinate(tnLatitude2, tnLongitude2)
    ERROR 11
ENDIF
lnLat1 = DTOR(tnLatitude1)
lnLat2 = DTOR(tnLatitude2)
lnDeltaLat = lnLat2 - lnLat1
lnDeltaLon = DTOR(tnLongitude2 - tnLongitude1)
lnHaversine = SIN(lnDeltaLat / 2) ^ 2 + ;
    COS(lnLat1) * COS(lnLat2) * SIN(lnDeltaLon / 2) ^ 2
lnHaversine = MAX(0, MIN(1, lnHaversine))
lnAngle = 2 * ATN2(SQRT(lnHaversine), SQRT(MAX(0, 1 - lnHaversine)))
RETURN tnRadius * lnAngle

FUNCTION direction
LPARAMETERS tnLongitude1, tnLatitude1, tnLongitude2, tnLatitude2
LOCAL lnLat1, lnLat2, lnDeltaLon, lnX, lnY
IF NOT validcoordinate(tnLatitude1, tnLongitude1) OR ;
    NOT validcoordinate(tnLatitude2, tnLongitude2) OR ;
    (tnLatitude1 = tnLatitude2 AND tnLongitude1 = tnLongitude2)
    ERROR 11
ENDIF
lnLat1 = DTOR(tnLatitude1)
lnLat2 = DTOR(tnLatitude2)
lnDeltaLon = DTOR(tnLongitude2 - tnLongitude1)
lnY = SIN(lnDeltaLon) * COS(lnLat2)
lnX = COS(lnLat1) * SIN(lnLat2) - ;
    SIN(lnLat1) * COS(lnLat2) * COS(lnDeltaLon)
RETURN normalizedegrees(RTOD(ATN2(lnY, lnX)))

FUNCTION extrefdeg
LPARAMETERS tcDirection
LOCAL lcDirection
lcDirection = UPPER(ALLTRIM(tcDirection))
DO CASE
CASE lcDirection = "N"
    RETURN 0
CASE lcDirection = "NE"
    RETURN 45
CASE lcDirection = "E"
    RETURN 90
CASE lcDirection = "SE"
    RETURN 135
CASE lcDirection = "S"
    RETURN 180
CASE lcDirection = "SW"
    RETURN 225
CASE lcDirection = "W"
    RETURN 270
CASE lcDirection = "NW"
    RETURN 315
OTHERWISE
    RETURN -1
ENDCASE

FUNCTION extrefdir
LPARAMETERS tnDirection
LOCAL lnDirection, lnIndex
lnDirection = normalizedegrees(tnDirection)
lnIndex = MOD(INT((lnDirection + 22.5) / 45), 8)
RETURN GETWORDNUM("N NE E SE S SW W NW", lnIndex + 1)

FUNCTION curvature
LPARAMETERS tnLongitude1, tnLatitude1, tnLongitude2, tnLatitude2, ;
    tnLongitude3, tnLatitude3
LOCAL lnFirst, lnSecond, lnChange
lnFirst = direction(tnLongitude1, tnLatitude1, tnLongitude2, tnLatitude2)
lnSecond = direction(tnLongitude2, tnLatitude2, tnLongitude3, tnLatitude3)
lnChange = ABS(normalizedegrees(lnSecond - lnFirst))
IF lnChange > 180
    lnChange = 360 - lnChange
ENDIF
RETURN lnChange

FUNCTION turnradius
LPARAMETERS tnLongitude1, tnLatitude1, tnLongitude2, tnLatitude2, ;
    tnLongitude3, tnLatitude3, tnRadius
LOCAL lnMeanLat, lnX1, lnY1, lnX2, lnY2, lnX3, lnY3
LOCAL lnA, lnB, lnC, lnTwiceArea
lnMeanLat = DTOR((tnLatitude1 + tnLatitude2 + tnLatitude3) / 3)
lnX1 = DTOR(tnLongitude1) * COS(lnMeanLat) * tnRadius
lnY1 = DTOR(tnLatitude1) * tnRadius
lnX2 = DTOR(tnLongitude2) * COS(lnMeanLat) * tnRadius
lnY2 = DTOR(tnLatitude2) * tnRadius
lnX3 = DTOR(tnLongitude3) * COS(lnMeanLat) * tnRadius
lnY3 = DTOR(tnLatitude3) * tnRadius
lnA = SQRT((lnX3 - lnX2) ^ 2 + (lnY3 - lnY2) ^ 2)
lnB = SQRT((lnX3 - lnX1) ^ 2 + (lnY3 - lnY1) ^ 2)
lnC = SQRT((lnX2 - lnX1) ^ 2 + (lnY2 - lnY1) ^ 2)
lnTwiceArea = ABS((lnX2 - lnX1) * (lnY3 - lnY1) - ;
    (lnY2 - lnY1) * (lnX3 - lnX1))
IF lnA = 0 OR lnB = 0 OR lnC = 0 OR lnTwiceArea = 0
    RETURN 0
ENDIF
RETURN (lnA * lnB * lnC) / (2 * lnTwiceArea)

FUNCTION roadlength
LPARAMETERS tnLongitude1, tnLatitude1, tnLongitude2, tnLatitude2, ;
    tnLongitude3, tnLatitude3, tnRadius
RETURN arclength(tnLongitude1, tnLatitude1, tnLongitude2, tnLatitude2, tnRadius) + ;
    arclength(tnLongitude2, tnLatitude2, tnLongitude3, tnLatitude3, tnRadius)

FUNCTION geoheight
LPARAMETERS tnChordLength, tnArcLength, tnTurnRadius
LOCAL lnHalfAngle
IF tnChordLength < 0 OR tnArcLength < 0 OR tnTurnRadius <= 0
    RETURN 0
ENDIF
lnHalfAngle = (tnArcLength / tnTurnRadius) / 2
RETURN tnTurnRadius * (1 - COS(lnHalfAngle))


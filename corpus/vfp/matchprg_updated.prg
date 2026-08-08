*! Generic graph-routing compatibility fixture recovered from MATCHPRG ideas.
*! Edges are rows: source node, destination node, non-negative cost.
*! This uses Dijkstra's algorithm and selects the global minimum unsettled node;
*! it does not blindly accept the next physical record or locally shortest arc.

FUNCTION shortestpath
LPARAMETERS taEdges, tnEdgeCount, tnNodeCount, tnStart, tnGoal, ;
    taPath, tnPathCount
EXTERNAL ARRAY taEdges, taPath
LOCAL ARRAY laDistance[1], laPrevious[1], laVisited[1], laReverse[1]
LOCAL lnNode, lnEdge, lnCurrent, lnCandidate, lnMinimum, lnCursor
LOCAL lnReverseCount

IF tnNodeCount < 1 OR tnStart < 1 OR tnStart > tnNodeCount OR ;
    tnGoal < 1 OR tnGoal > tnNodeCount
    RETURN -1
ENDIF

DIMENSION laDistance[tnNodeCount], laPrevious[tnNodeCount], laVisited[tnNodeCount]
FOR lnNode = 1 TO tnNodeCount
    laDistance[lnNode] = 1E+308
    laPrevious[lnNode] = 0
    laVisited[lnNode] = .F.
ENDFOR
laDistance[tnStart] = 0

FOR lnNode = 1 TO tnNodeCount
    lnCurrent = 0
    lnMinimum = 1E+308
    * Select the global minimum unsettled node.
    FOR lnCursor = 1 TO tnNodeCount
        IF NOT laVisited[lnCursor] AND laDistance[lnCursor] < lnMinimum
            lnMinimum = laDistance[lnCursor]
            lnCurrent = lnCursor
        ENDIF
    ENDFOR
    IF lnCurrent = 0
        EXIT
    ENDIF
    IF lnCurrent = tnGoal
        EXIT
    ENDIF
    laVisited[lnCurrent] = .T.
    FOR lnEdge = 1 TO tnEdgeCount
        IF taEdges[lnEdge, 1] = lnCurrent
            IF taEdges[lnEdge, 3] < 0
                RETURN -1
            ENDIF
            lnCandidate = laDistance[lnCurrent] + taEdges[lnEdge, 3]
            IF lnCandidate < laDistance[taEdges[lnEdge, 2]]
                laDistance[taEdges[lnEdge, 2]] = lnCandidate
                laPrevious[taEdges[lnEdge, 2]] = lnCurrent
            ENDIF
        ENDIF
    ENDFOR
ENDFOR

IF laDistance[tnGoal] >= 1E+307
    tnPathCount = 0
    RETURN -1
ENDIF

DIMENSION laReverse[tnNodeCount]
lnReverseCount = 0
lnCursor = tnGoal
DO WHILE lnCursor <> 0
    lnReverseCount = lnReverseCount + 1
    laReverse[lnReverseCount] = lnCursor
    IF lnCursor = tnStart
        EXIT
    ENDIF
    lnCursor = laPrevious[lnCursor]
ENDDO
IF lnCursor <> tnStart
    tnPathCount = 0
    RETURN -1
ENDIF

tnPathCount = lnReverseCount
DIMENSION taPath[MAX(1, tnPathCount)]
FOR lnNode = 1 TO tnPathCount
    taPath[lnNode] = laReverse[tnPathCount - lnNode + 1]
ENDFOR
RETURN laDistance[tnGoal]


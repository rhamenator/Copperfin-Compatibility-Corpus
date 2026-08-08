*! Executable VFP/Copperfin contract check for global shortest-path routing.

SET PROCEDURE TO matchprg_updated.prg ADDITIVE
LOCAL ARRAY laEdges[10, 3], laPath[1]
LOCAL lnPathCount, lnCost
laEdges[1,1] = 1
laEdges[1,2] = 2
laEdges[1,3] = 4
laEdges[2,1] = 2
laEdges[2,2] = 1
laEdges[2,3] = 4
laEdges[3,1] = 1
laEdges[3,2] = 3
laEdges[3,3] = 2
laEdges[4,1] = 3
laEdges[4,2] = 1
laEdges[4,3] = 2
laEdges[5,1] = 3
laEdges[5,2] = 2
laEdges[5,3] = 1
laEdges[6,1] = 2
laEdges[6,2] = 3
laEdges[6,3] = 1
laEdges[7,1] = 2
laEdges[7,2] = 4
laEdges[7,3] = 5
laEdges[8,1] = 4
laEdges[8,2] = 2
laEdges[8,3] = 5
laEdges[9,1] = 3
laEdges[9,2] = 4
laEdges[9,3] = 8
laEdges[10,1] = 4
laEdges[10,2] = 3
laEdges[10,3] = 8
lnCost = shortestpath(@laEdges, 10, 4, 1, 4, @laPath, @lnPathCount)
IF lnCost <> 8 OR lnPathCount <> 4 OR laPath[1] <> 1 OR ;
    laPath[2] <> 3 OR laPath[3] <> 2 OR laPath[4] <> 4
    ERROR "Shortest-path contract failed"
ENDIF
? "GRAPH_CONTRACT_OK"


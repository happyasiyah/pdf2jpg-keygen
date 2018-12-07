:exclamation: **For learning purpose only**    

 
# pdf2jpg-keygen    

## Key Generation    

### Software Editions     
"SG", Single License 1 PC    
"PS", Personal License 3 PCs    
"HM", Home License 10 PCs     
"TM", Team License (> 10 PCs)     
"EP", Enterprise License (>100 PCs)     

### Character mappings    
Z = 0    
S = 1    
Q = 2    
J = 3    
W = 4    
L = 5    
D = 6    
T = 7    
M = 8    
X = 9    

### Key Construction
key len >19    
[0,4] = NGIP    
[4] = "-"    
[5-2] {"SG", "PS", "HM", "TM", "EP"}    
0>= [7-2] < 24    
[7] = {"Z", "S", "Q"}    
if [7] = "Z" or [7] = "S" then [8] = {"Z", "S", "Q", "J", "W", "L", "D", "T", "M", "X"}    
if [7] = "Q" then [8] = {"Z", "S", "Q", "J"}    
[9] = "-"    

1>= [10-2] <32    
[10] = {"Z", "S", "Q", "J"}    
if [10] = "Z" then [11] = {"S", "Q", "J", "W", "L", "D", "T", "M", "X"}     
if [10] = "S" then [11] = {"Z", "S", "Q", "J", "W", "L", "D", "T", "M", "X"}    
if [10] = "Q" then [11] = {"S", "Q", "J", "W", "L", "D", "T", "M", "X"}    
if [10] = "J" then [11] = {"Z", S", "Q"}    

1>= [12-2] <13    
[12] = {"Z", "S"}    
if [12] = "Z" then [13] = {"S", "Q", "J", "W", "L", "D", "T", "M", "X"}    
if [12] = "S" then [13] = {"S", "Q", "J"}    

[14] = "-"    

0>=[15-2] <60    
[15] = {"Z", "S", "Q", "J", "W", "L"}    
if [15] = "Z", "S", "Q", "J", "W" or "L" then [16] = {"Z", "S", "Q", "J", "W", "L", "D", "T", "M", "X"}     

0>=[17-2]<60    
if [17] = "Z", "S", "Q", "J", "W" or "L" then [18] = {"Z", "S", "Q", "J", "W", "L", "D", "T", "M", "X"}     

if "TM" or "EP"     
[19] "-"    
11>=[20-6]<1000000    
Example: NGIP-TMQJ-ZSSQ-LXQD-ZZZLZZ    




//  ------------------------------------------

//             TOKENIZER MODULE
 
//  --------       ---------       
//  |      |       |       |       
//  | GH   |  -->  | REPO  |  -->  [ VECTOR REPRESENTATION PLUGGED INTO THE VDB ]
//  | REPO |       | OBJ.  |   ^   
//  |      |       |       |   |   
//  --------       ---------   |   
//                             |
//                         TOKENIZER
//  ------------------------------------------


//  -----------------------------------------

//             TOKENIZER WORKFLOW

//     ---------
//     |       |
//     | REPO  | --> " FLATTENED REPO OBJECT "  --> HASH FUNCTION (FOR DIM. REDUCTION) -->  PREDICT OUTPUT VECTOR
//     | OBJ.  |
//     |       |
//     ---------
//  -----------------------------------------


// ABOUT THE VOCAB USED TO TRAIN THE VECTOR PREDICTIONS

//     THE VOCAB SHOULD CONTAIN TEXT ABOUT GITHUB ITSELF OR COMPUTER SCIENCE IN GENERAL

//     IF LSH IS CHOSEN TO BE THE INDEXING SYSTEM, THEN NOTHING ELSE WILL BE NEEDED

//     IF NOT LSH, THE ONE PICKED MUST TOKENIZE REPOS BASED ON A NET THAT COULD REPRESENT THE ESSENCE OF A REPO (LIKE A CNN OR GloVe APPROACHES)


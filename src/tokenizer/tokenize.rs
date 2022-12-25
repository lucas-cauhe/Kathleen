
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

//    WITH IVF REPOS WILL BE TOKENIZED  WITH A NET THAT COULD REPRESENT THE ESSENCE OF A REPO (LIKE A CNN OR GloVe APPROACHES)



pub struct Embedding {
    
}
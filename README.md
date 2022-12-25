# Kathleen
Semantic Search Engine for github repos

# What's with Kathleen

Kathleen's purpose is to match the most similar results out of two isolated queries, i.e. it would fetch github repos' info from n-given repos so that a user receives those that are most similar (in repos' specified aspects) to the n the user queried.
When a user enters its query, most likely will focus on subjects the user is working on, thus by crawling through its github user account Kathleen can determine which cluster (if already built) the query is going to be matched to.

# Index Representation

The vector database will be indexed using either LSH or IVF, in the end Kathleen needs some k-dimensional vector space where to hold vector representations of each gh repo crawled. In fact, its design might be a little awkward. Kathleen will keep track of clusters of neighbor vectors formed during insertion into the DB, thus when a user performs a query which targets out of the 'user cluster' boundaries, a path will be drawn between the two clusters which will indicate in further searches the similarity between both clusters.
As a result, Kathleen starts to look like an undirected graph which joins different clusters (its nodes) with weighted paths (weigths) defined by the similarity of the nodes they are pulling together. Finally if we where to search for similar gh repos to n given, the result would be the node or 'station' whose 'similarity distance' is the highest to any other given node.

Now, once described how GH repos are going to be indexed, it seems natural that the indexing method chosen is IVF.
# Kathleen
Semantic Search Engine for github repos

# What's with Kathleen

Kathleen's purpose is to match the most similar results out of two isolated queries, i.e. it would fetch github repos' info from n-given repos so that a user receives those that are most similar (in repos' specified aspects) to the n the user queried.
When a user enters its query, most likely will focus on subjects the user is working on, thus by crawling through its github user account Kathleen can determine which cluster (if already built) the query is going to be matched to.

# Index Representation

The vector database will be indexed using either LSH or IVF, in the end Kathleen needs some k-dimensional vector space where to hold vector representations of each gh repo crawled. In fact, its design might be a little awkward. Kathleen will keep track of clusters of neighbor vectors formed during insertion into the DB, thus when a user performs a query which targets out of the 'user cluster' boundaries, a path will be drawn between the two clusters which will indicate in further searches the similarity between both clusters.
As a result, Kathleen starts to look like an undirected graph which joins different clusters (its nodes) with weighted paths (weigths) defined by the similarity of the nodes they are pulling together. Finally if we where to search for similar gh repos to n given, the result would be the node or 'station' whose 'similarity distance' is the highest to any other given node.

Now, once described how GH repos are going to be indexed, it seems natural that the indexing method chosen is IVF.


# Storage 

Following the schema presented in [this post](https://towardsdatascience.com/similarity-search-with-ivfpq-9c6348fd4db3), Kathleen's storage will be divided into two main parts: an "environment" for each cluster/partition made and an inverted list which will point to each "environment" where to find a set of vectors.

An environment will have every raw vector that belongs to that cluster as well as every object representation for each vector in the environment. This way each environment will be responsible for its own vectors (not talking about how this could scale).


# Why LMDB Got replaced by RocksDB

loaded_segment will only be valid until transaction is dropped, thus it will give an error
owning the value will force to store duplicates and keeping the transaction open won't allow to open a new one 

we're left with two options -> 1. Clone values 2. use a callback so that logic needed to be done with loaded embeddings is
done while the transaction is alive (3. change db jeje, yes)

Besides IVF is a memory intensive algorithm

db is always in-memory so it won't be dropped, besides when calling get(), you obtain a reference to the value
which is in memory so the table won't be dropped at least until the value is dereferenced and environment dropped

now, values are inside an abstract struct which keeps all references valid so that when multiple operations are made
requiring same segments or embeddings, different processes don't access directly the values in the database and have 
possible issues with multiple mutable references or so.


## RocksDB usage

what before where environments to each cluster information now are going to be column families containing all the information
from the cluster but caching is now going to be easier and more rational
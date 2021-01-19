# bkmers

This repo implements a sequence to binary kmer transformation, but writes the kmers in numeric ascii.

## usage

A FASTA or FASTQ input is required.
A kmer length given by `-k` can be used to specify the kmer width.
Currently, only 4, 8, 16, and 32-bp kmers are supported.

```
bkmers -k 16 seqs.fa >kmers.txt
```
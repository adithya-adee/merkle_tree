# Merkel Tree

It is a binary tree built from bottom up approach. Where all the non-leaf nodes are hash of their left and right node.

- Leaf Nodes -> Original Data
- Parent Nodes -> Combination of left and right nodes hash

## Merkle Tree Construction

- All the data fields become the leaf node & construction is based on the below formulae
- Where A is left node and B is right Node then parent Node = Hash AB

_Hash AB = Hash(Hash A || Hash B)_

## Merkle Tree Verfication

The main verification point is Merkle Root (which is the root node of the Merkle Tree) . This provides verfication based on the **Tamper-Proof Summary**.

Since any changes in original data , will cause a havoc of chained changes till the top causing the Merkle Root has to change.
So based on the Tamper-Proof Summary , we can say that if we correct trusted merkle root then the underlying data is correct

### How Does Merkle Proof Work??

A Merkle Proof allows someone to prove that a specific data block exists within the tree and was used to calculate the trusted Merkle Root, without having to see or download all the other data (Zero Knowledge Proof). This is actually cool right!!

So , now to prove the data that the user has, he has to send the data which he wants to verify.

Lets take an example of Data A, B, C ,D
User sends only the data C , to verify.

Now the server will find the corressponding sibling to Data C hash (Hash C) , which is Data D. Then it will find the Hash of AB (upper level of C) and then combine hash to check weather it is equal to Merkle Root Hash.

If we take example of Data A - H
User sends only the data E, to verify.

Now the server will find its sibling Hash F, then Hash GH, then Hash ABCD

## Code Working

Construction & Verification Algorithm :
Refer the [design document] (design.excaildraw)
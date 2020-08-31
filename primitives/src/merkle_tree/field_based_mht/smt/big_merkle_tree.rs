/* Sparse Big Merkle tree */
/* Assumption in the code: arity of the Merkle tree = 2 */

use crate::merkle_tree::field_based_mht::smt::{Coord, OperationLeaf, BigMerkleTreeState};
use crate::{FieldBasedHash, FieldBasedMerkleTreeParameters};
use crate::merkle_tree::field_based_mht::smt::ActionLeaf::Insert;

use algebra::{PrimeField, MulShort, FromBytes};
use algebra::{ToBytes, to_bytes};

use rocksdb::{DB, Options};

use std::marker::PhantomData;
use std::fs;
use crate::merkle_tree::field_based_mht::smt::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct BigMerkleTree<
    F: PrimeField + MulShort,
    T: FieldBasedMerkleTreeParameters<Data = F>,
    H: FieldBasedHash<Data = F>,
>{
    // if unset, all DBs and tree internal state will be deleted when an instance of this struct
    // gets dropped
    persistent: bool,
    // path to state required to restore the tree
    state_path: Option<String>,
    // tree in-memory state
    state: BigMerkleTreeState<F, T>,
    // the height of the Merkle tree
    height: usize,
    // path to the db
    path_db: String,
    // stores the leaves
    database: DB,
    // path to the cache
    path_cache: String,
    // stores the cached nodes
    db_cache: DB,

    _field: PhantomData<F>,
    _parameters: PhantomData<T>,
    _hash_parameters: PhantomData<H>,
}

impl<
    F: PrimeField + MulShort,
    H: FieldBasedHash<Data = F>,
    T: FieldBasedMerkleTreeParameters<Data = F>
> Drop for BigMerkleTree<F, T, H> {
    fn drop(&mut self) {
        self.close();
    }
}

impl<
    F: PrimeField + MulShort,
    H: FieldBasedHash<Data = F>,
    T: FieldBasedMerkleTreeParameters<Data = F>
> BigMerkleTree<F, T, H> {
    // Creates a new tree of specified `width`.
    // If `persistent` is specified, then DBs will be kept on disk and the tree state will be saved
    // so that the tree can be restored any moment later. Otherwise, no state will be saved on file
    // and the DBs will be deleted.
    pub fn new_unitialized(width: usize, persistent: bool, state_path: Option<String>, path_db: String, path_cache: String) -> Result<Self, Error> {

        // If the tree must be persistent, that a path to which save the tree state must be
        // specified.
        if !persistent { assert!(state_path.is_none()) } else { assert!(state_path.is_some()) }

        let height = width as f64;
        let height = height.log(T::MERKLE_ARITY as f64) as usize;
        let state = BigMerkleTreeState::<F, T>::get_default_state(width, height);
        let path_db = path_db;
        let database = DB::open_default(path_db.clone())
            .map_err(|e| Error::Other(e.to_string()))?;
        let path_cache = path_cache;
        let db_cache = DB::open_default(path_cache.clone())
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(BigMerkleTree {
            persistent,
            state_path,
            state,
            height,
            path_db,
            database,
            path_cache,
            db_cache,
            _field: PhantomData,
            _parameters: PhantomData,
            _hash_parameters: PhantomData,
        })
    }

    // Creates a new tree starting from state at `state_path` and DBs at `path_db` and `path_cache`.
    // The new tree may be persistent or not, the actions taken in both cases are the same as
    // in `new_unitialized()`.
    pub fn new(persistent: bool, state_path: String, path_db: String, path_cache: String) -> Result<Self, Error> {

        let state = {
            let state_file = fs::File::open(state_path.clone())
                .map_err(|e| Error::Other(e.to_string()))?;
            BigMerkleTreeState::<F, T>::read(state_file)
        }.map_err(|e| Error::Other(e.to_string()))?;

        let height = state.width as f64;
        let height = height.log(T::MERKLE_ARITY as f64) as usize;

        let opening_options = Options::default();

        let path_db = path_db;
        let database = DB::open(&opening_options, path_db.clone())
            .map_err(|e| Error::Other(e.to_string()))?;

        let path_cache = path_cache;
        let db_cache = DB::open(&opening_options,path_cache.clone())
            .map_err(|e| Error::Other(e.to_string()))?;

        Ok(BigMerkleTree {
            persistent,
            state_path: Some(state_path),
            state,
            height,
            path_db,
            database,
            path_cache,
            db_cache,
            _field: PhantomData,
            _parameters: PhantomData,
            _hash_parameters: PhantomData,
        })
    }

    pub fn close(&mut self) {
        if !self.persistent {

            if self.state_path.is_some() && Path::new(&self.state_path.clone().unwrap()).exists() {
                match fs::remove_file(self.state_path.clone().unwrap()) {
                    Ok(_) => (),
                    Err(e) => println!("Error deleting tree state: {}", e)
                }
            }

            // Deletes the folder containing the db
            match fs::remove_dir_all(self.path_db.clone()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error deleting the folder containing the db: {}", e);
                }
            };
            // Deletes the folder containing the cache
            match fs::remove_dir_all(self.path_cache.clone()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error deleting the folder containing the db: {}", e);
                }
            };
        } else {
            // Don't delete the DBs and save required state on file in order to restore the
            // tree later.
            let tree_state_file = fs::File::create(self.state_path.clone().unwrap())
                .expect("Should be able to create file for tree state");

            self.state.write(tree_state_file)
                .expect("Should be able to write into tree state file");
        }
    }

    pub fn height(&self) -> usize { self.height }

    pub fn set_persistency(&mut self, persistency: bool) {
        self.persistent = persistency;
    }

    /* ===============================================================================*/
    /* Cache operations */
    /*===============================================================================*/

    pub fn insert_to_cache(&self, coord: Coord, data:F) {
        // Inserts the node into the cache

        let elem = to_bytes!(data).unwrap();
        let index = bincode::serialize(&coord).unwrap();
        self.database.put(index, elem).unwrap();
    }

    pub fn contains_key_in_cache(&self, coord:Coord) -> bool {
        // Checks if the node is in the cache

        let coordinates = bincode::serialize(&coord).unwrap();
        match self.database.get(coordinates) {
            Ok(Some(_value)) => {
                return true;
            },
            Ok(None) => {
                return false;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return false;
            },
        }
    }

    pub fn get_from_cache(&self, coord:Coord) -> Option<F> {
        // Retrieves the node from the cache
        // If the node is in the cache, it returns the hash as an option
        // If the node is not in the cache, or there is an error, it returns none

        let coordinates = bincode::serialize(&coord).unwrap();
        match self.database.get(coordinates) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                return Some(retrieved_elem);
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }

    pub fn remove_from_cache(&self, coord: Coord) -> Option<F>{
        // Deletes the node from the cache
        // If the node was in the cache, it deletes the node and returns the hash as an option
        // If the node was not present in the cache, or if there was an error, it returns none

        let coordinates = bincode::serialize(&coord).unwrap();
        match self.database.get(coordinates.clone()) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                let res = self.database.delete(coordinates.clone());
                match res {
                    Ok(_) => {
                        return Some(retrieved_elem);
                    },
                    Err(e) => {
                        println!("Could not delete node from cache: {}", e);
                        return None;
                    }
                }
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }

    /* ===============================================================================*/
    /* Leaves operations */
    /*===============================================================================*/

    pub fn insert_to_db(&self, idx: usize, data: F) {
        // Inserts the leaf to the db

        let elem = to_bytes!(data).unwrap();
        let index = bincode::serialize(&idx).unwrap();
        self.database.put(index, elem).unwrap();
    }

    pub fn get_from_db(&self, idx: usize) -> Option<F>{
        // Retrieves the leaf from the db

        let index = bincode::serialize(&idx).unwrap();
        match self.database.get(index) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                return Some(retrieved_elem);
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }

    pub fn remove_from_db(&self, idx: usize) -> Option<F>{
        // Deletes the leaf from the db
        // If the leaf was in the db, it return the hash as an option
        // If the leaf was not present, or if there is an error, it returns none

        let index = bincode::serialize(&idx).unwrap();
        match self.database.get(index.clone()) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                let res = self.database.delete(index.clone());
                match res {
                    Ok(_) => {
                        return Some(retrieved_elem);
                    },
                    Err(e) => {
                        println!("Could not delete leaf from db: {}", e);
                        return None;
                    }
                }
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }

    /* ===============================================================================*/
    /* Merkle tree operations */
    /*===============================================================================*/

    pub fn insert_leaf(&mut self, coord: Coord, leaf: F) {
        // Inserts a leaf in the Merkle tree.
        // Updates the Merkle tree on the path from the leaf to the root

        // check that the index of the leaf to be inserted is less than the width of the Merkle tree
        assert!(coord.idx < self.state.width, "Leaf index out of bound.");
        // check that the coordinates of the node corresponds to the leaf level
        assert_eq!(coord.height, 0, "Coord of the node does not correspond to leaf level");

        if self.state.present_node.contains(&coord) {
            let old_leaf = self.get_from_db(coord.idx);
            let old_hash;
            if let Some(i) = old_leaf {
                old_hash = i;
            } else {
                old_hash = T::EMPTY_HASH_CST[0];
            }
            if old_hash != leaf {
                self.insert_to_db(coord.idx, leaf);
                self.state.cache_path.clear();
                self.state.cache_path.insert(coord, leaf);
                BigMerkleTree::update_tree(self, coord);
            }
        } else {
            // mark as non empty leaf
            self.state.present_node.insert(coord);
            self.insert_to_db(coord.idx, leaf);
            self.state.cache_path.clear();
            self.state.cache_path.insert(coord, leaf);
            BigMerkleTree::update_tree(self, coord);
        }
    }

    // Removes a leaf in the Merkle tree.
    // Updates the Merkle tree on the path from the leaf to the root
    pub fn remove_leaf(&mut self, coord: Coord) {

        // check that the index of the leaf to be inserted is less than the width of the Merkle tree
        assert!(coord.idx < self.state.width, "Leaf index out of bound.");
        // check that the coordinates of the node corresponds to the leaf level
        assert_eq!(coord.height, 0, "Coord of the node does not correspond to leaf level");

        // take that leaf from the non-empty set
        self.state.present_node.remove(&coord);
        self.state.cache_path.clear();
        self.state.cache_path.insert(coord, T::EMPTY_HASH_CST[0]);
        // removes the leaf from the db
        let res = self.remove_from_db(coord.idx);
        // if it was in the db, update the tree
        if res != None {
            BigMerkleTree::update_tree(self, coord);
        }
    }

    pub fn is_leaf_empty(&self, coord: Coord) -> bool {
        // check that the index of the leaf is less than the width of the Merkle tree
        assert!(coord.idx < self.state.width, "Leaf index out of bound.");
        // check that the coordinates of the node corresponds to the leaf level
        assert_eq!(coord.height, 0, "Coord of the node does not correspond to leaf level");

        !self.state.present_node.contains(&coord)
    }

    // Updates the tree visiting the parent nodes from the leaf to the root
    // Calculates the hash and caches it
    pub fn update_tree(&mut self, coord: Coord) {

        // Process the node of level 1 with the inserted/removed leaf
        // check whether the hash corresponds to the left or right child
        let mut idx = coord.idx;
        let left_child_coord: Coord;
        let right_child_coord: Coord;
        let left_child_idx: usize;
        let right_child_idx: usize;
        let left_child: Option<F>;
        let right_child: Option<F>;
        let left_hash: F;
        let right_hash: F;
        let mut height = 0;

        assert_eq!(T::MERKLE_ARITY,2, "Arity of the Merkle tree is not 2.");

        if idx % T::MERKLE_ARITY == 0 {
            left_child_idx = idx;
            left_child_coord = Coord { height, idx: left_child_idx };
            // get the left child from the cache_path
            let hash = self.state.cache_path.get(&left_child_coord);
            if let Some(i) = hash {
                left_hash = *i;
            } else {
                left_hash = T::EMPTY_HASH_CST[0];
            }
            right_child_idx = idx + 1;
            right_child_coord = Coord { height, idx: right_child_idx };
            if self.state.present_node.contains(&right_child_coord) {
                right_child = self.get_from_db(right_child_idx);
                if let Some(i) = right_child {
                    right_hash = i;
                } else {
                    right_hash = T::EMPTY_HASH_CST[0];
                }
            } else {
                right_hash = T::EMPTY_HASH_CST[0];
            }
        } else {
            right_child_idx = idx;
            right_child_coord = Coord { height, idx: right_child_idx };
            // get the right child from the cache path
            let hash = self.state.cache_path.get(&right_child_coord);
            if let Some(i) = hash {
                right_hash = *i;
            } else {
                right_hash = T::EMPTY_HASH_CST[0];
            }
            left_child_idx = idx - 1;
            left_child_coord = Coord { height, idx: left_child_idx };
            if self.state.present_node.contains(&left_child_coord) {
                left_child = self.get_from_db(left_child_idx);
                if let Some(i) = left_child {
                    left_hash = i;
                } else {
                    left_hash = T::EMPTY_HASH_CST[0];
                }
            } else {
                left_hash = T::EMPTY_HASH_CST[0];
            }
        }

        // go up one level
        height += 1;
        idx = idx / T::MERKLE_ARITY;
        let parent_coord = Coord { height, idx };

        let mut node_hash: F;
        if (!self.state.present_node.contains(&left_child_coord)) & (!self.state.present_node.contains(&right_child_coord)) {
            // if both children are empty

            node_hash = T::EMPTY_HASH_CST[height];

            // insert the parent node into the cache_path
            self.state.cache_path.insert(parent_coord, node_hash);

            // Both children are empty leaves
            // remove the parent node from the presence set
            self.state.present_node.remove(&parent_coord.clone());
            // remove node from cache
            self.remove_from_cache(parent_coord.clone());
        } else {

            // compute the hash of the node with the hashes of the children
            node_hash = Self::field_hash(left_hash, right_hash);
            // insert the parent node into the cache_path
            self.state.cache_path.insert(parent_coord, node_hash);
            // set the parent as present
            self.state.present_node.insert(parent_coord.clone());
            // Both children are not empty leaves
            if (self.state.present_node.contains(&left_child_coord)) & (self.state.present_node.contains(&right_child_coord)) {
                // cache the node
                self.insert_to_cache(parent_coord.clone(), node_hash);
            }
        }

        // Process level >= 2
        while height != self.height {

            let left_child_height = height;
            let right_child_height = height;

            // go to parent node
            height += 1;
            let idx_child = idx;
            idx = idx / T::MERKLE_ARITY;
            let parent_coord = Coord { height, idx };

            // retrieve the left child
            let left_child_idx = parent_coord.idx * T::MERKLE_ARITY;
            let left_child_coord = Coord { height: left_child_height, idx: left_child_idx };
            let left_hash: F;
            if left_child_idx == idx_child {
                // It was cached in the previous iteration. Get it from the cache_path
                left_hash = *self.state.cache_path.get(&left_child_coord).unwrap();
            } else {
                left_hash = BigMerkleTree::node(self, left_child_coord);
            }

            // retrieve the right child
            let right_child_idx = left_child_idx + 1;
            let right_child_coord = Coord { height: right_child_height, idx: right_child_idx };
            let right_hash: F;
            if right_child_idx == idx_child {
                // It was cached in the previous iteration. Get it from the cache_path
                right_hash = *self.state.cache_path.get(&right_child_coord).unwrap();
            } else {
                right_hash = BigMerkleTree::node(self, right_child_coord);
            }

            if (!self.state.present_node.contains(&left_child_coord)) & (!self.state.present_node.contains(&right_child_coord)) {
                // both children are empty => parent as well

                node_hash = T::EMPTY_HASH_CST[height];
                // insert the parent node into the cache_path
                self.state.cache_path.insert(parent_coord, node_hash);
                // remove node from non_empty set
                self.state.present_node.remove(&parent_coord.clone());
            } else {
                // at least one is non-empty

                // compute the hash of the parent node based on the hashes of the children
                node_hash = Self::field_hash(left_hash, right_hash);
                // insert the parent node into the cache_path
                self.state.cache_path.insert(parent_coord, node_hash);

                if self.state.present_node.contains(&left_child_coord) & self.state.present_node.contains(&right_child_coord) {
                    // both children are present

                    // set the parent node as non_empty
                    self.state.present_node.insert(parent_coord.clone());
                    // children not empty leaves, then cache the parent node
                    self.insert_to_cache(parent_coord.clone(), node_hash);
                    // cache the children
                    self.insert_to_cache(left_child_coord.clone(), left_hash);
                    self.insert_to_cache(right_child_coord.clone(), right_hash);
                } else {
                    // one is empty and the other is non-empty child, include the parent node in a non-empty set
                    self.state.present_node.insert(parent_coord.clone());
                }
            }
            if (!self.state.present_node.contains(&left_child_coord)) | (!self.state.present_node.contains(&right_child_coord)) {
                // at least one child is empty

                // remove subtree from cache
                BigMerkleTree::remove_subtree_from_cache(self, parent_coord);

            }
        }
        self.state.root = node_hash;
    }

    pub fn get_root(&self) -> F {
        self.state.root.clone()
    }

    pub fn remove_subtree_from_cache(&mut self, coord: Coord) {

        assert_eq!(T::MERKLE_ARITY,2, "Arity of the Merkle tree is not 2.");

        if coord.height <= 1 {
            // This condition should never occur
            return;
        }

        // remove the parent node from the cache
        self.remove_from_cache(coord.clone());

        let left_child_idx = coord.idx * T::MERKLE_ARITY;
        let left_child_height = coord.height - 1;
        let left_coord = Coord { height: left_child_height, idx: left_child_idx };

        if self.state.present_node.contains(&left_coord) {
            // go to the left child
            let ll_child_idx = left_child_idx * T::MERKLE_ARITY;
            let ll_child_height = left_child_height - 1;
            let ll_coord = Coord { height: ll_child_height, idx: ll_child_idx };

            let lr_child_idx = ll_child_idx + 1;
            let lr_child_height = ll_child_height;
            let lr_coord = Coord { height: lr_child_height, idx: lr_child_idx };

            if (!self.state.present_node.contains(&ll_coord)) | (!self.state.present_node.contains(&lr_coord)) {
                self.remove_from_cache(left_coord);
            }
        }

        let right_child_idx = left_child_idx + 1;
        let right_child_height = left_child_height;
        let right_coord = Coord { height: right_child_height, idx: right_child_idx };

        if self.state.present_node.contains(&right_coord) {
            // go to the right child

            let rl_child_idx = right_child_idx * T::MERKLE_ARITY;
            let rl_child_height = right_child_height - 1;
            let rl_coord = Coord { height: rl_child_height, idx: rl_child_idx };

            let rr_child_idx = rl_child_idx + 1;
            let rr_child_height = rl_child_height;
            let rr_coord = Coord { height: rr_child_height, idx: rr_child_idx };

            if (!self.state.present_node.contains(&rl_coord)) | (!self.state.present_node.contains(&rr_coord)) {
                self.remove_from_cache(right_coord);
            }
        }
        return;
    }

    pub fn node(&mut self, coord: Coord) -> F {
        // Returns the hash value associated to the node.
        // If the node is in the cache, it retrieves from it.
        // If not, recomputes it.
        // Only used for nodes of level >= 1 (not for leaves).


        assert_eq!(T::MERKLE_ARITY,2, "Arity of the Merkle tree is not 2.");

        //let coord = Coord{height, idx};
        // if the node is an empty node return the hash constant
        if !self.state.present_node.contains(&coord) {
            return T::EMPTY_HASH_CST[coord.height];
        }
        let res = self.get_from_cache(coord);

        // if not in the cache, recompute it
        if res == None {
            /* the node is not in the cache, compute it */
            let node_hash;
            if coord.height == 1 {
                /* get leaves to compute */
                let left_child_idx = coord.idx * T::MERKLE_ARITY;
                let left_child = self.get_from_db(left_child_idx);
                let left_hash: F;
                if let Some(i) = left_child {
                    left_hash = i;
                } else {
                    left_hash = T::EMPTY_HASH_CST[0];
                }

                let right_child_idx = left_child_idx + 1;
                let right_child = self.get_from_db(right_child_idx);
                let right_hash: F;
                if let Some(i) = right_child {
                    right_hash = i;
                } else {
                    right_hash = T::EMPTY_HASH_CST[0];
                }
                node_hash = Self::field_hash(left_hash, right_hash);
            } else {
                let height_child = coord.height - 1;
                let left_child_idx = coord.idx * T::MERKLE_ARITY;
                let coord_left = Coord { height: height_child, idx: left_child_idx };
                let left_child_hash = BigMerkleTree::node(self, coord_left);

                let right_child_idx = left_child_idx + 1;
                let coord_right = Coord { height: height_child, idx: right_child_idx };
                let right_child_hash = BigMerkleTree::node(self, coord_right);

                node_hash = Self::field_hash(left_child_hash, right_child_hash);
            }
            return node_hash;
        }

        res.unwrap()
    }

    pub fn field_hash(x: F, y: F) -> F {
        H::init(None)
            .update(x)
            .update(y)
            .finalize()
    }

    pub fn process_leaves_normal(&mut self, lidx: Vec<OperationLeaf<F>>) -> F {

        for k in 0..lidx.len() {
            let x = lidx[k];
            let coord = x.coord;
            let action = x.action;
            let hash = x.hash;

            if action == Insert {
                self.insert_leaf(coord, hash.unwrap());
            } else {
                self.remove_leaf(coord);
            }
        }
        self.state.root
    }

}


#[cfg(test)]
mod test {
    use crate::merkle_tree::field_based_mht::smt::{MNT4PoseidonHash, OperationLeaf, Coord, ActionLeaf};
    use crate::merkle_tree::field_based_mht::{FieldBasedMerkleTreeConfig, NaiveMerkleTree};
    use crate::merkle_tree::field_based_mht::{MNT4753MHTPoseidonParameters, MNT6753MHTPoseidonParameters};
    use crate::crh::{
        MNT6PoseidonHash
    };

    use algebra::fields::mnt6753::Fr as MNT6753Fr;
    use algebra::fields::mnt4753::Fr as MNT4753Fr;
    use algebra::biginteger::BigInteger768;

    use std::str::FromStr;
    use std::path::Path;

    use rand_xorshift::XorShiftRng;
    use rand::SeedableRng;
    use crate::merkle_tree::field_based_mht::smt::big_merkle_tree::BigMerkleTree;

    pub type MNT4PoseidonSmt = BigMerkleTree<MNT4753Fr, MNT4753MHTPoseidonParameters, MNT4PoseidonHash>;

    struct MNT4753FieldBasedMerkleTreeParams;

    impl FieldBasedMerkleTreeConfig for MNT4753FieldBasedMerkleTreeParams {
        const HEIGHT: usize = 6;
        type H = MNT4PoseidonHash;
    }

    type MNT4753FieldBasedMerkleTree = NaiveMerkleTree<MNT4753FieldBasedMerkleTreeParams>;

    #[test]
    fn compare_merkle_trees_mnt4_1() {
        use algebra::{
            fields::mnt4753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves_to_process: Vec<OperationLeaf<MNT4753Fr>> = Vec::new();

        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 0 }, action: ActionLeaf::Insert, hash: Some(MNT4753Fr::from_str("1").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 9 }, action: ActionLeaf::Insert, hash: Some(MNT4753Fr::from_str("2").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT4753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 29 }, action: ActionLeaf::Insert, hash: Some(MNT4753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT4753Fr::from_str("3").unwrap()) });

        let mut smt = MNT4PoseidonSmt::new_unitialized(
            num_leaves,
            false,
            None,
            String::from("./db_leaves_compare_merkle_trees_mnt4_1"),
            String::from("./db_cache_compare_merkle_trees_mnt4_1")
        ).unwrap();
        smt.process_leaves_normal(leaves_to_process);

        //=============================================

        let mut leaves = Vec::new();
        leaves.push(MNT4753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        assert_eq!(tree.root(), smt.state.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt4_2() {
        use algebra::{
            fields::mnt4753::Fr,
            UniformRand,
        };

        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        let num_leaves = 32;
        let mut leaves = Vec::new();
        for _ in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new_unitialized(
            num_leaves,
            false,
            None,
            String::from("./db_leaves_compare_merkle_trees_mnt4_2"),
            String::from("./db_cache_compare_merkle_trees_mnt4_2")
        ).unwrap();
        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        for i in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            smt.insert_leaf(
                Coord{height:0, idx:i},
                f,
            );
        }

        assert_eq!(tree.root(), smt.state.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt4_3() {
        use algebra::{
            fields::mnt4753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves = Vec::new();
        leaves.push(MNT4753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new_unitialized(
            num_leaves,
            false,
            None,
            String::from("./db_leaves_compare_merkle_trees_mnt4_3"),
            String::from("./db_cache_compare_merkle_trees_mnt4_3")
        ).unwrap();
        
        smt.insert_leaf(Coord{height:0, idx:0}, MNT4753Fr::from_str("1").unwrap());
        smt.insert_leaf(Coord{height:0, idx:9}, MNT4753Fr::from_str("2").unwrap());
        smt.insert_leaf(Coord{height:0, idx:16}, MNT4753Fr::from_str("10").unwrap());
        smt.insert_leaf(Coord{height:0, idx:29}, MNT4753Fr::from_str("3").unwrap());
        smt.remove_leaf(Coord{height:0, idx:16});

        assert_eq!(tree.root(), smt.state.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    struct MNT6753FieldBasedMerkleTreeParams;

    impl FieldBasedMerkleTreeConfig for MNT6753FieldBasedMerkleTreeParams {
        const HEIGHT: usize = 6;
        type H = MNT6PoseidonHash;
    }

    type MNT6753FieldBasedMerkleTree = NaiveMerkleTree<MNT6753FieldBasedMerkleTreeParams>;

    pub type MNT6PoseidonSmt = BigMerkleTree<MNT6753Fr, MNT6753MHTPoseidonParameters, MNT6PoseidonHash>;

    #[test]
    fn compare_merkle_trees_mnt6_1() {
        use algebra::{
            fields::mnt6753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves_to_process: Vec<OperationLeaf<MNT6753Fr>> = Vec::new();

        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 0 }, action: ActionLeaf::Insert, hash: Some(MNT6753Fr::from_str("1").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 9 }, action: ActionLeaf::Insert, hash: Some(MNT6753Fr::from_str("2").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT6753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 29 }, action: ActionLeaf::Insert, hash: Some(MNT6753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT6753Fr::from_str("3").unwrap()) });

        let mut smt = MNT6PoseidonSmt::new_unitialized(
            num_leaves,
            false,
            None,
            String::from("./db_leaves_compare_merkle_trees_mnt6_1"),
            String::from("./db_cache_compare_merkle_trees_mnt6_1")
        ).unwrap();
        smt.process_leaves_normal(leaves_to_process);

        //=============================================

        let mut leaves = Vec::new();
        leaves.push(MNT6753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT6753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT6753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT6753FieldBasedMerkleTree::new(&leaves).unwrap();

        assert_eq!(tree.root(), smt.state.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt6_2() {
        use algebra::{
            fields::mnt4753::Fr,
            UniformRand,
        };

        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        let num_leaves = 32;
        let mut leaves = Vec::new();
        for _ in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new_unitialized(
            num_leaves,
            false,
            None,
            String::from("./db_leaves_compare_merkle_trees_mnt6_2"),
            String::from("./db_cache_compare_merkle_trees_mnt6_2")
        ).unwrap();
        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        for i in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            smt.insert_leaf(
                Coord{height:0, idx:i},
                f,
            );
        }

        assert_eq!(tree.root(), smt.state.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt6_3() {
        use algebra::{
            fields::mnt4753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves = Vec::new();
        leaves.push(MNT4753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new_unitialized(
            num_leaves,
            false,
            None,
            String::from("./db_leaves_compare_merkle_trees_mnt6_3"),
            String::from("./db_cache_compare_merkle_trees_mnt6_3")).unwrap();
        smt.insert_leaf(Coord{height:0, idx:0}, MNT4753Fr::from_str("1").unwrap());
        smt.insert_leaf(Coord{height:0, idx:9}, MNT4753Fr::from_str("2").unwrap());
        smt.insert_leaf(Coord{height:0, idx:16}, MNT4753Fr::from_str("10").unwrap());
        smt.insert_leaf(Coord{height:0, idx:29}, MNT4753Fr::from_str("3").unwrap());
        smt.remove_leaf(Coord{height:0, idx:16});
        println!("{:?}", smt.state.root);

        assert_eq!(tree.root(), smt.state.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn test_persistency() {
        let root = MNT4753Fr::new(
            BigInteger768([
                17131081159200801074,
                9006481350618111567,
                12051725085490156787,
                2023238364439588976,
                13194888104290656497,
                14162537977718443379,
                13575626123664189275,
                9267800406229717074,
                8973990559932404408,
                1830585533392189796,
                16667600459761825175,
                476991746583444
            ])
        );

        // create a persistent smt in a separate scope
        {
            let mut smt = MNT4PoseidonSmt::new_unitialized(
                32,
                true,
                Some(String::from("./persistency_test_info")),
                String::from("./db_leaves_persistency_test_info"),
                String::from("./db_cache_persistency_test_info")
            ).unwrap();

            //Insert some leaves in the tree
            smt.insert_leaf(Coord{height:0, idx:0}, MNT4753Fr::from_str("1").unwrap());
            smt.insert_leaf(Coord{height:0, idx:9}, MNT4753Fr::from_str("2").unwrap());

            // smt gets dropped but its info should be saved
        }
        // files and directories should have been created
        assert!(Path::new("./persistency_test_info").exists());
        assert!(Path::new("./db_leaves_persistency_test_info").exists());
        assert!(Path::new("./db_cache_persistency_test_info").exists());

        // create a non-persistent smt in another scope by restoring the previous one
        {
            let mut smt = MNT4PoseidonSmt::new(
                false,
                String::from("./persistency_test_info"),
                String::from("./db_leaves_persistency_test_info"),
                String::from("./db_cache_persistency_test_info")
            ).unwrap();

            // insert other leaves
            smt.insert_leaf(Coord { height: 0, idx: 16 }, MNT4753Fr::from_str("10").unwrap());
            smt.insert_leaf(Coord { height: 0, idx: 29 }, MNT4753Fr::from_str("3").unwrap());

            // if truly state has been kept, then the equality below must pass, since `root` was
            // computed in one go with another smt
            assert_eq!(root, smt.state.root);

            // smt gets dropped and state and dbs are deleted
        }

        // files and directories should have been deleted
        assert!(!Path::new("./persistency_test_info").exists());
        assert!(!Path::new("./db_leaves_persistency_test_info").exists());
        assert!(!Path::new("./db_cache_persistency_test_info").exists());
    }
}
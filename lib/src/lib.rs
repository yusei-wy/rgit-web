use rgit::fs::{inmem::InMemFileSystem, FileSystem};
use rgit::object::GitObject;
use rgit::Git;
use std::result::Result;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;

fn log(s: &str) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub struct Context {
    git: Git<InMemFileSystem>,
}

#[wasm_bindgen]
impl Context {
    pub fn new() -> Self {
        let fs = InMemFileSystem::init();
        Self { git: Git::new(fs) }
    }

    pub fn write(&mut self, path: String, text: String) -> Result<(), JsValue> {
        let result = self.git.filesystem.write(path, text.as_bytes());
        result.map_err(|x| JsValue::from(x.to_string()))
    }

    pub fn read(&self, path: String) -> Result<Vec<u8>, JsValue> {
        self.git
            .filesystem
            .read(path)
            .map_err(|x| JsValue::from(x.to_string()))
    }

    pub fn cat_file_p(&self, hash: String) -> Result<JsValue, JsValue> {
        let obj = self
            .git
            .read_object(hash)
            .and_then(|x| self.git.cat_file_p(&x))
            .map_err(|x| JsValue::from(x.to_string()))?;

        JsValue::from_serde(&obj).map_err(|x| JsValue::from(x.to_string()))
    }

    pub fn git_add(&mut self, path: String) -> Result<(), JsValue> {
        let bytes = self.read(path.clone())?;

        // git hash-object -w path
        let blob = self
            .git
            .hash_object(&bytes)
            .map(GitObject::Blob)
            .map_err(|x| JsValue::from(x.to_string()))?;
        self.git
            .write_object(&blob)
            .map_err(|x| JsValue::from(x.to_string()))?;

        let index = self
            .git
            .update_index(&blob.calc_hash(), path)
            .map_err(|x| JsValue::from(x.to_string()))?;
        self.git
            .write_index(&index)
            .map_err(|x| JsValue::from(x.to_string()))?;

        Ok(())
    }

    pub fn git_commit(&mut self, message: String) -> Result<(), JsValue> {
        // git write-tree
        let tree = self
            .git
            .write_tree()
            .map(GitObject::Tree)
            .map_err(|x| JsValue::from(x.to_string()))?;
        self.git
            .write_object(&tree)
            .map_err(|x| JsValue::from(x.to_string()))?;

        let tree_hash = tree.calc_hash();
        // echo message | git commit-tree <hash>
        let commit = self
            .git
            .commit_tree(
                "yusei-wy".to_string(),
                "yusei.kasa@gmail.com".to_string(),
                hex::encode(tree_hash),
                message,
            )
            .map(GitObject::Commit)
            .map_err(|x| JsValue::from(x.to_string()))?;
        self.git
            .write_object(&commit)
            .map_err(|x| JsValue::from(x.to_string()))?;

        // git update-ref refs/heads/master <hash>
        let head_ref = self
            .git
            .head_ref()
            .map_err(|x| JsValue::from(x.to_string()))?;
        self.git
            .update_ref(head_ref, &commit.calc_hash())
            .map_err(|x| JsValue::from(x.to_string()))?;

        Ok(())
    }

    pub fn read_head(&self) -> Result<String, JsValue> {
        self.git
            .head_ref()
            .and_then(|x| self.git.read_ref(x))
            .map_err(|x| JsValue::from(x.to_string()))
    }
}

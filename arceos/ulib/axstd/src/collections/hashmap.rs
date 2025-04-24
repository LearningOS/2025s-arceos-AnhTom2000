#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
const LOAD_FACTOR: f32 = 0.75;
const PRIME_BASE: u64 = 257; // 选择一个质数作为基数

// 自定义hash函数构造器
struct FastPolynomialHasher(u64);

impl FastPolynomialHasher {
    fn new() -> Self {
        Self(0)
    }
}

impl Hasher for FastPolynomialHasher{
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes{
            self.0  = self.0.wrapping_mul(PRIME_BASE).wrapping_add(byte as u64);
        }
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

// 键值对构建
struct Entry<K, V> {
    key: K,
    value: V,
    hash: u64,
}

// 哈希表构建
pub struct HashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
    size: usize,
}

impl<K,V> HashMap<K,V> where K:Hash + Eq{
    pub fn new() -> Self{
        HashMap {
            buckets :Vec::with_capacity(0), 
            size: 0,
        }
    }
    // hash函数
    fn hash(&self, key: &K) -> u64 {
       let mut hasher = FastPolynomialHasher::new();
       key.hash(&mut hasher);
       hasher.finish()
    }
    // put 操作
    pub fn insert(&mut self,key:K,value:V){
        if self.buckets.is_empty() || self.size >= (self.buckets.len() as f32 * LOAD_FACTOR) as usize { // 判断是否扩容
            self.resize();
        }
        let hash = self.hash(&key);
        let buckets_index = (hash as usize) % self.buckets.len(); //计算桶的位置
        for entry in &mut self.buckets[buckets_index] {
            if  entry.key == key {
                entry.value = value;
                return;
            }
        }
        self.buckets[buckets_index].push(Entry { // 默认尾插法解决hash冲突
            key,
            value,
            hash,
        });
        self.size += 1;
    }
    // get操作
    pub fn get(&mut self,key : &K)->Option<&V>{
        let hash = self.hash(key);
        let buckets_index = (hash as usize) % self.buckets.len();
        for entry in &mut self.buckets[buckets_index] {
            if entry.key == *key{
                return Some(&entry.value);
            }
        }
        return None;
    }
    // 扩容操作
    fn resize(&mut self){
        if self.buckets.is_empty() {
            let mut original_buckets:Vec<Vec<Entry<K,V>>> =  Vec::with_capacity(8);
            original_buckets.resize_with(8, Vec::new);
            self.buckets = original_buckets; // 初始化8个桶
        }else {
            let new_size = self.buckets.len() * 2;// 默认扩容两倍
            let mut new_buckets =  Vec::with_capacity(new_size);
            new_buckets.resize_with(new_size, Vec::new);// 初始化新桶
            for bucket in self.buckets.drain(..) {
                for entry in bucket{
                    let new_bucket_index = (entry.hash as usize) % new_size;
                    new_buckets[new_bucket_index].push(entry);
                }
            }
            self.buckets = new_buckets;
        }
    }

    pub fn iter(&self) -> Iter<K,V> {
        Iter {
            buckets: &self.buckets,
            bucket_index: 0,
            entry_index: 0,
        }
    }
}

// 迭代器实现
pub struct Iter<'a,K,V>{
    buckets: &'a Vec<Vec<Entry<K,V>>>,
    bucket_index: usize,
    entry_index: usize,
}
impl<'a,K,V> Iterator for Iter<'a,K,V>{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_index < self.buckets.len() {
            if self.entry_index < self.buckets[self.bucket_index].len() {
                let entry = &self.buckets[self.bucket_index][self.entry_index];
                self.entry_index += 1;
                return Some((&entry.key, &entry.value));
            } else {
                self.bucket_index += 1;
                self.entry_index = 0;
            }
        }
        None
    }
}
#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    Env,
    String,
    Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: u32,
}

#[contracttype]
pub enum DataKey {
    Product(u32),
}

#[contract]
pub struct CafeContract;

#[contractimpl]
impl CafeContract {

    // Thêm sản phẩm
    pub fn add_product(
        env: Env,
        id: u32,
        name: String,
        price: u32,
    ) {
        let product = Product {
            id,
            name,
            price,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);
    }

    // Lấy thông tin sản phẩm
    pub fn get_product(
        env: Env,
        id: u32,
    ) -> Product {

        env.storage()
            .persistent()
            .get(&DataKey::Product(id))
            .unwrap()
    }

    // Cập nhật giá
    pub fn update_price(
        env: Env,
        id: u32,
        new_price: u32,
    ) {

        let mut product: Product = env
            .storage()
            .persistent()
            .get(&DataKey::Product(id))
            .unwrap();

        product.price = new_price;

        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);
    }

    // Xóa sản phẩm
    pub fn delete_product(
        env: Env,
        id: u32,
    ) {

        env.storage()
            .persistent()
            .remove(&DataKey::Product(id));
    }
}
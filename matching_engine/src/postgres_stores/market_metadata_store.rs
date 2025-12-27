use crate::market_metadata::MarketMetadata;
use crate::market_metadata::{MarketMetadataStoreRead, MarketMetadataStoreWrite};
use crate::schema::market_images;
use crate::schema::market_metadata;
use diesel::insert_into;
use diesel::prelude::*;
use ethers::types::Bytes;
use ethers::types::H256;
use ethers::types::U256;

use super::models::MarketMetadataRecord;
use super::models::MarketMetadataStoreDB;

impl MarketMetadataStoreRead for MarketMetadataStoreDB {
    fn count_markets(&self) -> usize {
        0
    }

    fn get_all_markets(&self) -> Vec<MarketMetadata> {
        unimplemented!("get_all_markets")
    }

    fn get_median_proof_time(&self) -> U256 {
        U256::zero()
    }

    fn get_median_proof_time_market_wise(&self, market_id: &U256) -> U256 {
        U256::zero()
    }

    fn get_median_proof_cost(&self) -> U256 {
        U256::zero()
    }

    fn get_median_proof_cost_market_wise(&self, market_id: &U256) -> U256 {
        U256::zero()
    }

    fn get_market_by_market_id(&self, market_id: &U256) -> Option<MarketMetadata> {
        None
    }

    fn get_earnings(&self, market_id: &U256) -> Option<U256> {
        // Safely access the earnings map
        None
    }
}

impl MarketMetadataStoreWrite for MarketMetadataStoreDB {
    fn insert(&mut self, market: MarketMetadata) {
        let conn = &mut self.pool.get().expect("DB connection error");

        let market_id_str = market.market_id.to_string();
        let new_meta = MarketMetadataRecord {
            market_id: market.market_id.to_string(),
            verifier: market.verifier.to_string(),
            activation_block: market.activation_block.to_string(),
            metadata: market.metadata.to_vec(),
            proof_time: "0".to_string(),
            proof_cost: "0".to_string(),
            earnings: "0".to_string(),
        };

        insert_into(market_metadata::table)
            .values(&new_meta)
            .execute(conn)
            .expect("Error inserting market metadata");

        // Insert images into market_images.
        // For prover images:
        for image in market.prover_images {
            let new_img = (
                market_images::market_id.eq(market_id_str.clone()),
                market_images::image_type.eq("prover"),
                market_images::image.eq(image.to_string()),
            );
            insert_into(market_images::table)
                .values(&new_img)
                .execute(conn)
                .expect("Error inserting prover image");
        }

        // For IVS images:
        for image in market.ivs_images {
            let new_img = (
                market_images::market_id.eq(market_id_str.clone()),
                market_images::image_type.eq("ivs"),
                market_images::image.eq(image.to_string()),
            );
            insert_into(market_images::table)
                .values(&new_img)
                .execute(conn)
                .expect("Error inserting IVS image");
        }
    }

    fn remove_by_market_id(&mut self, market_id: &U256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();

        // Delete from market_metadata.
        diesel::delete(
            market_metadata::table.filter(market_metadata::market_id.eq(&market_id_str)),
        )
        .execute(conn)
        .expect("Error deleting market metadata");

        // Delete associated images.
        diesel::delete(market_images::table.filter(market_images::market_id.eq(&market_id_str)))
            .execute(conn)
            .expect("Error deleting market images");
    }

    fn note_proof_submission_stats_for_valid_proof(
        &mut self,
        market_id: &U256,
        proof_time_val: U256,
        proof_cost_val: U256,
    ) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();

        // Update proof_time and proof_cost.

        // Need to reimplment this
        diesel::update(
            market_metadata::table.filter(market_metadata::market_id.eq(&market_id_str)),
        )
        .set((
            market_metadata::proof_time.eq(proof_time_val.to_string()),
            market_metadata::proof_cost.eq(proof_cost_val.to_string()),
        ))
        .execute(conn)
        .expect("Error updating proof stats");

        // Update earnings: read current earnings, add proof_cost, update.
        let current_earnings: Option<String> = market_metadata::table
            .filter(market_metadata::market_id.eq(&market_id_str))
            .select(market_metadata::earnings)
            .first(conn)
            .optional()
            .expect("Error reading earnings");
        let new_earnings = if let Some(current) = current_earnings {
            let current_val: U256 = current.parse().unwrap_or(U256::zero());
            (current_val + proof_cost_val).to_string()
        } else {
            proof_cost_val.to_string()
        };

        diesel::update(
            market_metadata::table.filter(market_metadata::market_id.eq(&market_id_str)),
        )
        .set(market_metadata::earnings.eq(new_earnings))
        .execute(conn)
        .expect("Error updating earnings");
    }

    fn note_proof_submission_stats_for_invalid_inputs(
        &mut self,
        market_id: &U256,
        proof_cost_val: U256,
    ) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();

        // Update proof_cost.
        diesel::update(
            market_metadata::table.filter(market_metadata::market_id.eq(&market_id_str)),
        )
        .set(market_metadata::proof_cost.eq(proof_cost_val.to_string()))
        .execute(conn)
        .expect("Error updating proof cost");

        // Update earnings similarly.
        let current_earnings: Option<String> = market_metadata::table
            .filter(market_metadata::market_id.eq(&market_id_str))
            .select(market_metadata::earnings)
            .first(conn)
            .optional()
            .expect("Error reading earnings");
        let new_earnings = if let Some(current) = current_earnings {
            let current_val: U256 = current.parse().unwrap_or(U256::zero());
            (current_val + proof_cost_val).to_string()
        } else {
            proof_cost_val.to_string()
        };

        diesel::update(
            market_metadata::table.filter(market_metadata::market_id.eq(&market_id_str)),
        )
        .set(market_metadata::earnings.eq(new_earnings))
        .execute(conn)
        .expect("Error updating earnings");
    }

    fn add_prover_image(&mut self, market_id: U256, image: H256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();
        let image_str = image.to_string();

        let new_image = (
            market_images::market_id.eq(&market_id_str),
            market_images::image_type.eq("prover"),
            market_images::image.eq(image_str),
        );
        insert_into(market_images::table)
            .values(&new_image)
            .execute(conn)
            .expect("Error inserting prover image");
    }

    fn remove_prover_image(&mut self, market_id: U256, image: H256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();
        let image_str = image.to_string();

        diesel::delete(
            market_images::table
                .filter(market_images::market_id.eq(&market_id_str))
                .filter(market_images::image_type.eq("prover"))
                .filter(market_images::image.eq(&image_str)),
        )
        .execute(conn)
        .expect("Error deleting prover image");
    }

    fn add_ivs_image(&mut self, market_id: U256, image: H256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();
        let image_str = image.to_string();

        let new_image = (
            market_images::market_id.eq(&market_id_str),
            market_images::image_type.eq("ivs"),
            market_images::image.eq(image_str),
        );
        insert_into(market_images::table)
            .values(&new_image)
            .execute(conn)
            .expect("Error inserting ivs image");
    }

    fn remove_ivs_image(&mut self, market_id: U256, image: H256) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();
        let image_str = image.to_string();

        diesel::delete(
            market_images::table
                .filter(market_images::market_id.eq(&market_id_str))
                .filter(market_images::image_type.eq("ivs"))
                .filter(market_images::image.eq(&image_str)),
        )
        .execute(conn)
        .expect("Error deleting ivs image");
    }

    fn update_marketmeta_bytes(&mut self, market_id: U256, metadata_bytes: Bytes) {
        let conn = &mut self.pool.get().expect("DB connection error");
        let market_id_str = market_id.to_string();

        diesel::update(
            market_metadata::table.filter(market_metadata::market_id.eq(&market_id_str)),
        )
        .set(market_metadata::metadata.eq(metadata_bytes.to_vec()))
        .execute(conn)
        .expect("Error updating market metadata bytes");
    }
}

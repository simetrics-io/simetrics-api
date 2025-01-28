use rustis::{
    client::Client,
    commands::{GenericCommands, StringCommands},
    resp::{PrimitiveResponse, SingleArg},
    Error,
};
use serde::de::DeserializeOwned;
use tracing::{debug, error};

use crate::Exception;

/// Redis client.
#[derive(Clone)]
pub struct Redis {
    /// Redis connection.
    pub client: Client,
}

impl Redis {
    /// Create a new Redis client and connect to it.
    ///
    /// # Returns
    ///
    /// Redis client.
    pub async fn new() -> Self {
        debug!("Connecting to Redis ...");
        let url = std::env::var("REDIS_URL").expect("REDIS_URL is missed");

        let client = Client::connect(url)
            .await
            .expect("Failed to connect to Redis");

        debug!("Connected to Redis");

        Redis { client }
    }

    /// Check if a key exists in Redis.
    ///
    /// # Arguments
    ///
    /// - `key`: Key to check.
    ///
    /// # Returns
    ///
    /// Flag indicating if the key exists.
    pub async fn exists(&mut self, key: &str) -> bool {
        match self.client.exists(key).await {
            Ok(value) => value > 0,
            Err(err) => {
                error!("Failed to check key: {:?} with error: {:?}", key, err);
                false
            }
        }
    }

    /// Get a value from Redis.
    ///
    /// # Arguments
    ///
    /// - `key`: Key to get the value.
    ///
    /// # Returns
    ///
    /// Value from Redis, if available.
    pub async fn get<T: PrimitiveResponse + Send + DeserializeOwned>(
        &mut self,
        key: &str,
    ) -> Option<T> {
        match self.client.get(key).await {
            Ok(value) => Some(value),
            Err(err) => {
                error!("Failed to get key: {:?} with error: {:?}", key, err);
                None
            }
        }
    }

    /// Set a value in Redis.
    ///
    /// # Arguments
    ///
    /// - `key`: Key to set.
    /// - `value`: Value to set.
    /// - `ttl`: Time to live in seconds, if applicable.
    ///
    /// # Returns
    ///
    /// Result of the operation.
    pub async fn set<T: SingleArg>(
        &mut self,
        key: &str,
        value: T,
        ttl: Option<u64>,
    ) -> Result<(), Exception> {
        if let Some(ttl) = ttl {
            if let Err(err) = self.client.setex(key, ttl, value).await {
                error!("Failed to set key: {:?} with error: {:?}", key, err);
                return Err(Exception::InternalError);
            }
        } else if let Err(err) = self.client.set(key, value).await {
            error!("Failed to set key: {:?} with error: {:?}", key, err);
            return Err(Exception::InternalError);
        }

        Ok(())
    }

    /// Delete a value from Redis.
    ///
    /// # Arguments
    ///
    /// - `key`: Key to delete.
    ///
    /// # Returns
    ///
    /// Result of the operation.
    pub async fn delete(&mut self, key: &str) -> Result<usize, Error> {
        self.client.del(key).await
    }
}

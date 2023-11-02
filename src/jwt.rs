use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json;

fn get_current_time() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_epoch.as_secs()
}

#[derive(Clone)]
pub struct JWT {
    secret: String,
}

impl JWT {
    pub fn new(secret: String) -> JWT {
        JWT { secret }
    }
    
    pub fn generate<T: Serialize>(&self, claims: &T) -> String {
      let header = Header::default();
      let key = EncodingKey::from_secret(self.secret.as_ref());
      let token = encode(&header, &claims, &key).unwrap();
      token
    }

    fn decode<T: for<'de> Deserialize<'de> + DeserializeOwned>(
        &self,
        value: &str,
    ) -> Result<TokenData<T>, String> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let validation = Validation::default();
        decode::<T>(value, &decoding_key, &validation).map_err(|_| "Invalid token".to_string())
    }

    pub fn get_claims<T: for<'de> Deserialize<'de> + DeserializeOwned>(
        &self,
        value: &str,
    ) -> Result<T, String> {
        let claims = self.decode::<T>(value)?
            .claims;

        Ok(claims)
    }

    pub fn validate(&self, token: &str) -> Result<(), String> {
        let token_data = self.decode::<serde_json::Value>(token)?;

        let exp = match token_data.claims.get("exp") {
            Some(exp) => match exp.as_u64() {
                Some(exp) => exp,
                None => return Err("Invalid exp claim".to_string()),
            },
            None => return Err("Missing exp claim".to_string()),
        };
    
        let now = get_current_time();
        if now >= exp {
            return Err("Token has expired".to_string());
        }

        Ok(())
    }
}

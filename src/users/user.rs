// PBKDF2 < bcrypt < scrypt


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAddress {
    pub share_address: String,
    pub anonymous_address:String,
    pub evm_anonymous_address:String,
    pub transaction_hash:String,
    pub signature:String,
    pub signature_data:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
    pub url:String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryAll {
    pub limit: i16,
    pub offset:i16,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct AddressExperience{
    pub address:String,
    pub experience: String,
}


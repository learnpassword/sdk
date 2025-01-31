/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganizationUserResetPasswordEnrollmentRequestModel {
    #[serde(rename = "resetPasswordKey", skip_serializing_if = "Option::is_none")]
    pub reset_password_key: Option<String>,
}

impl OrganizationUserResetPasswordEnrollmentRequestModel {
    pub fn new() -> OrganizationUserResetPasswordEnrollmentRequestModel {
        OrganizationUserResetPasswordEnrollmentRequestModel {
            reset_password_key: None,
        }
    }
}

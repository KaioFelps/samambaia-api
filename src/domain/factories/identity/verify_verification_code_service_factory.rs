use crate::domain::services::identity::verify_verification_code_service::VerifyVerificationCodeService;
use crate::infra::factories::external_services::hotels_utilities_external_services_factory;

pub fn exec() -> VerifyVerificationCodeService {
    VerifyVerificationCodeService::new(hotels_utilities_external_services_factory::exec())
}

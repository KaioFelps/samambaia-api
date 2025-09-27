use crate::configs::hotels::{AvailableHotel, HOTELS_CONFIG};
use crate::domain::external_services::hotel_utilities_external_services::HotelUtilitiesExternalService;
use crate::infra::hotels::habblet::HabbletUtilitiesExternalServices;
use crate::infra::hotels::habblive::HabbliveUtilitiesExternalServices;

pub fn exec() -> Box<dyn HotelUtilitiesExternalService> {
    let utilities_services: Box<dyn HotelUtilitiesExternalService> =
        match HOTELS_CONFIG.active_hotel {
            AvailableHotel::Habblet => Box::new(HabbletUtilitiesExternalServices::new(
                HOTELS_CONFIG.get_active_hotel_info().api_base_url,
            )),
            AvailableHotel::Habblive => Box::new(HabbliveUtilitiesExternalServices::new(
                HOTELS_CONFIG.get_active_hotel_info().api_base_url,
            )),
        };

    utilities_services
}

use crate::configs::hotels::{AvailableHotel, HOTELS_CONFIG};
use crate::infra::imager::Imager;
use crate::infra::imager::providers::{
    HabbletImagerProvider,
    HabbliveImagerProvider,
    ImagerProvider,
};

pub struct ImagerFactory;

impl ImagerFactory {
    pub fn get_imager() -> Imager {
        let hotels_config = HOTELS_CONFIG.get_active_hotel_info();

        let provider: Box<dyn ImagerProvider> = match HOTELS_CONFIG.active_hotel {
            AvailableHotel::Habblet => Box::new(HabbletImagerProvider::new(
                hotels_config.api_base_url,
                hotels_config.imager_url,
            )),
            AvailableHotel::Habblive => {
                Box::new(HabbliveImagerProvider::new(hotels_config.imager_url))
            }
        };

        Imager::new(provider)
    }
}

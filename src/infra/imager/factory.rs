use crate::configs::hotels::{AvailableHotel, HOTELS_CONFIG};
use crate::error::SamambaiaError;
use crate::infra::imager::Imager;
use crate::infra::imager::providers::{
    HabbletImagerProvider,
    HabbliveImagerProvider,
    ImagerProvider,
};

pub struct ImagerFactory;

impl ImagerFactory {
    pub fn get_imager() -> Result<Imager, SamambaiaError> {
        let hotels_config = HOTELS_CONFIG.get_active_hotel_info();

        let provider = match HOTELS_CONFIG.active_hotel {
            AvailableHotel::Habblet => {
                HabbletImagerProvider::new(hotels_config.api_base_url, hotels_config.imager_url)
                    .map(|provider| Box::new(provider) as Box<dyn ImagerProvider>)
            }
            AvailableHotel::Habblive => Ok(Box::new(HabbliveImagerProvider::new(
                hotels_config.imager_url,
            )) as Box<dyn ImagerProvider>),
        }?;

        Ok(Imager::new(provider))
    }
}

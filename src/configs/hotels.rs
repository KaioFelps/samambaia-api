use std::sync::LazyLock;

pub struct HotelInfo {
    pub imager_url: &'static str,
    pub api_base_url: &'static str,
}

pub static HOTELS_CONFIG: LazyLock<HotelsConfig> = LazyLock::new(HotelsConfig::initialize);

pub enum AvailableHotel {
    Habblive,
    Habblet,
}

pub struct HotelsConfig {
    pub active_hotel: AvailableHotel,
    pub habblet: HotelInfo,
    pub habblive: HotelInfo,
}

impl HotelsConfig {
    pub fn initialize() -> Self {
        Self {
            active_hotel: AvailableHotel::Habblet,
            habblet: HotelInfo {
                imager_url: "https://imaging.habblet.city/avatarimage",
                api_base_url: "https://api.habblet.city",
            },
            habblive: HotelInfo {
                imager_url: "https://habblive.in/imager.php",
                api_base_url: "https://habblive.in/api",
            },
        }
    }

    pub fn get_active_hotel_info(&self) -> &HotelInfo {
        match self.active_hotel {
            AvailableHotel::Habblet => &self.habblet,
            AvailableHotel::Habblive => &self.habblive,
        }
    }
}

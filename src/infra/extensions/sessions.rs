use actix_session::{Session, SessionExt, SessionInsertError};
use actix_web::HttpRequest;
use serde_json::Map;

use crate::configs::app::SESSION_FLASH_KEY;

pub trait SessionHelpers {
    fn flash<T: ToString>(req: &HttpRequest, key: &str, msg: T) -> Result<(), SessionInsertError>;

    fn flash_silently<T: ToString>(req: &HttpRequest, key: &str, msg: T);
}

impl SessionHelpers for Session {
    #[inline]
    fn flash<T: ToString>(req: &HttpRequest, key: &str, msg: T) -> Result<(), SessionInsertError> {
        let mut flash_messages = req
            .get_session()
            .remove(SESSION_FLASH_KEY)
            .map(|map| serde_json::from_str::<Map<_, _>>(&map).unwrap_or_default())
            .unwrap_or_default();

        flash_messages.insert(key.to_string(), msg.to_string().into());

        req.get_session()
            .insert(SESSION_FLASH_KEY, flash_messages)?;

        Ok(())
    }

    #[inline]
    fn flash_silently<T: ToString>(req: &HttpRequest, key: &str, msg: T) {
        let _ = Self::flash(req, key, msg);
    }
}

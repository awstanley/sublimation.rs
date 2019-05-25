//! Steam Client

/*
macro_rules! STEAM_INTERFACE_VERSION {
    () => {
        "SteamClient018".as_ptr()
    }
}
*/

use crate::{
    CVOID,
    //AppId,
    HSteamUser,
    HSteamPipe,
};

/// SteamClient
pub struct SteamClient {
    pub (crate) instance: CVOID,
    //pub (crate) current_app: AppId,
    pub (crate) user: HSteamUser,
    pub (crate) pipe: HSteamPipe,
}

impl Drop for SteamClient {
    fn drop(&mut self) {
        //self.shutdown();
    }
}

impl SteamClient {
    /// Shuts the client down.
    pub fn shutdown(&mut self) {
        unsafe {
            if self.user != 0 {
                super::C_STEAMAPI_ISTEAMCLIENT_RELEASEUSER(self.instance, self.pipe, self.user);
            }
            
            if self.pipe != 0 {
                super::C_STEAMAPI_ISTEAMCLIENT_BRELEASESTEAMPIPE(self.instance, self.pipe);
            }
        }
    }

    /// Creates a new SteamClient using the existing Steam User and Pipe.
    pub fn new() -> Option<Self> {
        unsafe {
            let pipe = super::C_STEAMAPI_GETHSTEAMPIPE();
            let user = super::C_STEAMAPI_GETHSTEAMUSER();
            let instance = super::C_STEAMCLIENT();

            // The other way to go for multiplexing;
            // not a great way to go and requires steamclient.ext workarounds
            /*
            let instance = super::C_STEAMINTERNAL_CREATEINTERFACE(
                STEAM_INTERFACE_VERSION!()
            );
            */
            //let pipe = super::C_STEAMAPI_ISTEAMCLIENT_CREATESTEAMPIPE(instance);
            //let user = super::C_STEAMAPI_ISTEAMCLIENT_CONNECTTOGLOBALUSER(instance, pipe);

            Some(SteamClient {
                instance,
                //current_app: 0,
                pipe,
                user,
            })
        }
    }
}
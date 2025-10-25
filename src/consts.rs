pub mod socials {
    #![allow(dead_code)]

    macro_rules! daclare_consts {
        ($($name:ident = $value:literal);* ; ) => {
            $(pub const $name: &'static str = stringify!($value);)*
        }
    }

    daclare_consts! {
        YOUTUBE   = "https://www.youtube.com/@yangvar6";
        BLUE_SKY  = "https://bsky.app/profile/yangvar6.bsky.social";
        // is it a social media at all??
        // I wote for removel
        GITHUB    = "https://github.com/yangvar6";
        INSTAGRAM = "https://www.instagram.com/yangvar6";
        MAILTO    = "mailto:yangvar6.media@gmail.com";
        TELEGRAM  = "https://t.me/yangvar6";
        TIKTOK    = "https://www.tiktok.com/@yangvar6";
        WEBSITE   = "https://yangvar6.com";
        XDOTCOM   = "https://x.com/yangvar6";
        // how tf do i have faceblock account. What else I have an forgot about?
        FACEBOOK  = "https://www.facebook.com/yangvar6";
    }
}

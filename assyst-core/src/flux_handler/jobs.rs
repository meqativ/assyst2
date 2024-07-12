use std::collections::HashMap;

use crate::flux_handler::flux_request::FluxRequest;

use super::limits::LIMITS;
use super::FluxHandler;

pub type FluxResult = anyhow::Result<Vec<u8>>;

impl FluxHandler {
    pub async fn ahshit(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "ah-shit");

        self.run_flux(request, limits.time).await
    }

    pub async fn aprilfools(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "april-fools");

        self.run_flux(request, limits.time).await
    }

    pub async fn back_tattoo(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "back-tattoo");

        self.run_flux(request, limits.time).await
    }

    pub async fn billboard(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "billboard");

        self.run_flux(request, limits.time).await
    }

    pub async fn bloom(
        &self,
        media: Vec<u8>,
        radius: Option<u64>,
        sharpness: Option<u64>,
        brightness: Option<u64>,
        user_id: u64,
    ) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;

        let limits = &LIMITS[tier];
        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        if let Some(r) = radius {
            options.insert("radius".to_owned(), r.to_string());
        };
        if let Some(s) = sharpness {
            options.insert("sharpness".to_owned(), s.to_string());
        };
        if let Some(b) = brightness {
            options.insert("brightness".to_owned(), b.to_string());
        };

        request.operation("bloom".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn blur(&self, media: Vec<u8>, power: Option<f32>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;

        let limits = &LIMITS[tier];
        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        if let Some(p) = power {
            options.insert("strength".to_owned(), p.to_string());
        };

        request.operation("blur".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn book(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "book");

        self.run_flux(request, limits.time).await
    }

    pub async fn caption(&self, media: Vec<u8>, text: String, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;

        let limits = &LIMITS[tier];
        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        options.insert("text".to_owned(), text);

        request.operation("caption".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn circuitboard(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "circuitboard");

        self.run_flux(request, limits.time).await
    }

    pub async fn deepfry(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "deepfry");

        self.run_flux(request, limits.time).await
    }

    pub async fn drip(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "drip");

        self.run_flux(request, limits.time).await
    }

    pub async fn femurbreaker(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "femurbreaker");

        self.run_flux(request, limits.time).await
    }

    pub async fn fisheye(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "fisheye");

        self.run_flux(request, limits.time).await
    }

    pub async fn flag(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "flag");

        self.run_flux(request, limits.time).await
    }

    pub async fn flag2(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "flag2");

        self.run_flux(request, limits.time).await
    }

    pub async fn flip(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "flip");

        self.run_flux(request, limits.time).await
    }

    pub async fn flop(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "flop");

        self.run_flux(request, limits.time).await
    }

    pub async fn fortune_cookie(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "fortune-cookie");

        self.run_flux(request, limits.time).await
    }

    pub async fn frame_shift(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "frame-shift");

        self.run_flux(request, limits.time).await
    }

    pub async fn frames(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "frames");

        self.run_flux(request, limits.time).await
    }

    pub async fn ghost(&self, media: Vec<u8>, depth: Option<u64>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;

        let limits = &LIMITS[tier];
        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        if let Some(d) = depth {
            options.insert("depth".to_owned(), d.to_string());
        };

        request.operation("ghost".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn gif(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "gif");

        self.run_flux(request, limits.time).await
    }

    pub async fn heart_locket(&self, media: Vec<u8>, text: String, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        options.insert("text".to_owned(), text);

        request.operation("heart-locket".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn magik(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "magik");

        self.run_flux(request, limits.time).await
    }

    pub async fn meme(&self, media: Vec<u8>, top: Option<String>, bottom: Option<String>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        top.map(|t| options.insert("top".to_owned(), t));
        bottom.map(|b| options.insert("bottom".to_owned(), b));

        request.operation("meme".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn ping_pong(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "ping-pong");

        self.run_flux(request, limits.time).await
    }

    pub async fn resize_absolute(&self, media: Vec<u8>, width: u32, height: u32, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;

        let limits = &LIMITS[tier];
        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        options.insert("width".to_owned(), width.to_string());
        options.insert("height".to_owned(), height.to_string());

        request.operation("resize".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn resize_scale(&self, media: Vec<u8>, scale: f32, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;

        let limits = &LIMITS[tier];
        let mut request = FluxRequest::new_with_input_and_limits(media, limits);

        let mut options = HashMap::new();
        options.insert("scale".to_owned(), scale.to_string());

        request.operation("resize".to_owned(), options);
        request.output();

        self.run_flux(request, limits.time).await
    }

    pub async fn reverse(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "reverse");

        self.run_flux(request, limits.time).await
    }

    pub async fn rubiks(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "rubiks");

        self.run_flux(request, limits.time).await
    }

    pub async fn siren(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "siren");

        self.run_flux(request, limits.time).await
    }

    pub async fn sweden(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "sweden");

        self.run_flux(request, limits.time).await
    }

    pub async fn terraria(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "terraria");

        self.run_flux(request, limits.time).await
    }

    pub async fn toaster(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "toaster");

        self.run_flux(request, limits.time).await
    }

    pub async fn valentine(&self, media: Vec<u8>, user_id: u64) -> FluxResult {
        let tier = self.get_request_tier(user_id).await?;
        let limits = &LIMITS[tier];

        let request = FluxRequest::new_basic(media, limits, "valentine");

        self.run_flux(request, limits.time).await
    }
}

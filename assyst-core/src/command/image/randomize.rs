use std::collections::HashMap;
use std::time::Duration;

use anyhow::Context;
use assyst_flux_iface::flux_request::FluxRequest;
use assyst_proc_macro::command;
use rand::{thread_rng, Rng};

use crate::command::arguments::Image;
use crate::command::{Availability, Category, CommandCtxt};

const VALID_EFFECTS: &[&str] = &[
    "bloom",
    "blur",
    "deepfry",
    "fisheye",
    "flip",
    "flop",
    "globe",
    "grayscale",
    "invert",
    "jpeg",
    "magik",
    "neon",
    "paint",
    "pixelate",
    "rainbow",
];

#[command(
    description = "apply random effects to an image",
    aliases = ["random", "randomise", "badcmd"],
    cooldown = Duration::from_secs(2),
    access = Availability::Public,
    category = Category::Image,
    usage = "[image] <effect count: 1-5>",
    examples = ["https://link.to.my/image.png 3"],
    send_processing = true
)]
pub async fn randomize(ctxt: CommandCtxt<'_>, source: Image, count: Option<u64>) -> anyhow::Result<()> {
    let mut effects: Vec<&str> = Vec::new();

    for _ in 0..count.unwrap_or(3).clamp(1, 5) {
        let next = loop {
            let tmp = VALID_EFFECTS[thread_rng().gen_range(0..VALID_EFFECTS.len())];
            if effects.last() != Some(&tmp) {
                break tmp;
            }
        };

        effects.push(next);
    }

    let limits = ctxt
        .assyst()
        .flux_handler
        .get_request_limits(ctxt.data.author.id.get(), ctxt.data.guild_id.map(twilight_model::id::Id::get))
        .await?;

    let mut request = FluxRequest::new_with_input_and_limits(source.0, &limits);
    for e in &effects {
        request.operation((*e).to_string(), HashMap::new());
    }

    request.output();

    let result = ctxt
        .assyst()
        .flux_handler
        .run_flux(request, limits.time)
        .await
        .context(format!("Applied effects: {}", effects.join(", ")))?;

    ctxt.reply((
        result,
        &format!(
            "Applied effects: {}",
            effects.iter().map(|e| format!("`{e}`")).collect::<Vec<_>>().join(", ")
        )[..],
    ))
    .await?;

    Ok(())
}

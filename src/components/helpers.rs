use super::{
    breaking_block_component::BreakingBlock,
    social_media_block_component::{SocialMediaBlock, SocialMediaEnum},
};

pub fn codeur_facile_description_block() -> BreakingBlock {
    let block_description = "Bienvenue sur le site de Codeur facile. Tout comme pour trouver ces quelques lignes de textes, le but de ce site est de proposer des expériences ludiques. La curiosité est de mise pour touver tout ce que ce site a à proposer. Il s'agit d'un site développé en Rust 🦀 avec le framework Yew, avec beaucoup d'❤️".to_string();
    BreakingBlock{
        id: 0,
        text: "Trouvez ce que je cache!".to_string(),
        text_destroyed: block_description,
        texture_url: "https://cdn.discordapp.com/attachments/1074719812871258122/1074719876909908058/breaking_cube.png".to_string(),
        broken_texture_url: "https://media.discordapp.net/attachments/1074719812871258122/1074722110892679178/breaking_cube.gif".to_string()
    }
}

pub fn youtube_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Youtube,
        social_media_url: "https://www.youtube.com/@codeurfacile".to_string(),
    }
}

pub fn github_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Github,
        social_media_url: "https://github.com/codeurfacile".to_string(),
    }
}

pub fn twitter_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Twitter,
        social_media_url: "https://twitter.com/CodeurFacile".to_string(),
    }
}

pub fn facebook_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Facebook,
        social_media_url: "https://www.facebook.com/profile.php?id=100089960516590".to_string(),
    }
}

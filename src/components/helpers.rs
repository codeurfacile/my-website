use chrono::NaiveDate;
use yew::html;

use super::{
    article_component::ArticleBlock,
    breaking_block_component::BreakingBlock,
    social_media_block_component::{SocialMediaBlock, SocialMediaEnum},
};

pub fn codeur_facile_description_block() -> BreakingBlock {
    let block_description = "Bienvenue sur le site de Codeur facile. Tout comme pour trouver ces quelques lignes de textes, le but de ce site est de proposer des expériences ludiques. La curiosité est de mise pour touver tout ce que ce site a à proposer. Il s'agit d'un site développé en Rust 🦀 avec le framework Yew, avec beaucoup d'❤️. Découvrez également ci-dessous les articles de blog pour la chaine Codeur Facile.".to_string();
    BreakingBlock{
        id: 0,
        text: "Trouvez ce que je cache!".to_string(),
        text_destroyed: block_description,
        texture_url: "https://cdn.discordapp.com/attachments/1074719812871258122/1074719876909908058/breaking_cube.png".to_string(),
        broken_texture_url: "https://media.discordapp.net/attachments/1074719812871258122/1074722110892679178/breaking_cube.gif".to_string()
    }
}

pub fn introduction_rust_article(expanded: bool) -> ArticleBlock {
    ArticleBlock {
        title: "Introduction à Rust".to_string(),
        text: html! {
            <>
                <h3>{"Qu'est-ce que Rust??"}</h3>
                <p>{"Rust est un langage de programmation informatique universel multi-paradigme. Il est typé statiquement et fournit un environnement de programmation rapide, fiable, productif et économe en mémoire ainsi qu’en énergie (non négligeable de nos jours)."}</p>
                <p>{"Il est idéal pour les services critiques en termes de performances, peut s’exécuter sur des appareils embarqués et s’intègre très facilement à d’autres langages de programmation. Il reste un excellent choix dans d'autres situations nottament grâce à une communauté très riche et active."}</p>
                <h3>{"Pourquoi Rust?"}</h3>
                <ul>
                    <li>{"Open-Source: Rust est un projet open source avec une multitude de ressources d’apprentissage et, comme mentionné ci-dessus, a été développé chez Mozilla Research. La Rust Foundation a été créée pour soutenir le maintien et le développement de ce langage."}</li>
                    <li>{"Comme C/C++ mais en mieux: les développeurs de logiciels utilisant les langages de programmation C et C++ sont confrontés depuis très longtemps à diverses erreurs de mémoire et à des problèmes de programmation récurrents. L’une des principales forces de Rust est qu’il résout efficacement ces problèmes."}</li>
                    <li>{"Borrow checker: une autre caractéristique importante est le vérificateur d’emprunt de Rust, une partie du compilateur qui garantit que les références de données ne dépassent pas les données auxquelles elles se réfèrent. En éliminant les bugs de violation de mémoire en les détectant au moment de la compilation, le garbage collector peut être oublié (et c'est une très bonne chose!)."}</li>
                    <li>{"Référence à vie: dans Rust, toutes les références de données ont des durées de vie. Cela signifie que les développeurs de logiciels peuvent définir dans quelle mesure cette référence de données est valide, corriger les bugs courants liés aux références invalides et distinguer le langage Rust des problèmes C et C++."}</li>
                    <li>{"Rust sûr 🆚 Rust non sûr: il existe deux modes d’écriture de code en Rust : Safe Rust et Unsafe Rust. Le premier mode assure l’efficacité du code en imposant diverses restrictions au programmeur. Le second mode offre une autonomie de programmation sans 🔓 mesures de sécurité du code."}</li>
                    <li>{"Programmation concurrente: Rust permet de programmer facilement des logiciels concurrents car il empêche les courses de données pendant la compilation. Les courses de données peuvent causer des problèmes lorsque au moins deux processus différents accèdent au même emplacement de mémoire de l’ordinateur simultanément."}</li>
                </ul>
                <h3>{"Comment commencer?"}</h3>
                <p>{"Dans cet article, nous allons voir comment commencer à développer en Rust sans installation d'aucun outil. Vous allez voir comment développer en Rust (ou même n'importe quel autre langage) en utilisant un EDI(Environnement de développement intégré) directement dans le Cloud. Personnellement, je préfère quand même installer tous mes outils en local et vous recommande de suivre la page de documentation suivante pour installer tous les outils nécessaires: "}<a href="https://www.rust-lang.org/learn/get-started">{"Getting started"}</a></p>
                <h4>{"Gitpod"}</h4>
                <p>{"Si vous souhaitez développer dans le cloud directement, nous allons nous intéresser à "}<a href="https://www.gitpod.io/">{"Gitpod."}</a></p>
                <p>{"Vous pouvez vous logger en utilisant votre compte Gitlab, Github ou Bitbucket:"}</p>
                <img src="https://cdn.discordapp.com/attachments/1075772415088533604/1075772522034896957/image.png" />
                <p>{"Une fois enregistré, il est possible de créer n’importe quel workspace pour se mettre à travailler directement. Pas besoin d’installer quoi que ce soit en plus:"}</p>
                <img src="https://cdn.discordapp.com/attachments/1075772415088533604/1075772677408702534/image.png" />
                <p>{"Il suffit de copier/coller l’URL de n’importe quel projet Git présent dans votre outil de sources préféré:"}</p>
                <img src="https://cdn.discordapp.com/attachments/1075772415088533604/1075773026748092456/image.png" />
                <p>{"Une fois l’environnement initialisé, il est possible de travailler directement dans l’IDE choisi comme dans l’exemple ci-dessous:"}</p>
                <img src="https://cdn.discordapp.com/attachments/1075772415088533604/1075773234735239208/image.png" />
                <p>{"Vous êtes prêts à travailler! Vous pouvez vérifier que tout fonctionne en utilisant les commandes suivantes dans le terminal:"}</p>
                <ul>
                    <li>{"cargo init test-project"}</li>
                    <li>{"cd test-project"}</li>
                    <li>{"cargo run"}</li>
                </ul>
                <p>{"Tout fonctionne bien si le texte `Hello World!` apparait dans le terminal. Nous verrons dans les prochains articles les bases du développement en Rust."}</p>
            </>
        },
        date: NaiveDate::from_ymd_opt(2023, 2, 16).unwrap(),
        expanded_default: expanded,
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

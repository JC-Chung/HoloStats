#![allow(clippy::unreadable_literal)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct VTuber {
    pub id: &'static str,
    pub youtube: Option<&'static str>,
    pub bilibili: Option<&'static str>,
}

impl VTuber {
    pub fn find_by_youtube_channel_id(channel_id: &str) -> Option<&VTuber> {
        VTUBERS.iter().find(|v| v.youtube == Some(channel_id))
    }
}

macro_rules! vtubers {
    ($( $(#[doc = $_:expr])? $id:ident, $youtube:expr, $bilibili:expr, )*) => {
        pub const VTUBERS: &[VTuber] = &[
            $(
                VTuber {
                    id: stringify!($id),
                    youtube: $youtube,
                    bilibili: $bilibili
                }
            ),*
        ];
    };
}

vtubers! {
    hololive,     Some("UCJFZiqLMntJufDCHc6bQixg"), Some("286700005"),
    yagoo,        Some("UCu2DMOGLeR_DSStCyeQpi5Q"), None,
    // Hololive Talents
    sora,         Some("UCp6993wxpyDPHUpavwDFqgg"), Some("286179206"),
    roboco,       Some("UCDqI2jOz0weumE8s7paEk6g"), Some("20813493"),
    miko,         Some("UC-hM6YJuNYVAmUWxeIr9FeA"), Some("366690056"),
    suisei,       Some("UC5CwaMl1eIgY8h02uZw7u8A"), Some("9034870"),
    // Hololive 1st Gen
    fubuki,       Some("UCdn5BQ06XqgXoAxIhbqw5Rg"), Some("332704117"),
    matsuri,      Some("UCQ0UDLQCjY0rmuxCDE38FGg"), Some("336731767"),
    haato,        Some("UC1CfXB_kRs3C-zaeTG3oGyg"), Some("339567211"),
    aki,          Some("UCFTLzh12_nrtzqBPsTCqenA"), Some("389857131"),
    mel,          Some("UCD8HOxPs4Xvsm8H0ZxXGiBw"), Some("389856447"),
    // Hololive 2nd Gen
    choco,        Some("UC1suqwovbL1kzsoaZgFZLKg"), Some("389858754"),
    choco_alt,    Some("UCp3tgHXw_HI0QMk1K8qh3gQ"), None,
    shion,        Some("UCXTpFs_3PqI41qX2d9tL2Rw"), Some("389857640"),
    aqua,         Some("UC1opHUrw8rvnsadT-iGp7Cg"), Some("375504219"),
    subaru,       Some("UCvzGlP9oQwU--Y0r9id_jnA"), Some("389859190"),
    ayame,        Some("UC7fk0CB07ly8oSl0aqKkqFg"), Some("389858027"),
    // Hololive 3rd Gen
    pekora,       Some("UC1DCedRgGHBdm81E1llLhOQ"), Some("443305053"),
    rushia,       Some("UCl_gCybOJRIgOXw6Qb4qJzQ"), Some("443300418"),
    flare,        Some("UCvInZx9h3jC2JzsIzoOebWg"), Some("454737600"),
    marine,       Some("UCCzUftO8KOVkV4wQG1vkUvg"), Some("454955503"),
    noel,         Some("UCdyqAaZDKHXg4Ahi7VENThQ"), Some("454733056"),
    // Hololive 4th Gen
    kanata,       Some("UCZlDXzGoo7d44bwdNObFacg"), Some("491474048"),
    coco,         Some("UCS9uQI-jC3DE0L4IpXyvr6w"), Some("491474049"),
    watame,       Some("UCqm3BQLlJfvkTsX_hvm0UmA"), Some("491474050"),
    towa,         Some("UC1uv2Oq6kNxgATlCiez59hw"), Some("491474051"),
    himemoriluna, Some("UCa9Y57gfeY0Zro_noHRVrnw"), Some("491474052"),
    // Hololive 5th Gen
    lamy,         Some("UCFKOVgVbGmX65RxO3EtH3iw"), Some("624252706"),
    nene,         Some("UCAWSyEs_Io8MtpY3m-zqILA"), Some("624252709"),
    botan,        Some("UCUKD-uaobj9jiqB-VXt71mA"), Some("624252710"),
    polka,        Some("UCK9V2B22uJYu3N7eR_BT9QA"), Some("624252712"),
    // Hololive Gamers
    mio,          Some("UCp-5t9SrOQwXMU7iIjQfARg"), Some("389862071"),
    okayu,        Some("UCvaTdHTWBGv3MKj3KVqJVCw"), Some("412135222"),
    korone,       Some("UChAnqc_AY5_I3Px5dig3X1Q"), Some("412135619"),
    // Innk Music
    azki,         Some("UC0TXe_LYZ4scaW2XMyi5_kw"), Some("389056211"),
    // Hololive Indonesia 1st Gen
    risu,         Some("UCOyYb1c43VlX9rc_lT6NKQw"), None,
    moona,        Some("UCP0BspO_AMEe3aQqqpo89Dg"), None,
    iofi,         Some("UCAoy6rzhSf4ydcYjJw3WoVg"), None,
    // Hololive Indonesia 2nd Gen
    ollie,        Some("UCYz_5n-uDuChHtLo7My1HnQ"), None,
    melfissa,     Some("UC727SQYUvx5pDDGQpTICNWg"), None,
    reine,        Some("UChgTyjG-pdNvxxhdsXfHQ5Q"), None,
    // Hololive English
    amelia,       Some("UCyl1z3jo3XHR1riLFKG5UAg"), Some("674600649"),
    calliope,     Some("UCL_qhgtOy0dy1Agp8vkySQg"), Some("674600645"),
    gura,         Some("UCoSrY_IQQVpmIRZ9Xf-y93g"), Some("674600648"),
    inanis,       Some("UCMwGHR0BTZuLsmjY_NT5Pwg"), Some("674600647"),
    kiara,        Some("UCHsx4Hqa-1ORjQTh9TYDhww"), Some("674600646"),
    // Holostars 1st Gen
    miyabi,       Some("UC6t3-_N8A6ME1JShZHHqOMw"), None,
    izuru,        Some("UCZgOv3YDEs-ZnZWDYVwJdmA"), None,
    aruran,       Some("UCKeAhJvy8zgXWbh9duVjIaQ"), None,
    rikka,        Some("UC9mf_ZVpouoILRY9NUIaK-w"), None,
    // Holostars 2nd Gen
    astel,        Some("UCNVEsYbiZjH5QLmGeSgTSzg"), None,
    temma,        Some("UCGNI4MENvnsymYjKiZwv9eg"), None,
    roberu,       Some("UCANDOlYTJT7N5jlRC3zfzVA"), None,
    // Holostars 3rd Gen
    shien,        Some("UChSvpZYRPh0FvG4SJGSga3g"), None,
    oga,          Some("UCwL7dgTxKo8Y4RFIKWaf8gA"), None,
    // Others
    luna,         Some("UCQYADFw7xEJ9oZSM5ZbqyBw"), Some("265224956"),
    nekomiya,     Some("UCevD0wKzJFpfIkvHOiQsfLQ"), Some("291296062"),
    tamaki,       Some("UC8NZiqKx6fsDT3AVcMiVFyA"), Some("12362451"),
    nana,         Some("UCbfv8uuUXt3RSJGEwxny5Rw"), Some("386900246"),
    ui,           Some("UCt30jJgChL8qeT9VPadidSw"), Some("2601367"),
    pochimaru,    Some("UC22BVlBsZc6ta3Dqz75NU6Q"), None,
    ayamy,        Some("UCr9p1ZjLKgfaoqNorY7PiWQ"), Some("521070071"),
}

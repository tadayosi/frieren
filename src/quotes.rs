use rand::prelude::SliceRandom;

pub fn quote(ja: bool) {
    let mut rnd = rand::thread_rng();
    let quote = QUOTES.choose(&mut rnd).unwrap();
    println!("❄️  > {}", if ja { quote.ja } else { quote.en });
}

struct Quote {
    ja: &'static str,
    en: &'static str,
}

const QUOTES: [Quote; 3] = [
    Quote {
        ja: "たった10年一緒に旅しただけだし…",
        en: "We only traveled together for a mere ten years...",
    },
    Quote {
        ja: "そうだね、ヒンメルならそう言う。",
        en: "Yes, Himmel would say so.",
    },
    Quote {
        ja: "お前の前にいるのは、千年以上生きた魔法使いだ。",
        en: "You are in front of a mage who has lived for more than one thousand years.",
    },
];

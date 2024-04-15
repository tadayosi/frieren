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

const QUOTES: [Quote; 28] = [
    Quote {
        ja: "たった10年一緒に旅しただけだし…",
        en: "We only traveled together for a mere ten years...",
    },
    Quote {
        ja: "…人間の寿命は短いってわかっていたのに……なんでもっと知ろうと思わなかったんだろう…",
        en: "...I knew that human life was short, but why didn't I try to learn more about him...",
    },
    Quote {
        ja: "私はもっと人間を知ろうと思う。",
        en: "I would try to know more about humans.",
    },
    Quote {
        ja: "勇者ヒンメルの死から20年後。",
        en: "Twenty years after the death of Himmel the Hero.",
    },
    Quote {
        ja: "とてもよいことでございますね。",
        en: "That'd be a very good thing, ma'am.",
    },
    Quote {
        ja: "フリーレン、あなたはやはり優しい子です。",
        en: "Frieren, you are indeed a tender girl.",
    },
    Quote {
        ja: "勇者ヒンメルの死から26年後。",
        en: "Twenty six years after the death of Himmel the Hero.",
    },
    Quote {
        ja: "勇者ヒンメルの死から27年後。",
        en: "Twenty seven years after the death of Himmel the Hero.",
    },
    Quote {
        ja: "メルクーアプリンですよね。",
        en: "Merkur pudding, right?",
    },
    Quote {
        ja: "あなたが私を知ろうとしてくれたことが、堪らなく嬉しいのです。",
        en: "I am unbearably happy that you've been trying to get to know me.",
    },
    Quote {
        ja: "あれが人を殺す魔法（ゾルトラーク）だよ。",
        en: "That is Zoltraak (the human killing magic).",
    },
    Quote {
        ja: "お前、私のスカートを捲ったクソガキだな。",
        en: "You are that damn kid who rolled up my skirt.",
    },
    Quote {
        ja: "勇者ヒンメルの死から28年後。",
        en: "Twenty eight years after the death of Himmel the Hero.",
    },
    Quote {
        ja: "フランメの著書に本物無し。",
        en: "There are no real Flamme's books.",
    },
    Quote {
        ja: "天国はある。そのほうが都合がいいだろう。",
        en: "There is a heaven. It would be more convenient that way.",
    },
    Quote {
        ja: "そうだね、ヒンメルならそう言う。",
        en: "Yes, Himmel would say so.",
    },
    Quote {
        ja: "言っておくけど私強いよ。断頭台のアウラよりも。",
        en: "I'm telling you, I am strong. Stronger than Aura the Guillotine.",
    },
    Quote {
        ja: "葬送のフリーレン。",
        en: "Frieren the Slayer.",
    },
    Quote {
        ja: "ヒンメルはもういないじゃない。",
        en: "Himmel is no longer there.",
    },
    Quote {
        ja: "戦士ってのは最後まで立っていた奴が勝つんだ。",
        en: "The warrior who stands until the end wins.",
    },
    Quote {
        ja: "一生だ。お前は一生を掛けて魔族を欺くんだ。",
        en: "A lifetime. You will deceive the demons through your whole life.",
    },
    Quote {
        ja: "フリーレン、私の一番好きな魔法は、綺麗な花畑を出す魔法だ。",
        en: "Frieren, my most favorite spell is the one that yields a beautiful field of flowers.",
    },
    Quote {
        ja: "…ふざけるな。私は五百年以上生きた大魔族だ。",
        en: "...Don't be silly. I am a great demon who has lived for more than five hundred years.",
    },
    Quote {
        ja: "お前の前にいるのは、千年以上生きた魔法使いだ。",
        en: "You are in front of a mage who has lived for more than one thousand years.",
    },
    Quote {
        ja: "俺達はエルフってことだ。",
        en: "We are elves, you know.",
    },
    Quote {
        ja: "勇者ヒンメルの死から29年後。",
        en: "Twenty nine years after the death of Himmel the Hero.",
    },
    Quote {
        ja: "…そうだね。ヒンメルはこの剣を抜けなかったんだ。",
        en: "...That's right. Himmel couldn't draw this sword out.",
    },
    Quote {
        ja: "私は今の話をしている。",
        en: "I am talking about now.",
    },
];

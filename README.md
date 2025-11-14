# notado-cli

A CLI to search through your notes and backup your [Notado](https://notado.app) library

## Installation

This tool can be compiled using `cargo install`:

```
cargo install --git https://github.com/LGUG2Z/notado-cli
```

## Usage

### Credentials

In order to retrieve information from your personal account, make sure that you have set the `NOTADO_API_TOKEN`
environment variable to the API token shown on [https://notado.app/settings](https://notado.app/settings) before using
this tool.

### Backup

The `backup` command prints all notes in your account to stdout. You can redirect this output to a file and use
scheduled tasks to automatically export snapshots of your library.

```bash
notado-cli backup >"notado-backup-$(date -u +'%Y-%m-%d-%H').json"
```

### Search

The `search` command prints same results as the fuzzy search bar on
your [main feed page](https://notado.app) to stdout.

```jsonc
// ❯ notado-cli search "empathy"

[
  {
    "content": "@ellie @mo8it At certain scales, it's not possible for one person to even read everything. I have a team of volunteers and a helpful community and things still get dropped.\n
\nWriting any of these comments creates noise, frustration and distracts from things that might be critical (eg. security issues or spambots on the chat server).\n\nI totally empathize with people wanting their issue fixed or PR in, but as @ellie says - let us manage our priorities. If it drops off our radar - that's how it is.",
    "url": "https://hachyderm.io/@imsnif/112337649471974955",
    "title": "Aram 🌈♾️ (@imsnif@hachyderm.io)",
    "created": "2024-04-26T13:17:08.450321Z",
    "tags": []
  },
  {
    "content": "I am very worried about Arab and Muslim students. They are watching a genocide unfold while their countries either cheer it on or do nothing. While most professors are lovely and supportive, many lack understanding and empathy. If I'm struggling I know they are. It hurts.",
    "url": "https://twitter.com/SEltantawi/status/1724202946647224405",
    "title": "Sarah Eltantawi (@SEltantawi)",
    "created": "2023-11-14T01:25:51.240893Z",
    "tags": [
      "academia",
      "islam",
      "usa"
    ]
  },
  {
    "content": "The Forks episode of The Bear is one of my favorite stories on giving a shit: a fork polisher at a fancy restaurant learns you don’t work in food service because you love pol
ishing forks. You do it because you want to bring people joy and polishing the forks is one of many steps to that end. You can find purpose in fork polishing through both excellence and empathy for your customers. There’s a lot of days the code I write is about as exciting as fork polishing, but you do it for your teammates and users.",
    "url": "https://news.ycombinator.com/item?id=38059357",
    "title": "Giving a Shit as a Service (2022)",
    "created": "2023-10-29T20:43:24.918225Z",
    "tags": [
      "development",
      "learning",
      "software"
    ]
  },
  {
    "content": "In order to be kind, we have to shut down that animal instinct and force our brain to travel a different pathway. Empathy and compassion are evolved states of being. They req
uire the mental capacity to step past our most primal urges. I’m here to tell you that when someone’s path through this world is marked with acts of cruelty, they have failed the first test 
of an advanced society. They never forced their animal brain to evolve past its first instinct. They never forged new mental pathways to overcome their own instinctual fears. And so, their thinking and problem-solving will lack the imagination and creativity that the kindest people have in spades.",
    "url": "https://daringfireball.net/linked/2023/07/27/pritzker-kindness-intelligence",
    "title": "Daring Fireball: Kindness as a Signifier of Intelligence",
    "created": "2023-07-28T14:59:34.311966Z",
    "tags": [
      "kindness",
      "mental-health"
    ]
  },
  {
    "content": "Unless you're working completely alone, it's not just your ability to solve technical problems, to write good code, etc, that matters. To the contrary, they matter even less 
if you make the people around you unhappy and less productive. Just like learning to write good code, you have to learn \"to people\" good as well. Empathy is a big part of this, as is recognising that people are different – be caring, be understanding, help others and ask for help yourself, be nice. Be an engineer others want to work with.",
    "url": "https://martinrue.com/my-engineering-axioms/",
    "title": "My Engineering Axioms",
    "created": "2020-12-22T00:57:07.685875Z",
    "tags": [
      "development",
      "software"
    ]
  },
  {
    "content": "Empathy is not automatic for everyone. My emotions are so unreliable and unpredictable that they are useless for understanding other people.\n\nAll of my social skills are ba
sed on logical analysis and a lot of trial and error.\n\nI enter every conversation with a goal of what I wish to communicate and how I want the other person to feel. I then attempt to do th
e proper things to make that happen.\n\nSociety tells us that this way of handling interpersonal interaction is manipulative and evil. For many people, this is the only way we can function in a society that doesn’t think the way we do.",
    "url": "https://news.ycombinator.com/item?id=25092445",
    "title": "Empathy and perspective taking: How social skills are built | Max Planck Institute for Human Cognitive and Brain Sciences",
    "created": "2020-11-14T15:29:22.347890Z",
    "tags": [
      "emotion",
      "empathy",
      "friendship",
      "mental-health",
      "society"
    ]
  },
  {
    "content": "This article is a long-winded philosophical musing that misses the core point:\n\nYou can only not care about politics if you are incredibly priviliged. Period.\n\nIf one of 
the main parties running wants to prevent you from marrying the person you love, you can't just be apolitical. If one of the parties running wants to allow police to disproportionally harass
, arrest, and kill people who look like you, you can't just be apolitical. If you're poor, struggling to pay your bills, and one of the parties wants to add new hoops to jump through to clai
m any welfare or healthcare, you can't just be apolitical.\n\nIf you have any empathy and care about any people in those positions, you can't just be apolitical.\n\nThe only way to truly \"not care\" about politics is both to be very privileged and very selfish.",
    "url": "https://news.ycombinator.com/item?id=25028180",
    "title": "It’s OK to Not Care About Politics – Meta-Nomad",
    "created": "2020-11-09T02:10:59.383056Z",
    "tags": [
      "politics",
      "privilege",
      "race"
    ]
  }
]
```

### QuoteGen

The `quote-gen` takes an input file (`sources` by default) of newline-separated
URLs from supported commentary sites and attempts to populate a `library.json`
file with the data needed to render those notes
via [shortcodes](https://lgug2z.com/articles/im-done-writing-shortcodes-for-other-websites/)
to embed them into static site generator pages.

If an entry for a source already exists in `library.json`, it will be skipped.

If a URL in the source file does not correspond to a note in your account, it
will be skipped.

```text
# sources

https://lobste.rs/s/rzskjk/i_think_i_m_done_thinking_about_genai_for#c_l9x7we
https://tildes.net/~tech/17xe/permanent_archival_formats_do_they_exist#comment-9feo
https://old.reddit.com/r/patientgamers/comments/udzo11/i_miss_the_days_of_server_browsers_and_community/i6lga1o/
https://programming.dev/comment/5789966
https://discourse.nixos.org/t/should-organizations-relating-to-the-defense-sector-being-able-to-sponsor-nixos/41252/6
https://defcon.social/@corbden/113473397794111625
https://bsky.app/profile/hmsnofun.bsky.social/post/3lmr6sm5k4k2b
https://twitter.com/mitchellh/status/1744850961309597855?s=12
https://www.youtube.com/watch?v=k0J0Dxf5JKc&lc=Ugwsmg7c0JpYiXyADQV4AaABAg
```

```text
❯ notado-cli quote-gen
Adding quote data for https://bsky.app/profile/hmsnofun.bsky.social/post/3lmr6sm5k4k2b to library.json
Adding quote data for https://defcon.social/@corbden/113473397794111625 to library.json
Adding quote data for https://discourse.nixos.org/t/should-organizations-relating-to-the-defense-sector-being-able-to-sponsor-nixos/41252/6 to library.json
Adding quote data for https://lobste.rs/s/rzskjk/i_think_i_m_done_thinking_about_genai_for#c_l9x7we to library.json
Adding quote data for https://old.reddit.com/r/patientgamers/comments/udzo11/i_miss_the_days_of_server_browsers_and_community/i6lga1o/ to library.json
Adding quote data for https://programming.dev/comment/5789966 to library.json
Adding quote data for https://tildes.net/~tech/17xe/permanent_archival_formats_do_they_exist#comment-9feo to library.json
Adding quote data for https://twitter.com/mitchellh/status/1744850961309597855?s=12 to library.json
Adding quote data for https://www.youtube.com/watch?v=k0J0Dxf5JKc&lc=Ugwsmg7c0JpYiXyADQV4AaABAg to library.json
```

```jsonc
// library.json

{
  "https://bsky.app/profile/hmsnofun.bsky.social/post/3lmr6sm5k4k2b": {
    "title": "hmsnofun.bsky.social",
    "source_display": "bsky.app",
    "source_url": "https://bsky.app/profile/hmsnofun.bsky.social/post/3lmr6sm5k4k2b",
    "content": "the labor of art is mostly invisible. what you don't see in a finished piece is all the hours of thought it required, all the little discoveries and surprises and tragedies of production. art IS the process. if you aren't  interested in the process, then i don't really know why you want to be artist"
  },
  "https://defcon.social/@corbden/113473397794111625": {
    "title": "corbden",
    "source_display": "defcon.social",
    "source_url": "https://defcon.social/@corbden/113473397794111625",
    "content": "Work isn't hard when you've got the capacity and capability to do what is needed, and you're in a good mood. Work is fulfilling when it is purposeful and not frustrating.\r\n\r\nThe conditions under which many of us were taught to work were not ideal, and only prepared us to tolerate without complaint the conditions under which we were expected to perform in our adult lives, conditions usually bad ONLY because the powerful need to abuse, and not for any necessary reason. \r\n\r\nThat's why nobody likes work. We're wired to like work, but we've been made to dislike it. \r\n\r\nBecause the cruelty is the point. Exploitation is the point. Dominance is the point."
  },
  "https://discourse.nixos.org/t/should-organizations-relating-to-the-defense-sector-being-able-to-sponsor-nixos/41252/6": {
    "title": "jakehamilton",
    "source_display": "discourse.nixos.org",
    "source_url": "https://discourse.nixos.org/t/should-organizations-relating-to-the-defense-sector-being-able-to-sponsor-nixos/41252/6",
    "content": "I do not want any of my work associated with arms dealers and/or the deaths of others. Having any sponsorship from a military entity steps over that line. I would much prefer that the NixOS Foundation set ethical guidelines for accepting sponsorship. I do not believe it is absurd to suggest that we shouldn’t take money from people who are responsible for the slaughter of others. Rejecting sponsorship from weapons manufacturers should be a reasonable standard.\n\nI would like to be clear here: this is a significant problem for myself and many others and will result in community fracture if it is not resolved. I’m sure we would all prefer to not split into two groups of “people okay with killing others” and “people who think killing others is bad”."
  },
  "https://lobste.rs/s/rzskjk/i_think_i_m_done_thinking_about_genai_for#c_l9x7we": {
    "title": "mattgreenrocks",
    "source_display": "lobste.rs",
    "source_url": "https://lobste.rs/s/rzskjk/i_think_i_m_done_thinking_about_genai_for#c_l9x7we",
    "content": "Yep. I arrived at a similar conclusion.\n\nLife is hard, but doing The Work (in whatever subject(s) that is for you) is one way to find solace and unmediated joy. It’s not just any work (hence the capitalization), but work that is good for me to do. I can apply myself to a challenge, work with other people to build things, and learn more about it in an endless cycle. It’s an infinite game. Anything that gets in the way of that cycle is harmful *for me:* more management than dev, politicking for better titles, even screwing around with my dev environment for too long. It’s about the work of shipping, refining, and learning. When that gets out of balance I become highly vulnerable to anxiety.\n\nWhere do LLMs fit there? Certainly not in the core loop! Part of The Work necessitates some time spent frustrated: that’s a primary indication of learning (and very easy to forget). I’m okay with them at the periphery of it, or sometimes spitballing things with them.\n\nBut there’s a real joy to applying your skills, working through frustration and feeling stuck, and gaining confidence that I would never want to outsource to an LLM. It’s essential for human flourishing."
  },
  "https://old.reddit.com/r/patientgamers/comments/udzo11/i_miss_the_days_of_server_browsers_and_community/i6lga1o/": {
    "title": "szthesquid",
    "source_display": "old.reddit.com",
    "source_url": "https://old.reddit.com/r/patientgamers/comments/udzo11/i_miss_the_days_of_server_browsers_and_community/i6lga1o/",
    "content": "This is one of the big reasons my whole Team Fortress 2 group can't go back to the game. We played *thousands* of hours together in its early years. At the time, some of us had 100% achievement completion.\n\nWe played on our local Toronto servers and got to know all the regulars. It's *such* a different game when you play with a community who recognizes player and clan names by sight and develop respect and rivalries.\n\nThe time when we skilled up enough to contend with the server's top Sniper.\n\nThe time when the server's best medic attached himself to me for the whole game because he recognized me as the best offensive player on the team.\n\nThe times when we'd all hop on the same team and other players would switch teams specifically to play with or against us. \n\nThe times when all talk was enabled and we could hear the other team get demoralized and panicked when they realized that our buddy was playing Spy this game.\n\nChecking the server leaderboards after the session and realizing our whole group had broken into the top 100.\n\nThe feeling that if we went out for a walk and ran into another Team Fortress player on the street, we might actually recognize their user name. \n\nQuick play with randos will never be the same."
  },
  "https://programming.dev/comment/5789966": {
    "title": "bugsmith",
    "source_display": "programming.dev",
    "source_url": "https://programming.dev/comment/5789966",
    "content": "I particularly enjoyed a recent company meeting that spent considerable time talking about the importance of flow state. It had an awkward pregnant pause when someone (usually very quiet) unmuted to ask, \"is the policy to increase the number of days we must spend in our open-plan office kind of undermining this?\". Literally all of our directors just shifted on their seats hoping another would answer that.\n\nEventually, HR director stated \"Not at all, that's what headphones are for!\"\n\nWhich was particularly delightful, as our tech director had only 20 minutes before stated how he would like to discourage people sitting in the office in silos with their headphones on."
  },
  "https://tildes.net/~tech/17xe/permanent_archival_formats_do_they_exist#comment-9feo": {
    "title": "xk3",
    "source_display": "tildes.net",
    "source_url": "https://tildes.net/~tech/17xe/permanent_archival_formats_do_they_exist#comment-9feo",
    "content": "All information *eventually* gives way to entropy.\n\nThe only real solution is constant refresh, validation, and multiple copies of data. The only reason we know about many historical things is due to multiple copies of organisms existing and we are able to find the lucky ones that were preserved in tree sap.\n\nLikewise, we are lucky to find old stories because they were passed down through constant rehearsal in oral tradition and/or because some crazy guy brought some scrolls up to a mountain. There are probably many scrolls that we don't know about because they weathered away. We just find the lucky ones. It's chance.\n\nAnalog may last longer than digital data but I wouldn't trust just one type of storage. If you care about something you need to have multiple copies and validate that the data is still readable every year or at the very least 5-10 years. If (VHS) players are no longer manufactured you also need to consider that aspect as well.\n\nEven stone tablets have bitrot at a timescale of months if the conditions are right"
  },
  "https://twitter.com/mitchellh/status/1744850961309597855?s=12": {
    "title": "mitchellh",
    "source_display": "twitter.com",
    "source_url": "https://twitter.com/mitchellh/status/1744850961309597855?s=12",
    "content": "For open source in particular, the misbalance that always made me sad was when an issue reporter spends 30 seconds writing an issue that's going to take a maintainer (working for free) hours, days, weeks to resolve and maintain, then gets mad when its not fixed quickly. 🤔"
  },
  "https://www.youtube.com/watch?v=k0J0Dxf5JKc&lc=Ugwsmg7c0JpYiXyADQV4AaABAg": {
    "title": "QuantaStarfire",
    "source_display": "www.youtube.com",
    "source_url": "https://www.youtube.com/watch?v=k0J0Dxf5JKc&lc=Ugwsmg7c0JpYiXyADQV4AaABAg",
    "content": "I feel like the decline of the arena shooter can also be attributed to the lack of dedicated servers and the lack of editing tools, because both of these things together are what have allowed games like DOOM and Quake to continue to exist for almost three decades now.\r\n\r\nDedicated servers allow people to form small communities of regular players, while editors allow people to create new modes and levels for the game to keep things fresh.\r\n\r\nPublishers don't like either of these things since a game you buy once and can play forever with nigh-infinite content doesn't generate the nigh-infinite revenue they want, so of course those features had to either be taken out back and shot, or at least be put under very heavy developer control."
  }
}
```
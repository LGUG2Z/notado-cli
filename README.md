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
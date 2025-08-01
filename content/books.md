---
title: "Bits from Books"
layout: "base.njk"
description: "Thought-provoking excerpts from books I've read"
permalink: "/books/"
---

# Bits from Books

```bash
$ find ~/library -name "*.epub" -o -name "*.pdf" | wc -l
267

$ grep -r "highlighted" ~/.book_notes | wc -l
1,247

$ echo "Current reading status: Always learning"
Current reading status: Always learning
```

A curated collection of thought-provoking excerpts, insights, and memorable passages from books that have shaped my thinking and understanding.

---

## Jordan Peterson - Maps of Meaning

**Published**: 1999  
**Pages**: 564  
**Genre**: Psychology, Philosophy

```bash
$ cat ~/notes/maps_of_meaning.txt | grep -A2 -B2 "urge to stab"
```

> "At some point during the lecture, I would unfailingly feel the urge to stab the point of my pen into the neck of the person in front of me."
>
> *Page 16*

This raw honesty about intrusive thoughts reveals Peterson's willingness to examine the darkest corners of human psychology. It's a reminder that even the most composed individuals wrestle with unwanted mental phenomena.

---

> "All the things I "believed" were things I thought sounded good, admirable, respectable, courageous. They weren't my things, however—I had stolen them. Most of them I had taken from books. Having "understood" them, abstractly, I presumed I had a right to them—presumed that I could adopt them, as if they were mine: presumed that they were me."
>
> *Page 17*

A powerful reflection on the difference between intellectual understanding and embodied wisdom. True beliefs must be lived, not just comprehended.

**Key Insights**:
- The danger of adopting beliefs without genuine understanding
- The importance of lived experience over abstract knowledge
- The psychological work required to make ideas truly your own

---

## Sir Michael Atiyah - Mathematics in the 20th Century

**Published**: 2002  
**Pages**: 15 (Bulletin paper)  
**Genre**: Mathematics, Philosophy of Mathematics

```bash
$ grep -i "devil" ~/papers/atiyah_mathematics_20th_century.pdf
```

> "Algebra is the offer made by the devil to the mathematician. The devil says: I will give you this powerful machine, it will answer any question you like. All you need to do is give me your soul: give up geometry and you will have this marvelous machine."
>
> *Page 7, Bulletin of the London Mathematical Society, 34 (2002) 1–15*

Atiyah captures the eternal tension in mathematics between algebraic abstraction and geometric intuition. The "Faustian bargain" metaphor perfectly encapsulates how powerful tools can distance us from fundamental understanding.

**Reflection**: This resonates deeply with embedded systems development. We have powerful abstractions and frameworks, but understanding the underlying hardware geometry remains crucial for optimal solutions.

---

## Clair Obscur: Expedition 33

**Released**: 2025  
**Genre**: Video Game Narrative  
**Platform**: RPG

```bash
$ grep -A3 "Verso:" ~/game_notes/clair_obscur.txt
```

> "Verso: Your father made the Axons, right?  
> Verso: They are the essence of your mother. And... your brother.  
> Monaco: She Who Plays with Wonder. He Who Guards Truth With Lies."
>
> *Act 3*

Even in interactive media, profound philosophical questions emerge about identity, creation, and the relationship between technology and humanity. The poetic language hints at deeper truths about the nature of artificial intelligence and consciousness.

**Technical Note**: The naming convention "Axons" is particularly interesting from a neuroscience perspective—axons are the projections from neurons that carry electrical impulses. The metaphor connects biological neural networks with artificial ones.

---

## Reading Statistics

```bash
$ cat ~/.reading_stats.json | jq '.'
{
  "books_completed_2024": 23,
  "pages_read_total": 8934,
  "average_pages_per_book": 389,
  "favorite_genres": [
    "Psychology",
    "Philosophy", 
    "Computer Science",
    "Mathematics",
    "Neuroscience"
  ],
  "notes_taken": 1247,
  "quotes_highlighted": 892
}
```

## Currently Reading

```bash
$ cat ~/current_reads.txt
```

**Primary**: *The Phenomenology of Perception* by Maurice Merleau-Ponty  
**Secondary**: *Operating Systems: Three Easy Pieces* by Remzi Arpaci-Dusseau  
**Technical**: *ARM System Developer's Guide* by Andrew Sloss

## Reading Philosophy

```bash
$ cat ~/.profile/reading_philosophy
```

My approach to reading has evolved from consumption to digestion. Rather than racing through books, I focus on:

1. **Active Engagement**: Taking notes, questioning assumptions, connecting ideas
2. **Cross-Pollination**: Finding connections between disparate fields
3. **Implementation**: Testing ideas in real-world contexts
4. **Reflection**: Regular review of highlights and notes

The most valuable books are those that change how you think, not just what you know.

## Book Recommendations by Category

### Technical Depth
```bash
$ ls ~/library/technical/ | head -5
- Operating Systems: Three Easy Pieces
- Computer Systems: A Programmer's Perspective  
- The Linux Programming Interface
- ARM System Developer's Guide
- Understanding the Linux Kernel
```

### Psychological Insight
```bash  
$ ls ~/library/psychology/ | head -5
- Maps of Meaning - Jordan Peterson
- Thinking, Fast and Slow - Daniel Kahneman
- The Righteous Mind - Jonathan Haidt
- Flow - Mihaly Csikszentmihalyi
- Man's Search for Meaning - Viktor Frankl
```

### Mathematical Beauty
```bash
$ ls ~/library/mathematics/ | head -5
- Mathematics in the 20th Century - Michael Atiyah
- A Mathematician's Apology - G.H. Hardy
- Gödel, Escher, Bach - Douglas Hofstadter
- The Art of Computer Programming - Donald Knuth
- Category Theory for Programmers - Bartosz Milewski
```

## Note-Taking System

```bash
$ tree ~/.book_notes/
.book_notes/
├── by_author/
│   ├── peterson_jordan/
│   │   ├── maps_of_meaning.md
│   │   └── 12_rules_for_life.md
│   └── atiyah_michael/
│       └── mathematics_20th_century.md
├── by_topic/
│   ├── consciousness.md
│   ├── mathematics.md
│   └── psychology.md
└── connections/
    ├── cross_references.md
    └── synthesis.md
```

Each book gets three types of notes:
- **Direct Quotes**: Exact passages with page numbers
- **Paraphrases**: Key ideas in my own words  
- **Connections**: Links to other books, experiences, or projects

## Impact on Engineering Work

The most unexpected benefit of diverse reading has been its impact on my technical work:

- **Psychology** helps understand user needs and team dynamics
- **Philosophy** provides frameworks for architectural decisions
- **Mathematics** offers elegant solutions to complex problems
- **Literature** improves documentation and communication skills

```bash
$ echo "The best code is written by people who read more than code"
The best code is written by people who read more than code
```

## Future Reading List

```bash
$ cat ~/reading_queue.txt | head -10
[ ] Consciousness Explained - Daniel Dennett
[ ] The Structure of Scientific Revolutions - Thomas Kuhn  
[ ] Metamorphosis of Prime Numbers - Enrico Bombieri
[ ] The Design of Everyday Things - Don Norman
[ ] Antifragile - Nassim Nicholas Taleb
[ ] The Quantum Theory Cannot Hurt You - Marcus Chown
[ ] Code: The Hidden Language - Charles Petzold
[ ] The Elegant Universe - Brian Greene
[ ] Mindset - Carol Dweck
[ ] The Pragmatic Programmer - Andy Hunt
```

---

*"A reader lives a thousand lives before he dies. The man who never reads lives only one."* — George R.R. Martin

```bash  
$ uptime
Reading habit active for 15+ years, currently processing 2-3 books simultaneously
```
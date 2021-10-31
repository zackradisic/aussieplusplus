![aussie_plus_plus](assets/code.png)
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-2-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
# aussie++

Programming language from down under, inspired by [this](https://www.reddit.com/r/ProgrammerHumor/comments/oa8chw/australian_programming_language/) Reddit post.

View live demo [here](http://aussieplusplus.vercel.app/).

Special thanks to [MarkWhyBird](https://github.com/MarkWhybird), [louis100](https://github.com/louis1001), and others who came up with the language [spec](https://github.com/louis1001/c---/issues/5).

## Key Features
* 🇦🇺 Syntax entirely comprised of Australian lingo and slang
* 🪃 Wield an Australian's greatest weapon: use boomerangs (angle-brackets) instead of curly braces
* **True aussie mode**, where uʍop ǝpᴉsdn characters become valid code

## Example
```
G'DAY MATE!

THE HARD YAKKA FOR fibonacci IS ( x ) <
    YA RECKON x <= 1 ? BAIL x;

    BAIL fibonacci(x - 1) + fibonacci(x - 2);
>

GIMME fibonacci(30);
```


# Docs
`aussie++` is a dynamically-typed and interpreted language inspired by [this](https://www.reddit.com/r/ProgrammerHumor/comments/oa8chw/australian_programming_language/) Reddit post. 

## General
All keywords are case-insensitive,
meaning `CHEERS C***!` is equivalent to `cheers c***!`, but all caps is strongly recommended.

We use boomerangs (`<` `>`) instead of curly braces (`{` `}`)

```aussie
// Programs must start with `G'DAY MATE!`
G'DAY MATE!

// Prints "crikey mate!" to console
GIMME "crikey mate!";

// Boomerangs for blocks/scopes
<
	I RECKON x = 5;
>

// Use this to indicate end of program
CHEERS C***!
```


## Types / Variables
Declare booleans, numbers, strings and `nil/null` like so:
```aussie
// Booleans
I RECKON thisIsFalse = YEAH, NAH;
I RECKON thisIsTrue = NAH, YEAH;

// Numbers
I RECKON regularInteger = 42069;
I RECKON tinyNum = 0.00001;
I RECKON negativeNum = -1;

// Strings
I RECKON goodStr = "fair dinkum mate!";

// Nil/Null
I RECKON emptiness = BUGGER ALL;
```

## Control flow
`aussie++` supports if statements and basic pattern matching:
```aussie
// If/else statemets
YA RECKON 1 == 2 ? <
	GIMME "fark we broke maths!";
> WHATABOUT NAH, YEAH == YEAH, NAH ? <
	GIMME "strewth we broke boolean logic!";
> WHATABOUT ? <
	GIMME "the universe is okay";
>

// Pattern matching
YA RECKON randomBeer() IS A <
	"Fosters"    ~ GIMME "Flamin' hell!";
	"Coopers"    ~ GIMME "You Beauty!";
	somethinElse ~ GIMME "Yeah, dunno that one: " + somethinElse;
>
```

## Loops
`aussie++` has for and while loops. With for loops the main thing to note is that the ranges are specified using interval notation (`[` or `]` is inclusive, and `(` or `)` is exclusive). You can mix and match. You can break out of a loop by saying `MATE FUCK THIS`:
```aussie
// From 0-100
I RECKON x IS A WALKABOUT FROM [0 TO 100] <
	GIMME x;
>

// From 0-99
I RECKON x IS A WALKABOUT FROM [0 TO 100) <
	GIMME x;
>

// Breaking with `MATE FUCK THIS`
I RECKON x IS A WALKABOUT FROM [0 TO 999999] <
	YA RECKON x > 1000 ? MATE FUCK THIS;
>
```

While loops are similar to those you would find in other languages, except that the loop only executes if the condition is false.

```aussie
// OI MATE, PAY ATTENTION! THIS LOOP STOPS WHEN I'VE WALKED OVER 3 KM!

I RECKON kmWalked = 0;
I RECKON I'LL HAVE A WALKABOUT UNTIL (kmWalked > 3) <
	GIMME "i walked 1 km!";
	kmWalked = kmWalked + 1;
>
GIMME "BLOODY OATH I'M TIRED!";
```

## Functions
Define functions like so, using `BAIL <somethin>` to return values:
```aussie
THE HARD YAKKA FOR greeting() IS <
	BAIL "G'day mate!";
>

GIMME greeting();
```

## Standard library / Built-ins
Use `IMPOHT ME FUNC <func>` to import built-in functions. The language currently comes with two built-ins, `ChuckSomeDice(start, end)` and `HitTheSack(ms)`:

```aussie
IMPOHT ME FUNC ChuckSomeDice;
IMPOHT ME FUNC HitTheSack;

THE HARD YAKKA FOR goIntoAComa() IS <
	// Return a random integer from 0-99
	I RECKON duration = ChuckSomeDice(0, 100);

	// Sleep for `duration` seconds
	HitTheSack(duration * 1000);

	GIMME "strewth! i went into a coma!";
>

goIntoAComa();
```

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/jwfxpr"><img src="https://avatars.githubusercontent.com/u/20788820?v=4?s=100" width="100px;" alt=""/><br /><sub><b>jwfxpr</b></sub></a><br /><a href="https://github.com/zackradisic/aussieplusplus/commits?author=jwfxpr" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/bbrk24"><img src="https://avatars.githubusercontent.com/u/25109429?v=4?s=100" width="100px;" alt=""/><br /><sub><b>bbrk24</b></sub></a><br /><a href="#ideas-bbrk24" title="Ideas, Planning, & Feedback">🤔</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
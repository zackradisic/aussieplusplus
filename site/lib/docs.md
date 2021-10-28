# Docs
`aussie++` is a dynamically-typed and interpreted language inspired by [this](https://www.reddit.com/r/ProgrammerHumor/comments/oa8chw/australian_programming_language/) Reddit post. 

## General
All keywords are case-insensitive,
meaning `CHEERS C***!` is equivalent to `cheers c***!`, but all caps is strongly recommended.

We use boomerangs (`<` `>`) instead of curly braces (`{` `}`)

```aussie
// Programs must start with `G'DAY MATE!`
G'DAY MATE!

// Prints "blimey mate!" to console
GIMME "blimey mate!";

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
	GIMME "fark we broke math!";
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
I RECKON x IS A WALKABOUT FROM [0, 100] <
	GIMME x;
>

// From 0-99
I RECKON x IS A WALKABOUT FROM [0, 100) <
	GIMME x;
>

// Breaking with `MATE FUCK THIS`
I RECKON x IS A WALKABOUT FROM [0, 999999] <
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

GIMME gdayMate();
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

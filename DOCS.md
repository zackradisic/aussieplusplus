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
Booleans are any sequence of `NAH`s and `YEAH`s separated by whitespace, a comma, or `\n` and followed by a terminal `!` denoting the end of the boolean. The last `NAH` or `YEAH` determines the truthiness of the boolean. The following are all valid booleans: 
```aussie
// Booleans
I RECKON thisIsFalse = YEAH, NAH!;
I RECKON thisIsTrue = NAH, YEAH!;
I RECKON alsoTrue = NAH YEAH YEAH YEAH YEAH YEAH YEAH!
I RECKON wow = NAH YEAH NAH 
NAH YEAH NAH NAH YEAH NAH NAH YEAH NAH!
```

Numbers, strings and `nil/null` are like other languages:
```aussie
// Numbers
I RECKON regularInteger = 42069;
I RECKON tinyNum = 0.00001;
I RECKON negativeNum = -1;

// Strings
I RECKON goodStr = "fair dinkum mate!";

// Nil/Null
I RECKON emptiness = BUGGER ALL;
```
## Operators

Most mathematical operators are familiar from other languages.
```aussie
I RECKON a = 1;
I RECKON b = 2;
I RECKON sum = a + b;
I RECKON diff = b - a;
I RECKON product = a * b;
I RECKON ratio = a / b;
```

However, the increment and decrement expressions are unique. To increment a variable `var`, use `GOOD ON YA var`; to decrement, use `PULL YA HEAD IN var`. These expressions are pre-increment and pre-decrement expressions: the value attached to the variable is modified, then returned.
```aussie
I RECKON a = 0;
GOOD ON YA a; // a == 1
I RECKON b = 10 + GOOD ON YA a; // a == 2, b == 12
I RECKON c = PULL YA HEAD IN b + GOOD ON YA a; // a == 3, b == 11, c == 14
PULL YA HEAD IN c; // c == 13
```

## Control flow
`aussie++` supports if statements and basic pattern matching:
```aussie
// If/else statemets
YA RECKON 1 == 2 ? <
	GIMME "fark we broke maths!";
> WHATABOUT NAH, YEAH! == YEAH, NAH! ? <
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

## Comments
All lines before `G'DAY MATE!` and after `CHEERS C***!` are ignored, and can be used to document your module.

`//` marks the start of documentation until the end of that line.

Block comments can be opened with `OI MATE!` and closed with `GOT IT?`.

```aussie
You bloody bewdy, cobbadiggamate, this is a rippa module!

G'DAY MATE!

THE HARD YAKKA FOR YOU_CAN_GET_IT_DOING(NOTHING_AT_ALL) IS <
	I RECKON A_HARD_EARNED_THIRST = "A BIG COLD BEER"; // And the best cold beer is Vic, Victoria Bitter.
OI MATE!
But I drink to get p*ssed!
GOT IT?
        BAIL A_HARD_EARNED_THIRST;
>

CHEERS C***!

References:
- https://www.youtube.com/watch?v=0uPCi_KnCiQ
- https://www.youtube.com/watch?v=7n9IE2jRtgs
```

/* eslint-disable camelcase */
const time = `G'DAY MATE!

IMPOHT ME FUNC GimmeTime;

GIMME "the time in sydney is: " + GimmeTime();
`

const fibonacci = `G'DAY MATE!

THE HARD YAKKA FOR fibonacci IS ( x ) <
    YA RECKON x <= 1 ? BAIL x;

	BAIL fibonacci(x - 1) + fibonacci(x - 2);
>

GIMME fibonacci(10);

CHEERS C***!`

const dreamtime = `G'DAY MATE!

IMPOHT ME FUNC HitTheSack;
IMPOHT ME FUNC ChuckSomeDice;

THE HARD YAKKA FOR dreamtime IS () <
	GIMME "'boutta get some winks mate";

	I RECKON I'LL HAVE A WALKABOUT UNTIL (YEAH, NAH) <
	    GIMME "zZz...";
		
		HitTheSack(1000);

		YA RECKON ChuckSomeDice(0, 6) == 0 ? MATE FUCK THIS;
	>

	GIMME "that nap was bonza mate!";
>

dreamtime();

CHEERS C***!
`

const random_beer = `G'DAY MATE!

IMPOHT ME FUNC ChuckSomeDice;

THE HARD YAKKA FOR randomBeer IS () <
	YA RECKON ChuckSomeDice(0, 3) IS A <
		0 ~ BAIL "Fosters";
		1 ~ BAIL "Coopers";
		2 ~ BAIL "Pilsner";
	>
>

YA RECKON randomBeer() IS A <
	"Fosters"    ~ GIMME "Flamin' hell!";
	"Coopers"    ~ GIMME "You Beauty!";
	somethinElse ~ GIMME "Yeah, dunno that one: " + somethinElse;
>

CHEERS C***!`

export const examples: Record<string, string> = {
  fibonacci,
  dreamtime,
  time,
  random_beer
}

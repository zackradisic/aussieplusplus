# aussie++

Programming language from down under, inspired by [this](https://www.reddit.com/r/ProgrammerHumor/comments/oa8chw/australian_programming_language/) Reddit post.

## Key Features
* Boomerangs (angle-brackets) instead of curly braces
* Pattern matching
* `NAH, YEAH == true` and `YEAH, NAH == false`
* Declare variables with `I RECON x = 420`
* Other Aussie keywords, CAPITALIZATION is encouraged but not required

## Example
```javascript
BLIMEY MATE

I RECON x = 5;
I RECON y = 10;

YA RECON x == y - 5 <
	// Pattern matching
	NAH, YEAH? <
		bail "dinkum"
	>
	YEAH, NAH? <
		bail "fark"
	>
> 

// Declaring a function
HARD YAKKA FOR fibonacci ( x ) <
		// If statements
		YA RECON x == 0 <
				bail 0
		>
		YA RECON x == 1 <
				bail 1
		>

		bail fibonacci(x - 1) + fibonacci(x - 2)
>

WALKABOUT (x = 0; x < 5; x = x + 1) <

>

CHOOK BICKEY
```

## Down under mode
With down under mode enabled, upside down code becomes valid
syntax.

```javascript
⅄ƎʞƆIq ʞOOHƆ

<

> (Ɩ + x = x ;ϛ > x ;0 = x) ┴∩Oq∀ʞ˥∀M

<
(ᄅ - x)ᴉɔɔɐuoqᴉɟ + (Ɩ - x)ᴉɔɔɐuoqᴉɟ lᴉɐq		

<		
Ɩ lᴉɐq				
> Ɩ == x NOƆƎɹ ∀⅄		
<		
0 lᴉɐq				
> 0 == x NOƆƎɹ ∀⅄		
sʇuǝɯǝʇɐʇs ɟI //		
> ( x ) ᴉɔɔɐuoqᴉɟ ɹOℲ ∀ʞʞ∀⅄ pɹ∀H
uoᴉʇɔunɟ ɐ ƃuᴉɹɐlɔǝp //

 <
<	
,,ʞɹɐɟ,, lᴉɐq		
> ¿H∀N 'H∀Ǝ⅄	
<	
,,ɯnʞuᴉp,, lᴉɐq		
> ¿H∀Ǝ⅄ 'H∀N	
ƃuᴉɥɔʇɐɯ uɹǝʇʇɐԀ //	
> ϛ - ʎ == x NOƆƎɹ ∀⅄

;0Ɩ = ʎ NOƆƎɹ I
;ϛ = x NOƆƎɹ I

Ǝ┴∀W ⅄ƎWI˥q
```

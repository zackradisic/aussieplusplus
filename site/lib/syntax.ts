import Prism, { languages } from 'prismjs'

export const aussieSyntax = languages.extend('clike', {
  string: {
    pattern: /(["`])(?:\\[\s\S]|(?!\1)[^\\])*\1/,
    greedy: true
  },
  keyword:
    /\b(?:GIMME|G'DAY MATE!|CHEERS C\*\*\*!|I RECKON|YA RECKON|WHATABOUT|IS A|IS|WALKABOUT|FROM|MATE FUCK THIS|I'LL HAVE A|UNTIL|THE HARD YAKKA FOR|IMPOHT ME FUNC|BAIL)\b/,
  boolean: /\b(?:BUGGER ALL|NAH|YEAH)\b/,
  number: /(?:\b0x[a-f\d]+|(?:\b\d+(?:\.\d*)?|\B\.\d+)(?:e[-+]?\d+)?)i?/i,
  operator:
    /[*/%^!=]=?|~|\+[=+]?|-[=-]?|\|[=|]?|&(?:=|&|\^=?)?|>(?:>=?|=)?|<(?:<=?|=|-)?|:=|\.\.\./
})

Prism.languages.aussie = aussieSyntax

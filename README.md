# hoicolor
> a simple rust lib to convert from hearts of iron 4 (hoi4) escaped color sequences to ansi for stdout.


the original purpose in creating this project was for my other hoi4 related project *hoiscan*. this along with *hoiscan* is my first experience using rust. originally i was going to use python or javascript, but bindings to the steamworks library for those languages were limited. my other options were c/cpp/rust. i chose rust because of my positive experience building/using rust from other open source projects.


hoi4 uses escape sequences similar to minecraft¹. a start of a color is indicated by the unicode character `U+0011 ()` followed by and english character representing the color (e.g. White: W). to end a sequence the unicode character followed by `!` is used². stdout however cannot understand this. so, hoicolor maps a hoi4 colored string and replaces the colors with ansi color sequences³.


thanks
* @ewof for the hoi4 colors paste


todo
* api to convert from ansi/human readable color -> hoi4 color


references
1. [minecraft colors](https://minecraft.wiki/w/Formatting_codes#Color_codes)
2. [hoi4 colors paste](https://pastebin.com/raw/esAQQ8RE)
3. [ansi colors](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)


license
* [MIT](LICENSE)

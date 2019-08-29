md = $(wildcard *.md)
html = $(md:.md=.html)

css_file = ../css/blood.css

pandoc_misc_args = -s --mathjax -i 
pandoc_reveal_args = -t revealjs -V revealjs-url=../../reveal.js -V transition=fade
pandoc_css_arg = --css $(css_file)
pandoc_args = $(pandoc_misc_args) $(pandoc_reveal_args) $(pandoc_css_arg)

all: $(html)

%.html : %.md
	pandoc $< -o $@ $(pandoc_args)

clean:
	rm -f *.html

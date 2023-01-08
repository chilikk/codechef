rust-%:
	mkdir $* $*/src
	printf '[package]\nname = "$(shell echo $* | tr A-Z a-z)"\nversion = "0.0.1"\n' > $*/Cargo.toml
	printf '*\n!.gitignore\n!Cargo.toml\n!src\n!*.rs\n' > $*/.gitignore
	cp template.rs $*/src/main.rs

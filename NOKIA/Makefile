TARGET=prog.beam

all: $(TARGET)

%.beam: %.erl
	erlc $<

run: $(TARGET)
	erl -noshell -eval "$(<:.beam=):main(), init:stop()"

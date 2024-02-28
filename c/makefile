CC=gcc
CFLAGS= -Wall -std=c11 -O3
LDFLAGS=
EXECUTABLE=msc
OBJECTS = $(patsubst %.c, %.o, $(wildcard *.c))

all: $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(LDFLAGS) $^ -o $@

%.o: %.c
	$(CC) $(CFLAGS) -c $^ -o $@

clean:
	rm -rf $(EXECUTABLE) $(OBJECTS) output

run: $(EXECUTABLE)
	mkdir -p output
	find ../testcases -type f -exec sh -c './msc "$$0" > output/$$(basename "$$0")' {} \;

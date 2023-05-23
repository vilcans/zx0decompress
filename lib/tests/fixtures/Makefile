FILES = \
		paper5 \
		paper4 \
		obj1 \
		paper6 \
		progc

COMPRESSED_FILES = $(FILES:=.zx0)

all: $(COMPRESSED_FILES)

# Compress with the zx0 command-line program.
# Install it with `cargo install zx0`.
%.zx0: %
	zx0 --quiet $<


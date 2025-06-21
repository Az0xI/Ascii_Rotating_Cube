NAME	=	rotating_cube

all:	build

build:
	cargo build
	mv target/debug/$(NAME) .

clean:
	rm -f $(NAME)

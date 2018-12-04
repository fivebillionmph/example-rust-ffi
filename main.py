import os

from mod.game import Game

def main():
	lib_file = os.path.join(
		os.path.dirname(os.path.realpath(__file__)),
		"game/target/debug/libgame.dylib"
	)
	with Game(lib_file) as game:
		print(game.get_name())
		game.set_n(20)
		print(game.get_n())
		game.set_name("new name")
		print(game.get_name())
		print(game.get_n())

main()

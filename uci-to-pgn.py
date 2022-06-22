import Chessnut
import chess
import chess.pgn

import sys

def long_sequence_to_pgn(seq):
    moves = seq
    game = chess.pgn.Game()
#     with open(filename, 'r') as f:
#         moves = f.read().split('\n');
    game.setup(chess.Board("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3"))
    node = game
    for move in moves:
        node = node.add_variation(chess.Move.from_uci(move))
    return str(game)

uci_long = sys.stdin.read().strip().split('\n')
print(uci_long)
print(long_sequence_to_pgn(uci_long))
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <cmath>
#include <queue>
#include <algorithm>

struct Coord {
    int x = 0;
    int y = 0;
	bool operator==(const Coord& rhs) const { return this->x == rhs.x && this->y == rhs.y;}
	Coord(int x, int y) : x(x), y(y) {}
};

struct GridSquare {
    bool visited = false;
    char c = '\0';
	int dist = 2000000;
	Coord coord{0, 0};
};

class Grid {
public:
    int len_x = 0;
    int len_y = 0;
    GridSquare** squares = nullptr;
    Coord start{0,0};
    Coord end{0,0};
    std::vector<char> path = {};

	void calc_lengths() const {
		// Calculate the path distances starting from the end point
		auto cmp = [](GridSquare* a, GridSquare* b) { return a->dist > b->dist; };
		auto queue = std::priority_queue<GridSquare*, std::vector<GridSquare*>, decltype(cmp)>(cmp);

		int x = this->end.x;
		int y = this->end.y;
		auto curr = this->squares[y * len_x + x];
		curr->dist = 0;
		queue.push(curr);

		// Check in 3x3 area around the point
		// Only continue if its distance is bigger than the distance checked, and is possible
		// Impossible points have distance +infinity
		// Point gets distance of self + 1
		// PriorityQueue check out the lowest points
		std::vector<Coord> coords;

		while (!queue.empty()) {
			GridSquare* current = queue.top();
			current = this->squares[current->coord.y * this->len_x + current->coord.x];
			current->visited = true;
			queue.pop();
			if (current->coord == start) {
				std::cout << "START!!! " << current->dist << std::endl;
			}

			int y_min = current->coord.y == 0 ? current->coord.y : current->coord.y - 1;
			int y_max = current->coord.y == this->len_y - 1 ? current->coord.y : current->coord.y + 1;
			int x_min = current->coord.x == 0 ? current->coord.x : current->coord.x - 1;
			int x_max = current->coord.x == this->len_x - 1 ? current->coord.x : current->coord.x + 1;

			for (int y_inner = y_min; y_inner <= y_max; y_inner++) {
				for (int x_inner = x_min; x_inner <= x_max; x_inner++) {
					if (x_inner == current->coord.x && y_inner == current->coord.y || (pow(x_inner - current->coord.x, 2) + pow(y_inner - current->coord.y, 2)) == 2) continue;
					auto next = this->squares[y_inner * len_x + x_inner];
					if (next->dist <= current->dist) continue;
					if (current->c - next->c != 1 && current->c - next->c != 0 && current->c - next->c != -2) continue;
					if (next->dist > current->dist + 1)
						next->dist = current->dist + 1;
					if (std::find(coords.begin(), coords.end(), next->coord) != coords.end()) {
						continue;
					}
					if (!next->visited) {
						queue.push(next);
						coords.push_back(next->coord);
					}
				}
			}
		}
	}

     ~Grid() {
         delete[] squares;
     }
};

int main(int argc, char** argv) {
	if (argc < 2) {
		std::cerr << "Please provide an argument" << std::endl;
		exit(1);
	}
	std::cout << "Using file: " << argv[1] << std::endl;
    std::ifstream file = std::ifstream(argv[1]);
    std::string line;
    int len_y = 0;
    int len_x = 0;
    Coord start{0, 0};
    Coord end{0, 0};
    Grid grid;

	while (std::getline(file, line)) {
		std::cout << line << std::endl;
		len_x = line.size();
		len_y++;
	}

	file.close();
	int y = 0;
	grid.squares = new GridSquare*[len_y * len_x];

	file = std::ifstream(argv[1]);
    while (std::getline(file, line)) {
        for (int i = 0; i < line.size(); i++) {
            char c = line[i];
            if (c == 'S') {
                start.x = i;
                start.y = y;
                c = 'a';
            } else if (c == 'E') {
                end.x = i;
                end.y = y;
                c = 'z';
            }
            auto* square = new GridSquare(); // oooooh, spoooky heap allocation
            square->c = c;
            square->visited = false;
			square->coord.x = i;
			square->coord.y = y;
            grid.squares[y * len_x + i] = square;
        }
		y++;
    }

    grid.len_x = len_x;
    grid.len_y = len_y;
    grid.start.x = start.x;
    grid.start.y = start.y;
    grid.end.x = end.x;
    grid.end.y = end.y;
    std::cout << "Start: X(" << start.x << "), Y(" << start.y << ")" << std::endl;
    std::cout << "End: X(" << end.x << "), Y(" << end.y << ")" << std::endl;
    //grid.solve(start.x, start.y);
	grid.calc_lengths();
	int shortest_a = 2000000;
	for (int row = 0; row < len_y; row++) {
		for (int col = 0; col < len_x; col++) {
			auto square = grid.squares[row * len_x + col];
			//std::cout << square->dist << ", ";
			if (square->c == 'a' && square->dist < shortest_a) {
				shortest_a = square->dist;
			}
		}
		std::cout << std::endl;
	}
	std::cout << "Shortest A (Part B): " << shortest_a << std::endl;
	file.close();
}
// Advent Of Code - Day 2 , Puzzle 1

#include <iostream>
#include<fstream>
#include<string>
#include<cstdio>
#include<sstream>

using namespace std;

int extractInteger(string a) {
	stringstream b;
	b << a;

	string temp;
	int find;
	while (!b.eof()) {
		b >> temp;
		if (stringstream(temp) >> find) {
			return find;
		}
	}
}

int main() {
	int horizontal = 0, depth = 0;
	ifstream InputFile("--<insert file location here>--");
	string inputText;

	while (getline(InputFile, inputText)) {
		char txt[30];
		strcpy_s(txt, inputText.c_str());
		int val;
		val = extractInteger(inputText);

		cout << txt[0] << " " << val << endl;

		if (txt[0] == 'f') {
			horizontal += val;
		}
		else if (txt[0] == 'd') {
			depth += val;
		}
		else if (txt[0] == 'u') {
			depth -= val;
		}
	}

	long int required = horizontal * depth;
	std::cout << std::endl << "The required value for you: " << required;
	return 0;
}


/*
    forward X increases the horizontal position by X units.
    down X increases the depth by X units.
    up X decreases the depth by X units.
*/

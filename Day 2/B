// Advent Of Code 2021 - Day 2 , Puzzle 2

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
	int horizontal = 0, depth = 0, aim = 0;
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
			depth += aim * val;
		}
		else if (txt[0] == 'd') {
			aim += val;
		}
		else if (txt[0] == 'u') {
			aim -= val;
		}
	}

	long int required = horizontal * depth;
	std::cout << std::endl << "The required value for you: " << required;
	return 0;
}

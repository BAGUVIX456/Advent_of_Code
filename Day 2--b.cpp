//the second part of the puzzle for Day 2
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
	ifstream InputFile("C:/Users/Gumbi/Desktop/Dhanvith Files/AoC Day 2 input.txt");
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


/*
	forward X increases the horizontal position by X units.
	down X increases the depth by X units.
	up X decreases the depth by X units.
*/



// Run program: Ctrl + F5 or Debug > Start Without Debugging menu
// Debug program: F5 or Debug > Start Debugging menu

// Tips for Getting Started: 
//   1. Use the Solution Explorer window to add/manage files
//   2. Use the Team Explorer window to connect to source control
//   3. Use the Output window to see build output and other messages
//   4. Use the Error List window to view errors
//   5. Go to Project > Add New Item to create new code files, or Project > Add Existing Item to add existing code files to the project
//   6. In the future, to open this project again, go to File > Open > Project and select the .sln file

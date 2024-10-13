#pragma once

#include "main.hpp"

#include "ESCtoken.hpp"

class ESClexer {
	public:
		ESClexer(std::string source);
		~ESClexer();
		ESCtoken nextToken();
	private:
		std::string source;
		int line = 1;
		int lineStart;
		int current;
		int start;
		ESCtoken makeToken(ESCtoken::TokenType type);
		char advance();
		char currentChar();
		char peek();
		char peek2();
		bool match(char c);
		bool atEOF();
		void skipWhitespace();
};
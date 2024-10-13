#include "main.hpp"

#include "ESClexer.hpp"

#include "ESCtoken.hpp"

ESClexer::ESClexer(std::string source) {
	this->source = source;
	lineStart = 0;
	current = 0;
	start = 0;
}

ESClexer::~ESClexer() {}

ESCtoken ESClexer::makeToken(ESCtoken::TokenType type) {
	ESCtoken token = {type, source.substr(start, current - start), line, start - lineStart};
	return token;
}

char ESClexer::advance() {
	current++;
	return source[current - 1];
}

char ESClexer::peek() {
	return source[current];
}

char ESClexer::peek2() {
	return source[current + 1];
}

bool ESClexer::match(char c) {
	if (atEOF()) return false;
	if (source[current] != c) return false;
	current++;
	return true;
}

void ESClexer::skipWhitespace() {
	for (;;) {
    	char c = peek();
    	switch (c) {
    		case ' ':
    		case '\r':
			case '\t':
        		advance();
        		break;
			case '/':
     			if (peek() == '/') {
         			 while (peek2() != '\n' && !atEOF()) advance();
        		} else {
          			return;
        		}
        break;
      		default:
        		return;
    }
  }
}

bool ESClexer::atEOF() {
	return source.size() - 1 <= current;
}

ESCtoken ESClexer::nextToken() {
	skipWhitespace();

	start = current;

	if (atEOF()) {
		return makeToken(ESCtoken::ESCEOF);
	}

	char c = advance();

	switch (c) {
		case '\n':
			line++;
			lineStart = current;
			return makeToken(ESCtoken::NL);
		case '+':
			return makeToken(
				match('=') ? ESCtoken::PLUS_EQUALS : ESCtoken::PLUS
			);
		case '-':
			return makeToken(
				match('=') ? ESCtoken::MINUS_EQUALS : ESCtoken::MINUS
			);
		case '*':
			return makeToken(
				match('=') ? ESCtoken::TIMES_EQUALS : ESCtoken::TIMES
			);
		case '/':
			return makeToken(
				match('=') ? ESCtoken::DIVIDE_EQUALS : ESCtoken::DIVIDE
			);
		default:
			for (;;) {
				if (atEOF()) break;
				char c = peek();
				logger.logTrace("Inside default loop char: " + std::to_string(c));
				if (c == ' ' || c == '\n' || c == '\t') break;
				else advance();
			}
			return makeToken(ESCtoken::IDENTIFIER);
	}

	logger.logFatal("Unreachable code reached!");
	return ESCtoken();
}
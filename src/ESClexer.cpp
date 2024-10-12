#include "common.hpp"

#include "ESClexer.hpp"

#include "ESCtoken.hpp"

ESClexer::ESClexer(std::string source) {
	this->source = source;
}

ESClexer::~ESClexer() {}

ESCtoken ESClexer::makeToken(ESCtoken::TokenType type) {
	ESCtoken token = {type, source.substr(start, current - start), line, start - lineStart};
	start = ++current;
	return token;
}



ESCtoken ESClexer::nextToken() {
	logger.logTrace("Inside nextToken");

	

	logger.logFatal("Unreachable code reached!");
	return ESCtoken();
}
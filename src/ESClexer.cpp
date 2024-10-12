#include "common.hpp"

#include "ESClexer.hpp"

#include "ESCtoken.hpp"

ESClexer::ESClexer(std::string source) {
	this->source = source;
}

ESClexer::~ESClexer() {
	
}

ESCtoken ESClexer::nextToken() {
	
}
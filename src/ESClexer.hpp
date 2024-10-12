#pragma once

#include "common.hpp"

#include "ESCtoken.hpp"

class ESClexer {
	public:
		ESClexer(std::string source);
		~ESClexer();
		ESCtoken nextToken();
	private:
		std::string source;
		int line = 1;
		int lineStart = 0;
		int current = 0;
		int start = 0;
		ESCtoken makeToken(ESCtoken::TokenType type);
		
};
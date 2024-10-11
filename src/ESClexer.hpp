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
		int current;
		int previous;
};
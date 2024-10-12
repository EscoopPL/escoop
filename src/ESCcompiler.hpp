#pragma once

#include "ESClexer.hpp"

class ESCcompiler {
	public:
		ESCcompiler(std::string source);
		~ESCcompiler();
		std::vector<ESCtoken> compile();

	private:
		ESClexer* lexer;
};
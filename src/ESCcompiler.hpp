#pragma once

#include "main.hpp"

#include "ESClexer.hpp"

class ESCcompiler {
	public:
		ESCcompiler(std::string source);
		~ESCcompiler();
		std::vector<ESCtoken> compile();

	private:
		ESClexer* lexer;
};
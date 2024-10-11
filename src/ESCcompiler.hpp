#pragma once

#include "ESClexer.hpp"

class ESCcompiler {
	public:
		ESCcompiler(std::string source);
		~ESCcompiler();

	private:
		ESClexer* lexer;
};
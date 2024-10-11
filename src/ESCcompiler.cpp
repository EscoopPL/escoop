#include "common.hpp"

#include "ESCcompiler.hpp"

#include "ESClexer.hpp"

ESCcompiler::ESCcompiler(std::string source) {
	lexer = new ESClexer(source);
}

ESCcompiler::~ESCcompiler() {
	delete lexer;
}
#include "common.hpp"

#include "ESCcompiler.hpp"

#include "ESClexer.hpp"

ESCcompiler::ESCcompiler(std::string source) {
	lexer = new ESClexer(source);
}

ESCcompiler::~ESCcompiler() {
	delete lexer;
}

std::vector<ESCtoken> ESCcompiler::compile() {
	ESCtoken token = lexer->nextToken();

	std::vector<ESCtoken> tokens = std::vector<ESCtoken>();

	while (token.type != ESCtoken::ESCEOF) {
		tokens.push_back(token);
		token = lexer->nextToken();
	}

	return tokens;
}
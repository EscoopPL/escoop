#include "main.hpp"

#include "ESCcompiler.hpp"

#include "ESClexer.hpp"

ESCcompiler::ESCcompiler(std::string source) {
	lexer = new ESClexer(source);
}

ESCcompiler::~ESCcompiler() {
	delete lexer;
}

std::vector<ESCtoken> ESCcompiler::compile() {
	logger.logTrace("Compile function started");

	std::vector<ESCtoken> tokens = std::vector<ESCtoken>();

	ESCtoken token = lexer->nextToken();
	tokens.push_back(token);
	logger.logTrace("Created token list");
	while (token.type != ESCtoken::ESCEOF && token.type != ESCtoken::ESCNULL) {
		token = lexer->nextToken();
		tokens.push_back(token);
	}

	if (token.type == ESCtoken::ESCNULL) {
		logger.logFatal("Error while scanning program");
	}

	return tokens;
}
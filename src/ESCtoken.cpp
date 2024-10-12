#include "common.hpp"

#include "ESCtoken.hpp"

ESCtoken::ESCtoken() {
	type = ESCtoken::ESCNULL;
	lexeme = "";
	line = -1;
	column = -1;
}

ESCtoken::ESCtoken(TokenType type, std::string lexeme, int line, int column) {
	this->type = type;
	this->lexeme = lexeme;
	this->line = line;
	this->column = column;
}

std::string ESCtoken::getName(TokenType type) {
	switch (type) {
		case STRING:
			return "STRING";
		case INT:
			return "INT";
		case FLOAT:
			return "FLOAT";
		case BOOL:
			return "BOOL";
		case CHAR:
			return "CHAR";
		case ARRAY:
			return "ARRAY";
		case CLASS:
			return "CLASS";
		case INTERFACE:
			return "INTERFACE";
		case RECORD:
			return "RECORD";
		case UNION:
			return "UNION";
		case VARIANT:
			return "VARIANT";
		case RUNFILE:
			return "RUNFILE";
		case IMPORT:
			return "IMPORT";
		case VIRTUAL:
			return "VIRTUAL";
		case IF:
			return "IF";
		case IDENTIFIER_KEY:
			return "IDENTIFIER_KEY";
		case CASE:
			return "CASE";
		case SWITCH:
			return "SWITCH";
		case WHILE:
			return "WHILE";
		case FOR:
			return "FOR";
		case FOREACH:
			return "FOREACH";
		case AS:
			return "AS";
		case FUNC:
			return "FUNC";
		case DO:
			return "DO";
		case IS:
			return "IS";
		case THEN:
			return "THEN";
		case END:
			return "END";
		case NOT:
			return "NOT";
		case OR:
			return "OR";
		case AND:
			return "AND";
		case INHERITS:
			return "INHERITS";
		case CONTAINS:
			return "CONTAINS";
		case CONSTRUCT:
			return "CONSTRUCT";
		case EQUALS_KEY:
			return "EQUALS_KEY";
		case COMPFUNC:
			return "COMPFUNC";
		case BULK:
			return "BULK";
		case NEW:
			return "NEW";
		case VOID:
			return "VOID";
		case STRING_LIT:
			return "STRING_LIT";
		case CHAR_LIT:
			return "CHAR_LIT";
		case INT_LIT:
			return "INT_LIT";
		case FLOAT_LIT:
			return "FLOAT_LIT";
		case PLUS:
			return "PLUS";
		case MINUS:
			return "MINUS";
		case TIMES:
			return "TIMES";
		case DIVIDE:
			return "DIVIDE";
		case PLUS_EQUALS:
			return "PLUS_EQUALS";
		case MINUS_EQUALS:
			return "MINUS_EQUALS";
		case TIMES_EQUALS:
			return "TIMES_EQUALS";
		case DIVIDE_EQUALS:
			return "DIVIDE_EQUALS";
		case EQUALS:
			return "EQUALS";
		case ESCEOF:
			return "ESCEOF";
		case ESCNULL:
			return "ESCNULL";
		case IDENTIFIER:
			return "IDENTIFIER";
		default:
			return "ERR";
	}
}

void ESCtoken::logDebug() {
	std::string output = "{type = ";
	output += getName(type);
	output += ", line = ";
	output += std::to_string(line);
	output += ", column = ";
	output += std::to_string(column);
	output += "}";
	logger.logDebug(output);
}
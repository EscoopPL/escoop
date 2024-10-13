#include "main.hpp"

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

ESCtoken::TokenType ESCtoken::getType(std::string name) {
	if ("STRING" == name) return ESCtoken::STRING;
	else if ("INT" == name) return ESCtoken::INT;
	else if ("FLOAT" == name) return ESCtoken::FLOAT;
	else if ("BOOL" == name) return ESCtoken::BOOL;
	else if ("CHAR" == name) return ESCtoken::CHAR;
	else if ("ARRAY" == name) return ESCtoken::ARRAY;
	else if ("CLASS" == name) return ESCtoken::CLASS;
	else if ("INTERFACE" == name) return ESCtoken::INTERFACE;
	else if ("RECORD" == name) return ESCtoken::RECORD;
	else if ("UNION" == name) return ESCtoken::UNION;
	else if ("VARIANT" == name) return ESCtoken::VARIANT;
	else if ("RUNFILE" == name) return ESCtoken::RUNFILE;
	else if ("IMPORT" == name) return ESCtoken::IMPORT;
	else if ("VIRTUAL" == name) return ESCtoken::VIRTUAL;
	else if ("IF" == name) return ESCtoken::IF;
	else if ("IDENTIFIER_KEY" == name) return ESCtoken::IDENTIFIER_KEY;
	else if ("CASE" == name) return ESCtoken::CASE;
	else if ("SWITCH" == name) return ESCtoken::SWITCH;
	else if ("WHILE" == name) return ESCtoken::WHILE;
	else if ("FOR" == name) return ESCtoken::FOR;
	else if ("FOREACH" == name) return ESCtoken::FOREACH;
	else if ("AS" == name) return ESCtoken::AS;
	else if ("FUNC" == name) return ESCtoken::FUNC;
	else if ("DO" == name) return ESCtoken::DO;
	else if ("IS" == name) return ESCtoken::IS;
	else if ("THEN" == name) return ESCtoken::THEN;
	else if ("END" == name) return ESCtoken::END;
	else if ("NOT" == name) return ESCtoken::NOT;
	else if ("OR" == name) return ESCtoken::OR;
	else if ("AND" == name) return ESCtoken::AND;
	else if ("INHERITS" == name) return ESCtoken::INHERITS;
	else if ("CONTAINS" == name) return ESCtoken::CONTAINS;
	else if ("CONSTRUCT" == name) return ESCtoken::CONSTRUCT;
	else if ("EQUALS_KEY" == name) return ESCtoken::EQUALS_KEY;
	else if ("COMPFUNC" == name) return ESCtoken::COMPFUNC;
	else if ("BULK" == name) return ESCtoken::BULK;
	else if ("NEW" == name) return ESCtoken::NEW;
	else if ("VOID" == name) return ESCtoken::VOID;
	else if ("STRING_LIT" == name) return ESCtoken::STRING_LIT;
	else if ("CHAR_LIT" == name) return ESCtoken::CHAR_LIT;
	else if ("INT_LIT" == name) return ESCtoken::INT_LIT;
	else if ("FLOAT_LIT" == name) return ESCtoken::FLOAT_LIT;
	else if ("PLUS" == name) return ESCtoken::PLUS;
	else if ("MINUS" == name) return ESCtoken::MINUS;
	else if ("TIMES" == name) return ESCtoken::TIMES;
	else if ("DIVIDE" == name) return ESCtoken::DIVIDE;
	else if ("PLUS_EQUALS" == name) return ESCtoken::PLUS_EQUALS;
	else if ("MINUS_EQUALS" == name) return ESCtoken::MINUS_EQUALS;
	else if ("TIMES_EQUALS" == name) return ESCtoken::TIMES_EQUALS;
	else if ("DIVIDE_EQUALS" == name) return ESCtoken::DIVIDE_EQUALS;
	else if ("EQUALS" == name) return ESCtoken::EQUALS;
	else if ("ESCEOF" == name) return ESCtoken::ESCEOF;
	else if ("ESCNULL" == name) return ESCtoken::ESCNULL;
	else if ("IDENTIFIER" == name) return ESCtoken::IDENTIFIER_KEY;
	else if ("NL" == name) return ESCtoken::NL;

	return ESCtoken::ESCNULL;
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
		case NL:
			return "NL";
		default:
			return "ERR";
	}
}

void ESCtoken::logDebug() {
	std::string output = "{type = ";
	output += getName(type);
	output += ", lexeme = ";
	output += lexeme;
	output += ", line = ";
	output += std::to_string(line);
	output += ", column = ";
	output += std::to_string(column);
	output += "}";
	logger.logDebug(output);
}
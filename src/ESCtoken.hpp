#pragma once

#include "main.hpp"

class ESCtoken {
	public:
		typedef enum {
			// Data Types
			STRING,
			INT,
			FLOAT,
			BOOL,
			CHAR,
			ARRAY,
		
			// Keywords
			CLASS,
			INTERFACE,
			RECORD,
			UNION,
			VARIANT,
			RUNFILE,
			IMPORT,
			VIRTUAL,
			IF,
			IDENTIFIER_KEY,
			CASE,
			SWITCH,
			WHILE,
			FOR,
			FOREACH,
			AS,
			FUNC,
			DO,
			IS,
			THEN,
			END,
			NOT,
			OR,
			AND,
			INHERITS,
			CONTAINS,
			CONSTRUCT,
			EQUALS_KEY,
			COMPFUNC,
			BULK,
			NEW,
			VOID,
		
			// Literals
			STRING_LIT,
			CHAR_LIT,
			INT_LIT,
			FLOAT_LIT,
		
			// Symbols
			PLUS,
			MINUS,
			TIMES,
			DIVIDE,
			PLUS_EQUALS,
			MINUS_EQUALS,
			TIMES_EQUALS,
			DIVIDE_EQUALS,
			EQUALS,
		
			// Miscellaneous
			ESCEOF,
			ESCNULL,
			IDENTIFIER,
			NL
		} TokenType;

		TokenType type;
		std::string lexeme;
		int line;
		int column;

		ESCtoken();
		ESCtoken(TokenType type, std::string lexeme, int line, int column);

		static std::string getName(TokenType type);
		static ESCtoken::TokenType getType(std::string name);

		void logDebug();
};
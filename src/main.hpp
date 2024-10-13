#pragma once

#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <algorithm>
#include <cctype>

#include "ESClogger.hpp"

extern ESClogger logger;

namespace std {
	std::string tolower(std::string str);
	std::string toupper(std::string str);
}
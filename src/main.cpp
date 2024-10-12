#include "common.hpp"

#include "ESCcompiler.hpp"
#include "ESClogger.hpp"

ESCloggerFlags loggerFlags = {ESCloggerFlags::LOG_LEVEL_ALL, true, true, false};
ESClogger logger(loggerFlags);

int main(int argc, char** argv) {
	logger.logTrace("Program started");

	std::string buf;
	std::string source = std::string("");
	std::string filename = std::string(argv[argc - 1]);
  	std::ifstream myfile(filename);
  	if (myfile.is_open())
  	{
  		while ( std::getline (myfile,buf) )
		{
 	    	source.append(buf);
			source.append("\n");
   		}
   		myfile.close();
  	}
	else logger.logFatal("Cannot open file " + filename);

	ESCcompiler compiler = ESCcompiler(source);

	logger.logTrace("Created compiler");

	std::vector<ESCtoken> tokens = compiler.compile();

	logger.logTrace("Finished compiling");

	for (int i = 0; i < tokens.size(); i++) {
		tokens.at(i).logDebug();
	}

	logger.logTrace("Program finished");
}
#pragma once

#include "common.hpp"

typedef struct {
	enum {
		LOG_LEVEL_ALL,
		LOG_LEVEL_WARN,
		LOG_LEVEL_ERROR,
		LOG_LEVEL_FATAL,
		LOG_LEVEL_NONE,
	} logLevel;
	bool showTrace;
	bool showDebug;
} ESCloggerFlags;

class ESClogger {
	public:
		ESClogger(ESCloggerFlags flags);
		void logTrace(std::string msg);
		void logDebug(std::string msg);
		void logInfo(std::string msg);
		void logWarn(std::string msg);
		void logError(std::string msg);
		void logFatal(std::string msg);
	private:
		ESCloggerFlags flags;
};
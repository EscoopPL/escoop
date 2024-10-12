#include "common.hpp"

#include "ESClogger.hpp"
#include "colors.hpp"

ESClogger::ESClogger(ESCloggerFlags flags) {
	this->flags = flags;
}

void ESClogger::logTrace(std::string msg) {
	if (!flags.showTrace) return;

	time_t timestamp;
	time(&timestamp);
	std::string timestampStr = std::string(ctime(&timestamp));

	timestampStr = timestampStr.erase(timestampStr.length() - 1, 1);

	std::cout << timestampStr << ": esc: " << stc::bold << stc::rgb_fg(200, 200, 200) << "trace: " << stc::reset << msg << std::endl;
}

void ESClogger::logDebug(std::string msg) {
	if (!flags.showDebug) return;

	time_t timestamp;
	time(&timestamp);
	std::string timestampStr = std::string(ctime(&timestamp));

	timestampStr = timestampStr.erase(timestampStr.length() - 1, 1);

	std::cout << timestampStr << ": esc: " << stc::bold << stc::rgb_fg(255, 255, 0) << "debug: " << stc::reset << msg << std::endl;
}

void ESClogger::logInfo(std::string msg) {
	if (!(flags.logLevel <= ESCloggerFlags::LOG_LEVEL_ALL)) return;

	time_t timestamp;
	time(&timestamp);
	std::string timestampStr = std::string(ctime(&timestamp));

	timestampStr = timestampStr.erase(timestampStr.length() - 1, 1);

	std::cout << timestampStr << ": esc: " << stc::bold << stc::rgb_fg(128, 128, 128) << "info: " << stc::reset << msg << std::endl;
}

void ESClogger::logWarn(std::string msg) {
	if (!(flags.logLevel >= ESCloggerFlags::LOG_LEVEL_WARN)) return;

	time_t timestamp;
	time(&timestamp);
	std::string timestampStr = std::string(ctime(&timestamp));

	timestampStr = timestampStr.erase(timestampStr.length() - 1, 1);

	std::cout << timestampStr << ": esc: " << stc::bold << stc::rgb_fg(128, 128, 0) << "warn: " << stc::reset << msg << std::endl;
}

void ESClogger::logError(std::string msg) {
	if (!(flags.logLevel >= ESCloggerFlags::LOG_LEVEL_ERROR)) return;

	time_t timestamp;
	time(&timestamp);
	std::string timestampStr = std::string(ctime(&timestamp));

	timestampStr = timestampStr.erase(timestampStr.length() - 1, 1);

	std::cout << timestampStr << ": esc: " << stc::bold << stc::rgb_fg(200, 0, 0) << "error: " << stc::reset << msg << std::endl;
}

void ESClogger::logFatalBypass(std::string msg) {
	time_t timestamp;
	time(&timestamp);
	std::string timestampStr = std::string(ctime(&timestamp));

	timestampStr = timestampStr.erase(timestampStr.length() - 1, 1);

	std::cout << timestampStr << ": esc: " << stc::bold << stc::rgb_fg(255, 0, 0) << "fatal: " << stc::reset << msg << std::endl;
}

void ESClogger::logFatal(std::string msg) {
	time_t timestamp;
	time(&timestamp);
	std::string timestampStr = std::string(ctime(&timestamp));

	timestampStr = timestampStr.erase(timestampStr.length() - 1, 1);

	std::cout << timestampStr << ": esc: " << stc::bold << stc::rgb_fg(255, 0, 0) << "fatal: " << stc::reset << msg << std::endl;

	if (flags.exitOnFatal) exit(1);
	logger.logDebug("Fatal error bypassed");
}
#ifndef LOGUTIL_H
#define LOGUTIL_H

#include <QString>


class LogUtil {

    public:
        static void logCommand(const QString &playerName, const QString &command);

        static void logNpcTalk(const QString &npcName, const QString &message);
};

#endif // LOGUTIL_H

#include "shoutcommand.h"


#define super Command

ShoutCommand::ShoutCommand(QObject *parent) :
    super(parent) {

    setDescription("Shout a message. Everyone in the same area and every "
                   "adjacent area can hear it.\n"
                   "\n"
                   "Example: shout Anyone there?");
}

ShoutCommand::~ShoutCommand() {
}

void ShoutCommand::execute(Player *player, const QString &command) {

    super::prepareExecute(player, command);

    if (!assertWordsLeft("Shout what?")) {
        return;
    }

    QString message = takeRest();

    player->shout(message);
}

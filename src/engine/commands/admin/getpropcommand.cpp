#include "getpropcommand.h"

#include "conversionutil.h"
#include "util.h"


#define super AdminCommand

GetPropCommand::GetPropCommand(QObject *parent) :
    super(parent) {

    setDescription("Show the value of some object's property.\n"
                   "\n"
                   "Usage: get-prop <object-name> [#] <property-name>");
}

GetPropCommand::~GetPropCommand() {
}

void GetPropCommand::execute(Player *player, const QString &command) {

    super::prepareExecute(player, command);

    GameObjectPtr object = takeObject(currentArea()->objects());
    if (!requireSome(object, "Object not found.")) {
        return;
    }

    QString propertyName = Util::fullPropertyName(object.cast<GameObject *>(), takeWord());

    if (propertyName == "id") {
        send(QString::number(object->id()));
    } else {
        QVariant value = object->property(propertyName.toAscii().constData());
        send(ConversionUtil::toUserString(value));
    }
}

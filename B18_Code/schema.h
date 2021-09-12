#ifndef B18_CODE_SCHEMA_H
#define B18_CODE_SCHEMA_H
#include "define/constants.h"
#include "disk_structures.h"

int createRel(char relname[16], int nAttrs, char attrs[][ATTR_SIZE], int attrtypes[]);
Attribute *make_relcatrec(char relname[16], int nAttrs, int nRecords, int firstBlock, int lastBlock);
Attribute* make_attrcatrec(char relname[ATTR_SIZE], char attrname[ATTR_SIZE], int attrtype, int rootBlock, int offset);
int deleteRel(char relname[ATTR_SIZE]);

#endif //B18_CODE_SCHEMA_H
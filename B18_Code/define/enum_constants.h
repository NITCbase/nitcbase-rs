//
// Created by Jessiya Joy on 11/09/21.
//

#ifndef B18_CODE_ENUM_CONSTANTS_H
#define B18_CODE_ENUM_CONSTANTS_H

enum AttributeType {
	STRING,
	NUMBER
};

enum ConditionalOperators {
	EQ, // =
	LE, // <=
	LT, // <
	GE, // >=
	GT, // >
	NE // !=
}

enum BlockType {
	BLOCK_ALLOCATION_MAP,
	RECORD,
	IND_INTERNAL,
	IND_LEAF,
	UNUSED_BLK
};

enum BlockHeaderField {
	BLOCK_TYPE = 0,
	P_BLOCK = 4,
	L_BLOCK = 8,
	R_BLOCK = 12,
	NUM_ENTRIES = 16,
	NUM_ATTRIBUTES = 20,
	NUMBER_SLOTS = 24,
	RESERVED = 28
};

enum RelationCatalogEntryField {
	RELATION_NAME_INDEX = 0,
	NO_ATTRIBUTES_INDEX = 1,
	NO_RECORDS_INDEX = 2,
	FIRST_BLOCK_INDEX = 3,
	LAST_BLOCK_INDEX = 4,
	NO_SLOTS_PER_BLOCK_INDEX = 5
};

enum AttributeCatalogEntryField {
	REL_NAME_INDEX = 0,
	ATTRIBUTE_NAME_INDEX = 1,
	ATTRIBUTE_TYPE_INDEX = 2,
	PRIMARY_FLAG_INDEX = 3,
	ROOT_BLOCK_INDEX = 4,
	OFFSET_INDEX = 5
};

enum ReturnTypes {
	SUCCESS,
    FAILURE,
    EXIT,
	E_OUTOFBOUND,
	E_FREESLOT,
	E_NOINDEX,
	E_DISKFULL,
	E_INVALIDBLOCK,
	E_RELNOTEXIST,
	E_RELEXIST,
	E_ATTRNOTEXIST,
	E_ATTREXIST,
	E_CACHEFULL,
	E_NOTOPEN,
	E_RELNOTOPEN,
	E_NATTRMISMATCH,
	E_DUPLICATEATTR,
	E_RELOPEN,
	E_ATTRTYPEMISMATCH,
	E_INVALID,
};


#endif //B18_CODE_ENUM_CONSTANTS_H

/* TODO :
 * - Return types
 * - Block types
 * - Block Header fields
 * - AttrCat entries
 * - RelCat entries
 */
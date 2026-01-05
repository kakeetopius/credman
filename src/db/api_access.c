#include <stdio.h>
#include <string.h>

#include "db/api_access.h"
#include "db/general.h"
#include "objects/api_obj.h"
#include "util/errors.h"

int check_apikey_exists(sqlite3 *db, char *api_name) {
    sqlite3_stmt *stmt;
    const char *sql = "SELECT EXISTS(SELECT 1 FROM api_keys WHERE api_name = ?);";
    int status;

    status = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    sqlite3_bind_text(stmt, 1, api_name, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    int exists = 0;
    if (sqlite3_step(stmt) == SQLITE_ROW) {
	exists = sqlite3_column_int(stmt, 0);
    }

    sqlite3_finalize(stmt);
    return exists == 1 ? DB_ROW_EXISTS : DB_ROW_NX;
}

int add_apikey_to_db(sqlite3 *db, Api_Key key) {
    if (!db) {
	printf("Database handle is NULL\n");
	return GENERAL_ERROR;
    }
    if (!key) {
	printf("Account object is NULL\n");
	return GENERAL_ERROR;
    }

    int status;
    sqlite3_stmt *pStmt = NULL;
    char *query = "INSERT INTO api_keys(api_name, service, user_name, api_key) VALUES (?, ?, ?, ?);";

    status = sqlite3_prepare_v2(db, query, -1, &pStmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 1, key->name, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 2, key->service, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 3, key->username, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }
    status = sqlite3_bind_text(pStmt, 4, key->key, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_step(pStmt);
    if (status != SQLITE_DONE) {
	printf("Error: %s", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }
    int affected_rows = sqlite3_changes(db);

    sqlite3_finalize(pStmt);
    if (affected_rows != 1) {
	return NON_AFFECTED_ERROR;
    }

    return SUCCESS_OP;
}

int delete_apikey_from_db(sqlite3 *db, char *key_name) {
    int status;
    sqlite3_stmt *pstmt = NULL;
    char *query = "DELETE FROM api_keys WHERE api_name = ?;";

    status = sqlite3_prepare_v2(db, query, -1, &pstmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pstmt, 1, key_name, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pstmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_step(pstmt);
    if (status != SQLITE_DONE) {
	printf("Error: %s", sqlite3_errmsg(db));
	sqlite3_finalize(pstmt);
	return SQLITE_RELATED_ERROR;
    }
    int affected_rows = sqlite3_changes(db);

    sqlite3_finalize(pstmt);
    if (affected_rows != 1) {
	return NON_AFFECTED_ERROR;
    }

    return SUCCESS_OP;
}

int update_api_db_field(sqlite3 *db, enum apikey_db_fields toUpdate, char *api_name, char *new_field_value) {
    char *query;
    sqlite3_stmt *pStmt;
    int status;
    switch (toUpdate) {
    case DB_API_NAME:
	query = "UPDATE api_keys SET api_name = ? WHERE api_name = ?;";
	break;
    case DB_API_USERNAME:
	query = "UPDATE api_keys SET user_name = ? WHERE api_name = ?;";
	break;
    case DB_API_SERVICE:
	query = "UPDATE api_keys SET service = ? WHERE api_name = ?;";
	break;
    case DB_API_KEY:
	query = "UPDATE api_keys SET api_key = ? WHERE api_name = ?;";
	break;
    }

    status = sqlite3_prepare_v2(db, query, -1, &pStmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 1, new_field_value, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 2, api_name, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_step(pStmt);
    if (status != SQLITE_DONE) {
	printf("Error: %s", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }
    int affected_rows = sqlite3_changes(db);

    sqlite3_finalize(pStmt);
    if (affected_rows != 1) {
	return NON_AFFECTED_ERROR;
    }

    return SUCCESS_OP;
}

int get_all_db_apikeys(sqlite3 *db, struct api_list *api_list) {
    if (!db) {
	return GENERAL_ERROR;
    }
    if (!api_list) {
	return GENERAL_ERROR;
    }

    const char *query = "SELECT api_name, service, user_name, api_key FROM api_keys;";
    sqlite3_stmt *pre_stmt = NULL;
    int status = 0;

    status = sqlite3_prepare_v2(db, query, -1, &pre_stmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    while ((status = sqlite3_step(pre_stmt)) == SQLITE_ROW) {
	unsigned const char *api_name = sqlite3_column_text(pre_stmt, 0);
	unsigned const char *service = sqlite3_column_text(pre_stmt, 1);
	unsigned const char *user_name = sqlite3_column_text(pre_stmt, 2);
	unsigned const char *api_key = sqlite3_column_text(pre_stmt, 3);
	int insert_status = insert_apinode(api_list, (char *)api_name, (char *)user_name, (char *)service, (char *)api_key);
	if (insert_status != 0) {
	    sqlite3_finalize(pre_stmt);
	    return GENERAL_ERROR;
	}
    }

    if (status != SQLITE_DONE) {
	printf("An error occured: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pre_stmt);
	return SQLITE_RELATED_ERROR;
    }

    sqlite3_finalize(pre_stmt);

    return SUCCESS_OP;
}

int get_db_apikey_by_name(sqlite3 *db, char *api_name, Api_Key key) {
    if (!key) {
	return GENERAL_ERROR;
    }
    int status;
    sqlite3_stmt *pstmt = NULL;
    char *query = "SELECT api_name, service, user_name, api_key FROM api_keys WHERE api_name = ?;";

    status = sqlite3_prepare_v2(db, query, -1, &pstmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pstmt, 1, api_name, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pstmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_step(pstmt);
    if (status != SQLITE_ROW) {
	printf("API KEY %s does not exist\n", api_name);
	sqlite3_finalize(pstmt);
	return SQLITE_RELATED_ERROR;
    }

    const unsigned char *key_name = sqlite3_column_text(pstmt, 0);
    const unsigned char *service = sqlite3_column_text(pstmt, 1);
    const unsigned char *user_name = sqlite3_column_text(pstmt, 2);
    const unsigned char *api_key = sqlite3_column_text(pstmt, 3);

    key->name = strdup((char *)key_name);
    key->username = strdup((char *)user_name);
    key->service = strdup((char *)service);
    key->key = strdup((char *)api_key);

    sqlite3_finalize(pstmt);
    return SUCCESS_OP;
}

#include <stdio.h>
#include <string.h>

#include "db/acc_access.h"
#include "db/general.h"
#include "objects/acc_obj.h"
#include "util/errors.h"

int check_account_exists(sqlite3 *db, char *acc_name) {
    sqlite3_stmt *stmt;
    const char *sql = "SELECT EXISTS(SELECT 1 FROM account WHERE acc_name = ?);";
    int status;

    status = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    sqlite3_bind_text(stmt, 1, acc_name, -1, SQLITE_TRANSIENT);
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

int add_account_to_db(sqlite3 *db, Account acc) {
    if (!db) {
	printf("Database handle is NULL\n");
	return GENERAL_ERROR;
    }
    if (!acc) {
	printf("Account object is NULL\n");
	return GENERAL_ERROR;
    }

    int status;
    sqlite3_stmt *pStmt = NULL;
    char *query = "INSERT INTO account(acc_name, user_name, password) VALUES (?, ?, ?);";

    status = sqlite3_prepare_v2(db, query, -1, &pStmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 1, acc->name, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 2, acc->username, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pStmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pStmt, 3, acc->password, -1, SQLITE_TRANSIENT);
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

int delete_account_from_db(sqlite3 *db, char *acc_name) {
    int status;
    sqlite3_stmt *pstmt = NULL;
    char *query = "DELETE FROM account WHERE acc_name = ?;";

    status = sqlite3_prepare_v2(db, query, -1, &pstmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pstmt, 1, acc_name, -1, SQLITE_TRANSIENT);
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

int update_acc_db_field(sqlite3 *db, enum account_db_fields toUpdate, char *acc_name, char *new_field_value) {
    char *query;
    sqlite3_stmt *pStmt;
    int status;
    switch (toUpdate) {
    case DB_ACC_NAME:
	query = "UPDATE account SET acc_name = ? WHERE acc_name = ?;";
	break;
    case DB_USER_NAME:
	query = "UPDATE account SET user_name = ? WHERE acc_name = ?;";
	break;
    case DB_ACC_PASSWORD:
	query = "UPDATE account SET password = ? WHERE acc_name = ?;";
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

    status = sqlite3_bind_text(pStmt, 2, acc_name, -1, SQLITE_TRANSIENT);
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

int get_db_account_by_name(sqlite3 *db, char *acc_name, struct account *acc) {
    if (!acc) {
	return GENERAL_ERROR;
    }
    int status;
    sqlite3_stmt *pstmt = NULL;
    char *query = "SELECT acc_name, user_name, password FROM account WHERE acc_name = ?;";

    status = sqlite3_prepare_v2(db, query, -1, &pstmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_bind_text(pstmt, 1, acc_name, -1, SQLITE_TRANSIENT);
    if (status != SQLITE_OK) {
	printf("Error: %s\n", sqlite3_errmsg(db));
	sqlite3_finalize(pstmt);
	return SQLITE_RELATED_ERROR;
    }

    status = sqlite3_step(pstmt);
    if (status != SQLITE_ROW) {
	printf("Account %s does not exist\n", acc_name);
	sqlite3_finalize(pstmt);
	return SQLITE_RELATED_ERROR;
    }

    const unsigned char *a_name = sqlite3_column_text(pstmt, 0);
    const unsigned char *username = sqlite3_column_text(pstmt, 1);
    const unsigned char *pass = sqlite3_column_text(pstmt, 2);

    acc->name = strdup((char *)a_name);
    acc->username = strdup((char *)username);
    acc->password = strdup((char *)pass);

    sqlite3_finalize(pstmt);
    return SUCCESS_OP;
}

int get_all_db_accounts(sqlite3 *db, struct account_list *acc_list) {
    if (!db) {
	return GENERAL_ERROR;
    }
    if (!acc_list) {
	return GENERAL_ERROR;
    }

    const char *query = "SELECT acc_name, user_name, password FROM account;";
    sqlite3_stmt *pre_stmt = NULL;
    int status = 0;

    status = sqlite3_prepare_v2(db, query, -1, &pre_stmt, NULL);
    if (status != SQLITE_OK) {
	printf("Error: %s", sqlite3_errmsg(db));
	return SQLITE_RELATED_ERROR;
    }

    while ((status = sqlite3_step(pre_stmt)) == SQLITE_ROW) {
	unsigned const char *acc_name = sqlite3_column_text(pre_stmt, 0);
	unsigned const char *user_name = sqlite3_column_text(pre_stmt, 1);
	unsigned const char *pass = sqlite3_column_text(pre_stmt, 2);
	int insert_status = insert_acc_node(acc_list, (char *)acc_name, (char *)pass, (char *)user_name);
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

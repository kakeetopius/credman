#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "util/argparser.h"
#include "util/errors.h"

struct Command commands[] = {
    {.name = "add", .argparser = addArgParser, .Run = runAdd},
    {.name = "get", .argparser = getArgParser, .Run = runGet},
    {.name = "change", .argparser = changeArgParser, .Run = runChange},
    {.name = "ls", .argparser = listArgParser, .Run = runList},
    {.name = "delete", .argparser = deleteArgParser, .Run = runDelete},
};

struct Command *current_command;

int num_of_commands = sizeof(commands) / sizeof(commands[0]);

int parse_args(int argc, char *argv[], struct Command **command) {
    if (argc < 2) {
	printf("%s", GENERAL_MESSAGE);
	return GENERAL_ERROR;
    }
    if (check_if_help_requested(argv[1]) == USER_REQUESTED_HELP) {
	printf("%s", GENERAL_MESSAGE);
	return USER_REQUESTED_HELP;
    }

    int sub_command_found = 0;
    struct Command *subcommand = NULL;
    for (int i = 0; i < num_of_commands; i++) {
	if (strings_match(argv[1], (char*)commands[i].name)) {
	    sub_command_found = 1;
	    subcommand = &commands[i];
	    break;
	}
    }
    if (!sub_command_found) {
	printf("Unknown command: %s\n", argv[1]);
	printf("%s\n", GENERAL_MESSAGE);
	return GENERAL_ERROR;
    }
    if (!subcommand) {
	return GENERAL_ERROR;
    }

    // initalising argument struct based on subcommand.
    void *arguments = NULL;
    if (strings_match((char*)subcommand->name, "add")) {
	arguments = malloc(sizeof(struct AddArgs));
    } else if (strings_match((char*)subcommand->name, "get")) {
	arguments = malloc(sizeof(struct GetArgs));
    } else if (strings_match((char*)subcommand->name, "change")) {
	arguments = malloc(sizeof(struct ChangeArgs));
    } else if (strings_match((char*)subcommand->name, "ls")) {
	arguments = malloc(sizeof(struct ListArgs));
    } else if (strings_match((char*)subcommand->name, "delete")) {
	arguments = malloc(sizeof(struct DeleteArgs));
    } else {
	return GENERAL_ERROR;
    }

    // argparser parses argv and then initialises arguments with the neccessary info
    int status = subcommand->argparser(argc, argv, arguments);
    if (status != SUCCESS_OP) {
	return status;
    }
    // add the pointer to the arguments to the command struct for later used.
    subcommand->arguments = arguments;

    *command = subcommand; // initalise main's pointer to point to the correct command stuct.
    current_command = subcommand;
    return SUCCESS_OP;
}

void free_arguments(struct Command *command) {
    if (!command) {
	return;
    }
    if (command->arguments) {
	free(command->arguments);
    }
}

bool strings_match(char *prefix, char *string) {
    // checks if a given prefix matches the given string.

    if (!*string || !*prefix) {
	// if the string is empty.
	return false;
    }

    while (*string && *string == *prefix) {
	string++;
	prefix++;
    }

    if (*prefix != 0) {
	// if the prefix did not reach null terminator meaning it did not match
	return false;
    }

    return true;
}

int check_if_help_requested(char *arg) {
    if (arg[0] == '-') {
	arg++;
	if (arg[0] == '-') {
	    arg++;
	}
    }
    if (strings_match(arg, "help")) {
	return USER_REQUESTED_HELP;
    }
    return SUCCESS_OP;
}

int addArgParser(int argc, char **argv, void *arguments) {
    if (!arguments) {
	return GENERAL_ERROR;
    }
    if (argc < 3) {
	printf("%s", ADD_MESSAGE);
	return GENERAL_ERROR;
    }

    struct AddArgs *args = (struct AddArgs *)arguments;

    if (check_if_help_requested(argv[2]) == USER_REQUESTED_HELP) {
	printf("%s", ADD_MESSAGE);
	return USER_REQUESTED_HELP;
    }
    args->flags = 0;
    args->secretName = argv[2];

    int i = 3;
    while (i < argc) {
	char *opt = argv[i];
	if (opt[0] == '-') {
	    // if argument
	    opt++;
	    if (opt[0] == '-') {
		// if argument like --batch
		opt++;
	    }
	    if (strings_match(opt, "batch")) {
		args->flags = args->flags | ADD_FLAG_BATCHFILE;
	    } else if (strings_match(opt, "type")) {
		if (argc == i + 1) {
		    printf("No type given. Use cman add -h for more information\n");
		    return GENERAL_ERROR;
		}
		char *type = argv[i + 1];
		if (strings_match(type, "login")) {
		    args->flags = args->flags | ADD_FLAG_TYPE_LOGIN;
		} else if (strings_match(type, "api")) {
		    args->flags = args->flags | ADD_FLAG_TYPE_APIKEY;
		} else {
		    printf("Unknown type: %s. Use cman add -h for more information.\n", type);
		    return GENERAL_ERROR;
		}
		i = i + 2; // skip checking the next argument.
		continue;
	    } else if (strings_match(opt, "no-auto")) {
		args->flags = args->flags | ADD_FLAG_NOAUTO;
	    } else if (strings_match(opt, "help")) {
		printf("%s", ADD_MESSAGE);
		return USER_REQUESTED_HELP;
	    } else {
		printf("Unknown option: %s. Use cman add -h for more information.s\n", argv[i]);
		return GENERAL_ERROR;
	    }
	} else {
	    printf("Unknown option: %s. Use cman add -h for more information\n", argv[i]);
	    return GENERAL_ERROR;
	}
	i++;
    }

    return SUCCESS_OP;
}

int getArgParser(int argc, char **argv, void *arguments) {
    if (!arguments) {
	return GENERAL_ERROR;
    }
    if (argc < 3) {
	printf("%s", GET_MESSAGE);
	return GENERAL_ERROR;
    }

    struct GetArgs *args = (struct GetArgs *)arguments;

    if (check_if_help_requested(argv[2]) == USER_REQUESTED_HELP) {
	printf("%s", GET_MESSAGE);
	return USER_REQUESTED_HELP;
    }
    args->flags = 0;
    args->secretName = argv[2];

    int i = 3; // start after secretName
    while (i < argc) {
	char *opt = argv[i];
	if (opt[0] == '-') {
	    // if argument
	    opt++;
	    if (opt[0] == '-') {
		// if argument like --arg
		opt++;
	    }
	    if (strings_match(opt, "field")) {
		if (argc == i + 1) {
		    // if this is the last argument in argument list.
		    printf("No field name given. Use cman get -h for more information\n");
		    return GENERAL_ERROR;
		}
		char *field = argv[i + 1];
		if (strings_match(field, "uname")) {
		    args->flags = args->flags | GET_FLAG_FIELD_USERNAME;
		} else if (strings_match(field, "pass")) {
		    args->flags = args->flags | GET_FLAG_FIELD_PASS;
		} else if (strings_match(field, "service")) {
		    args->flags = args->flags | GET_FLAG_FIELD_APISERVICE;
		} else if (strings_match(field, "key")) {
		    args->flags = args->flags | GET_FLAG_FIELD_APIKEY;
		} else if (strings_match(field, "accname")) {
		    args->flags = args->flags | GET_FLAG_FIELD_ACCNAME;
		} else if (strings_match(field, "apiname")) {
		    args->flags = args->flags | GET_FLAG_FIELD_APINAME;
		} else {
		    printf("Unknown field type: %s. Use cman get -h for more information.\n", field);
		    return GENERAL_ERROR;
		}

		i = i + 2; // skip checking the next argument.
		continue;
	    } else if (strings_match(opt, "type")) {
		if (argc == i + 1) {
		    printf("No type given. Use cman get -h for more information\n");
		    return GENERAL_ERROR;
		}
		char *type = argv[i + 1];
		if (strings_match(type, "login")) {
		    args->flags = args->flags | GET_FLAG_TYPE_LOGIN;
		} else if (strings_match(type, "api")) {
		    args->flags = args->flags | GET_FLAG_TYPE_APIKEY;
		} else {
		    printf("Unknown type: %s. Use cman get -h for more information.\n", type);
		    return GENERAL_ERROR;
		}
		i = i + 2; // skip checking the next argument.
		continue;
	    } else if (strings_match(opt, "quiet")) {
		args->flags = args->flags | GET_FLAG_QUIET;
	    } else if (strings_match(opt, "help")) {
		printf("%s", GET_MESSAGE);
		return USER_REQUESTED_HELP;
	    } else {
		printf("Unknown option: %s. Use cman get -h for more information.\n", argv[i]);
		return GENERAL_ERROR;
	    }
	} else {
	    printf("Unknown option: %s. Use cman get -h for more information.\n", argv[i]);
	    return GENERAL_ERROR;
	}
	i++;
    }

    return SUCCESS_OP;
}

int changeArgParser(int argc, char **argv, void *arguments) {
    if (!arguments) {
	return GENERAL_ERROR;
    }
    if (argc < 3) {
	printf("%s", CHANGE_MESSAGE);
	return GENERAL_ERROR;
    }

    struct ChangeArgs *args = (struct ChangeArgs *)arguments;
    if (strcmp(argv[2], "--master") == 0) {
	args->flags = args->flags | CHANGE_FLAG_MASTER;
	return SUCCESS_OP;
    }

    if (check_if_help_requested(argv[2]) == USER_REQUESTED_HELP) {
	printf("%s", CHANGE_MESSAGE);
	return USER_REQUESTED_HELP;
    }
    args->flags = 0;
    args->secretName = argv[2]; // guranteed to exist by the check above

    int i = 3; // start after secretName
    while (i < argc) {
	char *opt = argv[i];
	if (opt[0] == '-') {
	    // if argument
	    opt++;
	    if (opt[0] == '-') {
		// if argument like --arg
		opt++;
	    }
	    if (strings_match(opt, "field")) {
		if (argc == i + 1) {
		    // if this is the last argument in argument list.
		    printf("No field name given. Use cman change -h for more information\n");
		    return GENERAL_ERROR;
		}

		char *field = argv[i + 1];
		if (strings_match(field, "pass")) {
		    args->flags = args->flags | CHANGE_FLAG_FIELD_PASS;
		} else if (strings_match(field, "uname")) {
		    args->flags = args->flags | CHANGE_FLAG_FIELD_USERNAME;
		} else if (strings_match(field, "accname")) {
		    args->flags = args->flags | CHANGE_FLAG_FIELD_ACCNAME;
		} else if (strings_match(field, "apiname")) {
		    args->flags = args->flags | CHANGE_FLAG_FIELD_APINAME;
		} else if (strings_match(field, "service")) {
		    args->flags = args->flags | CHANGE_FLAG_FIELD_APISERVICE;
		} else if (strings_match(field, "key")) {
		    args->flags = args->flags | CHANGE_FLAG_FIELD_APIKEY;
		} else {
		    printf("Unknown field type: %s. Use cman change -h for more information.\n", field);
		    return GENERAL_ERROR;
		}

		i = i + 2; // skip checking the next argument.
		continue;
	    } else if (strings_match(opt, "type")) {
		if (argc == i + 1) {
		    printf("No type given. Use cman change -h for more information\n");
		    return GENERAL_ERROR;
		}
		char *type = argv[i + 1];
		if (strings_match(type, "login")) {
		    args->flags = args->flags | CHANGE_FLAG_TYPE_LOGIN;
		} else if (strings_match(type, "api")) {
		    args->flags = args->flags | CHANGE_FLAG_TYPE_APIKEY;
		} else {
		    printf("Unknown type: %s. Use cman change -h for more information\n", type);
		    return GENERAL_ERROR;
		}
		i = i + 2; // skip checking the next argument.
		continue;
	    } else if (strings_match(opt, "no-auto")) {
		args->flags = args->flags | CHANGE_FLAG_NOAUTO;
	    } else if (strings_match(opt, "help")) {
		printf("%s", CHANGE_MESSAGE);
		return USER_REQUESTED_HELP;
	    } else {
		printf("Unknown option: %s. Use cman change -h for more information\n", argv[i]);
		return GENERAL_ERROR;
	    }
	} else {
	    printf("Unknown option: %s. Use cman change -h for more information.\n", argv[i]);
	    return GENERAL_ERROR;
	}
	i++;
    }

    return SUCCESS_OP;
}

int listArgParser(int argc, char **argv, void *arguments) {
    if (!arguments) {
	return GENERAL_ERROR;
    }

    struct ListArgs *args = (struct ListArgs *)arguments;

    args->flags = 0;

    if (argc < 3) {
	return SUCCESS_OP;
    }

    if (check_if_help_requested(argv[2]) == USER_REQUESTED_HELP) {
	printf("%s", LS_MESSAGE);
	return USER_REQUESTED_HELP;
    }
    int i = 2;
    while (i < argc) {
	char *opt = argv[i];
	if (opt[0] == '-') {
	    // if argument
	    opt++;
	    if (opt[0] == '-') {
		// if argument like --arg
		opt++;
	    }
	    if (strings_match(opt, "type")) {
		if (argc == i + 1) {
		    printf("No type given. Use cman ls -h for more information\n");
		    return GENERAL_ERROR;
		}
		char *type = argv[i + 1];
		if (strings_match(type, "login")) {
		    args->flags = args->flags | LIST_FLAG_TYPE_LOGIN;
		} else if (strings_match(type, "api")) {
		    args->flags = args->flags | LIST_FLAG_TYPE_APIKEY;
		} else {
		    printf("Unknown type: %s. Use cman ls -h for more information.\n", type);
		    return GENERAL_ERROR;
		}
		i = i + 2; // skip checking the next argument.
		continue;
	    } else if (strings_match(opt, "help")) {
		printf("%s", LS_MESSAGE);
		return USER_REQUESTED_HELP;
	    } else {
		printf("Unknown option: %s. Use cman ls -h for more information.\n", argv[i]);
		return GENERAL_ERROR;
	    }
	    i++;
	} else {
	    printf("Unknown option: %s. Use cman ls -h for more information.\n", argv[i]);
	    return GENERAL_ERROR;
	}
    }

    return SUCCESS_OP;
}

int deleteArgParser(int argc, char **argv, void *arguments) {
    if (!arguments) {
	return GENERAL_ERROR;
    }

    if (argc < 3) {
	printf("%s\n", DELETE_MESSAGE);
	return GENERAL_ERROR;
    }

    struct DeleteArgs *args = (struct DeleteArgs *)arguments;

    args->flags = 0;

    if (check_if_help_requested(argv[2]) == USER_REQUESTED_HELP) {
	printf("%s", DELETE_MESSAGE);
	return USER_REQUESTED_HELP;
    }
    args->secretName = argv[2];
    int i = 3;
    while (i < argc) {
	char *opt = argv[i];
	if (opt[0] == '-') {
	    // if argument
	    opt++;
	    if (opt[0] == '-') {
		// if argument like --arg
		opt++;
	    }
	    if (strings_match(opt, "type")) {
		if (argc == i + 1) {
		    printf("No type given. Use cman delete -h for more information\n");
		    return GENERAL_ERROR;
		}
		char *type = argv[i + 1];
		if (strings_match(type, "login")) {
		    args->flags = args->flags | DELETE_FLAG_TYPE_LOGIN;
		} else if (strings_match(type, "api")) {
		    args->flags = args->flags | DELETE_FLAG_TYPE_APIKEY;
		} else {
		    printf("Unknown type: %s. Use cman delete -h for more information.\n", type);
		    return GENERAL_ERROR;
		}
		i = i + 2; // skip checking the next argument.
		continue;
	    } else if (strings_match(opt, "help")) {
		printf("%s", DELETE_MESSAGE);
		return USER_REQUESTED_HELP;
	    } else {
		printf("Unknown option: %s. Use cman delete -h for more information.\n", argv[i]);
		return GENERAL_ERROR;
	    }
	    i++;
	} else {
	    printf("Unknown option: %s. Use cman ls -h for more information.\n", argv[i]);
	    return GENERAL_ERROR;
	}
    }
    return SUCCESS_OP;
}


#ifndef ERRORS_H
#define ERRORS_H

/*---ERROR CODES----*/
#define SUCCESS_OP 0
#define USER_REQUESTED_HELP 1
#define LINE_EMPTY 2

#define NON_AFFECTED_ERROR 3
#define SQLITE_RELATED_ERROR 4
#define DB_ROW_EXISTS 5
#define DB_ROW_NX 6
#define WRONG_MASTER_PASSWORD 7

#define GENERAL_ERROR -1

#define GENERAL_MESSAGE "Usage: cman COMMAND [OPTIONS] \n\n"                                                                               \
			"cman is a simple tool to help manage and securely store secrets like login credentials and api keys locally.\n\n" \
			"COMMANDS:\n"                                                                                                      \
			"  ls 	      \t\tList all of the stored secrets of a particular type.\n"                                           \
			"  add	      \t\tAdd a new secret to the storage.\n"                                                               \
			"  change     \t\tChange details of a particular secret.\n"                                                        \
			"  get	      \t\tGet details for a particular secret.\n"                                                           \
			"  delete     \t\tDelete a secret from the storage.\n"                                                             \
			"  help	      \t\tDisplay this help message.\n\n"                                                                  \
			"Use cman COMMAND help for details about a command.\n\n"                                                           \
			"Note: cman checks for the credential database file path from the environment variable $CMAN_DBFILE\n"             \
			"      If that environment variable is not found it defaults to $HOME/.creds.db\n"

#define ADD_MESSAGE "Usage: cman add SECRET_NAME [OPTIONS]\n\n"                                                                                                \
		    "cman add adds the given secret to the storage.\n\n"                                                                                       \
		    "OPTIONS:\n"                                                                                                                               \
		    "  -t, --type [login | api]      \tSpecifies what type the secret is. 'login' for login credentials and 'api' for api keys.\n"             \
		    "  -b, --batch		     \t\tIf this option is given, SECRET_NAME is treated as file containing login credentials. See below.\n"               \
		    "  -h, --help		     \t\tShow this help message.\n"                                                                                         \
		    "  --no-auto		     \t\tIf this option is given and the type is 'login' the user is prompted for the password instead of generating one.\n" \
		    "\nRules for batch file:\n"                                                                                                                \
		    "1.Each line has details of a single set of login credentials i.e account name, user name and passowrd"                                    \
		    " separated by commas eg accName,uName,pass\n"                                                                                             \
		    "2.Empty lines are ignored\n"                                                                                                              \
		    "3.Spaces within the line are treated as part of the respective field\n"                                                                   \
		    "4.If it is required that program generates the password, use ? as a placeholder for the password i.e. "                                   \
		    "aName,uName,?\n"                                                                                                                          \
		    "5.The batch file is only supported for login credentials for now.\n"                                                                      \
		    "\nNote: If no type is given with -t or --type 'login' is assumed.\n"

#define CHANGE_MESSAGE "Usage: cman change SECRET_NAME [OPTIONS]\n\n"                                                                                                 \
		       "cman change is used to alter details of a particular secret.\n\n"                                                                             \
		       "OPTIONS:\n"                                                                                                                                   \
		       "  -t, --type [login | api]      \t\tSpecifies what type the secret is. 'login' for login credentials and 'api' for api keys.\n"               \
		       "  -f, --field FIELD		\t\tSpecifies which field or attribute of the secret to change. See Below.\n"                                            \
		       "  -h, --help			\t\tShow this help message.\n"                                                                                                 \
		       "  --no-auto			\t\tIf this option is given and FIELD is 'pass' then the password is not automatically generated rather asked from the user.\n" \
		       "  --master			\t\tChange the master password used to encrypt and decrypt the storage. No other option should be supplied.\n\n"                                                     \
		       "FIELD OPTIONS:\n"                                                                                                                             \
		       "1.login\n"                                                                                                                                    \
		       "    uname:   The Username\n"                                                                                                                  \
		       "    pass:    The Password\n"                                                                                                                  \
		       "    accname: The name used to reference the credentials\n"                                                                                    \
		       "2.api\n"                                                                                                                                      \
		       "    uname:    The username associated with the api key\n"                                                                                     \
		       "    apiname:  The name used to reference the api key\n"                                                                                       \
		       "    service:  The service the api key is for.\n"                                                                                              \
		       "    key:      The actual key.\n"                                                                                                              \
		       "\nNote: If no type is given with -t or --type 'login' is assumed.\n"

#define GET_MESSAGE "Usage: cman get SECRET_NAME [OPTIONS]\n\n"                                                                                                         \
		    "cman get is used to retrieve details about a particular secret\n\n"                                                                                \
		    "OPTIONS:\n"                                                                                                                                        \
		    "  -t, --type [login | api]      \tSpecifies what type the secret is. 'login' for login credentials and 'api' for api keys.\n"                      \
		    "  -f, --field FIELD	     \t\tSpecifies which field or attribute of the secret to get. See Below. If no field is given all details are returned.\n" \
		    "  -h, --help		     \t\tShow this help message.\n\n"                                                                                                \
		    "FIELD OPTIONS:\n"                                                                                                                                  \
		    "1.login\n"                                                                                                                                         \
		    "    uname:   The Username\n"                                                                                                                       \
		    "    pass:    The Password\n"                                                                                                                       \
		    "2.api\n"                                                                                                                                           \
		    "    uname:    The username associated with the api key\n"                                                                                          \
		    "    service:  The service the api key is for.\n"                                                                                                   \
		    "    key:      The actual key.\n"                                                                                                                   \
		    "\nNote: If no type is given with -t or --type 'login' is assumed.\n"

#define DELETE_MESSAGE "Usage: cman delete SECRET_NAME [OPTIONS]\n\n"                                                                                   \
		       "cman delete is used to delete the given secret permanently from storage. Use with care.\n\n"                                    \
		       "OPTIONS:\n"                                                                                                                     \
		       "  -t, --type [login | api]      \t\tSpecifies what type the secret is. 'login' for login credentials and 'api' for api keys.\n" \
		       "  -h, --help		        \t\tShow this help message.\n\n"                                                                          \
		       "Note: If no type is given with -t or --type 'login' is assumed.\n"

#define LS_MESSAGE "Usage: cman ls [OPTIONS]\n\n"                                                                                                  \
		   "cman ls is used to list all of the stored secrets of a particular type.\n\n"                                                   \
		   "OPTIONS:\n"                                                                                                                    \
		   "  -t, --type [login | api]      	\tSpecifies what type the secret is. 'login' for login credentials and 'api' for api keys.\n" \
		   "  -h, --help		        \t\tShow this help message.\n\n"                                                                         \
		   "Note: If no type is given with -t or --type 'login' is assumed.\n"

#endif

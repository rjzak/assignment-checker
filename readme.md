# Assignment Checker

The purpose is to scan a directory of assignments, or directory of many assignments, to get an idea if cheating may be occurring.

Currently, it uses [SSDeep](https://ssdeep-project.github.io/ssdeep/index.html) to check the similarity of text documents, and reports similarities greater than zero. Similarities greater than 95 (of 100) are shown in red.

## Limitations:
* SSDeep is best for text documents. [LZJD](https://github.com/EdwardRaff/jLZJD) is a better fit for binary documents, and this feature is not implemented yet.
* This application does **not** provide a definitive answer as to whether or not cheating has occurred. It's meant to be a quick mechanism to show where it's likely cheating **hasn't** occurred, so minimal time is spent checking several assignments from several students. You must still manually check individual submissions for evidence of cheating.

## Features:
* At the end of the command, provide the file extention if you wish to only check one or more extensions (instead of all files). Do not include an asterisk, as this is not a regular expression matcher.
* For assignments with multiple files, the files are concatenated then hashed. This can be adjusted by only selecting the desired file type.


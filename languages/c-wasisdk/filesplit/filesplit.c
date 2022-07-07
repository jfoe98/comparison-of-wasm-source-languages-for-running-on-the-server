#include <stdio.h>
#include <stdlib.h>
#include <errno.h>

#define __IMPORT(name) \
    __attribute__((__import_module__("env"), __import_name__(#name)))

void startup() __IMPORT(startup);
void finish() __IMPORT(finish);

int main(int argc, char *argv[]) {
    startup();

    int n = atoi(argv[2]);
 
    FILE *inputStream;
	char *line = NULL;
	size_t len = 0;
	ssize_t read;

    char inputFileName [50];
    sprintf(inputFileName, "./numbers_%d.txt", n);

	inputStream = fopen(inputFileName, "r");

	if (inputStream == NULL) {
        printf("%s", "Input stream is null");
        exit(EXIT_FAILURE);
    }
 
	while ((read = getline(&line, &len, inputStream)) != -1) {
        char outputFileName [50];
        sprintf(outputFileName, "./%d.txt", atoi(line) % 10);

        FILE *outputFile=fopen(outputFileName, "a+");
        fprintf(outputFile, "%s", line);
        fflush(outputFile);
        fclose(outputFile);
	}
 
	free(line);
	fclose(inputStream);

    finish();
}
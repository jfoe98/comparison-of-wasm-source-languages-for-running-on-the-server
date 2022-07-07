#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <emscripten/emscripten.h>

extern void startup();
extern void finish();

EMSCRIPTEN_KEEPALIVE void filesplit(int n) {
    startup();
 
    FILE *inputStream;
	char *line = NULL;
	size_t len = 0;
	ssize_t read;

    char inputFileName [50];
    sprintf(inputFileName, "./numbers_%d.txt", n);

    printf("Input file name is: %s", inputFileName);
	inputStream = fopen(inputFileName, "r");

	if (inputStream == NULL) {
        printf("%s", "Input stream is null");
        exit(EXIT_FAILURE);
    }
 
	while ((read = getline(&line, &len, inputStream)) != -1) {
		printf("Retrieved line of length %u :\n", read);
		printf("%s", line);

        char outputFileName [50];
        sprintf(outputFileName, "./%d.txt", atoi(line) % 10);

        FILE *outputFile=fopen(outputFileName, "a+");
        fprintf(outputFile, "%s\n", line);
        fflush(outputFile);
        fclose(outputFile);
	}
 
	free(line);
	fclose(inputStream);

    finish();
}
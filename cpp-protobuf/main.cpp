#include <iostream>
#include "protobuf/arithmetic.pb.h"

#include <unistd.h>

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

void sendSumTask(int a, int b, int data_socket) {
	// Setup task
	SumTask *sumTask = new SumTask();
	sumTask->set_val1(a);
	sumTask->set_val2(b);
	ArithmeticTask wrapper;
	wrapper.set_allocated_sum_task(sumTask);
	size_t size = wrapper.ByteSize();
	char c[size];
	wrapper.SerializeToArray(&c, size);
	ssize_t written = write(data_socket, c, size);

	if (written != size) {
		std::cout << "Something broke when sending" << std::endl;
		std::cout << "Written: " << written << std::endl;
		std::cout << "Size: " << size << std::endl;
		std::cout << "errno: " << errno << std::endl;
		exit(EXIT_FAILURE);
	}
}

int main() {
	// Verify that the version of the library that we linked against is
	// compatible with the version of the headers we compiled against.
	GOOGLE_PROTOBUF_VERIFY_VERSION;
	// Create socket
	int data_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
	if (data_socket == -1) {
		perror("socket");
		exit(EXIT_FAILURE);
	}

	// Connect socket to socket address
	struct sockaddr_un addr;
	addr.sun_family = AF_UNIX;
	strncpy(addr.sun_path, "/tmp/rust-ipc.sock", sizeof(addr.sun_path) - 1);
	int ret = connect(data_socket, (const struct sockaddr *) &addr, sizeof(struct sockaddr_un));
	if (ret == -1) {
		fprintf(stderr, "The server is down.\n");
		exit(EXIT_FAILURE);
	}
	for (int i=0; i < 200; i++) sendSumTask(1, i+1, data_socket);

	close(data_socket);
	return 0;
}

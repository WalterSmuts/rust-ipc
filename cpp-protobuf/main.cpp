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

#define BUFFER_SIZE 1024

void sendSumTask(int a, int b, int data_socket) {
	// Setup task
	SumTask sumTask;
	ArithmeticTask wrapper;
	ArithmeticResponse response;
	char buffer [BUFFER_SIZE];
	sumTask.set_val1(a);
	sumTask.set_val2(b);
	wrapper.set_allocated_sum_task(&sumTask);

	std::cout << "Sending question:\n" << wrapper.DebugString();
	bool worked = wrapper.SerializeToFileDescriptor(data_socket);
	if (!worked) {
		std::cout << "Something broke when sending" << std::endl;
		std::cout << "errno: " << errno << std::endl;
		exit(EXIT_FAILURE);
	}


	int size = read(data_socket, &buffer, BUFFER_SIZE);
	worked = response.ParseFromArray(&buffer, size);
	if (!worked) {
	      std::cout << "Something broke when receiving" << std::endl;
	      std::cout << "errno: " << errno << std::endl;
	      exit(EXIT_FAILURE);
	}
	std::cout << "Received response: \n" << response.DebugString() << std::endl;
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

	/* Bind socket to socket name. */
	struct sockaddr_un name;
	name.sun_family = AF_UNIX;
	strncpy(name.sun_path, "/tmp/rust-ipc.client", sizeof(name.sun_path) - 1);
	int ret = bind(data_socket, (const struct sockaddr *) &name,
	           sizeof(struct sockaddr_un));
	if (ret == -1) {
	    perror("bind");
	    exit(EXIT_FAILURE);
	}

	// Connect socket to socket address (server read; client write)
	struct sockaddr_un addr;
	addr.sun_family = AF_UNIX;
	strncpy(addr.sun_path, "/tmp/rust-ipc.server", sizeof(addr.sun_path) - 1);
	ret = -1;
	for (;;) {
		ret = connect(data_socket, (const struct sockaddr *) &addr, sizeof(struct sockaddr_un));
		if (!ret) {
			break;
		}
		fprintf(stderr, "The server is down.\n");
	}

	for (int i=0; i < 10000; i++) sendSumTask(1, i+1, data_socket);

	close(data_socket);
	unlink("/tmp/rust-ipc.client");
	return 0;
}

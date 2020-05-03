package rpc

import (
	"bytes"
	"errors"
	"io"
	"net/http"
)

// rpcRequest represents a RPC request.
// rpcRequest implements the io.ReadWriteCloser interface.
type rpcRequest struct {
	r    io.Reader     // holds the JSON formated RPC request
	rw   io.ReadWriter // holds the JSON formated RPC response
	done chan bool     // signals then end of the RPC request
}

// NewRPCRequest returns a new rpcRequest.
func newRPCRequest(r io.Reader) *rpcRequest {
	var buf bytes.Buffer
	done := make(chan bool)
	return &rpcRequest{r, &buf, done}
}

// Read implements the io.ReadWriteCloser Read method.
func (r *rpcRequest) Read(p []byte) (n int, err error) {
	return r.r.Read(p)
}

// Write implements the io.ReadWriteCloser Write method.
func (r *rpcRequest) Write(p []byte) (n int, err error) {
	return r.rw.Write(p)
}

// Close implements the io.ReadWriteCloser Close method.
func (r *rpcRequest) Close() error {
	r.done <- true
	return nil
}

// Call invokes the RPC request, waits for it to complete, and returns the results.
func (r *rpcRequest) Call() io.Reader {
	go ServeJSONConn(r)
	<-r.done
	return r.rw
}

// Serve rpc methods with port
func ServeHTTP(methods interface{}, port string) error {
	err := Register(methods)
	if err != nil {
		return errors.New("Generate RPC methods failed")
	}

	// Gather all requests to `/`
	go http.HandleFunc("/", func(w http.ResponseWriter, req *http.Request) {
		defer req.Body.Close()
		w.Header().Set("Content-Type", "application/json")

		res := newRPCRequest(req.Body).Call()
		_, err = io.Copy(w, res)
	})

	// Check the err in callback
	if err != nil {
		return errors.New("Generate response failed")
	}

	// Return the err to caller
	return http.ListenAndServe(port, nil)
}
